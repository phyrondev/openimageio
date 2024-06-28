use crate::*;
use anyhow::{anyhow, Result};
use std::{ffi::c_int, marker::PhantomData, mem::MaybeUninit, path::Path, ptr};

#[derive(Default, Debug)]
enum WrapMode {
    #[default]
    Default,
    Black,
    Clamp,
    Periodic,
    Mirror,
}

#[repr(C)]
#[derive(Debug)]
pub enum ImageBufStorage {
    /// An [`ImageBuf`] that doesn't represent any image at all (either
    /// because it is newly constructed with the default constructor,
    /// or had an error during construction).
    Uninitialized = oiio_IBStorage::oiio_IBStorage_UNINITIALIZED.0 as _,
    /// "Local storage" is allocated to hold the image pixels internal to the
    /// [`ImageBuf`]. This memory will be freed when the `ImageBuf` is
    /// destroyed.
    Local = oiio_IBStorage::oiio_IBStorage_LOCALBUFFER.0 as _,
    /// The [`ImageBuf`] 'wraps' pixel memory already allocated and owned by
    /// the calling application. The caller will continue to own that
    /// memory and be responsible for freeing it after the `ImageBuf` is
    /// destroyed.
    App = oiio_IBStorage::oiio_IBStorage_APPBUFFER.0 as _,
    /// The [`ImageBuf`] is 'backed' by an [`ImageCache`], which will
    /// automatically be used to retrieve pixels when requested, but the
    /// `ImageBuf` will not allocate separate storage for it. This brings
    /// all the advantages of the `ImageCache`, but can only be used for
    /// read-only `ImageBuf`'s that reference a stored image file.
    ImageCache = oiio_IBStorage::oiio_IBStorage_IMAGECACHE.0 as _,
}

#[derive(Debug)]
pub struct ImageBuf<'a> {
    ptr: *mut oiio_ImageBuf_t,
    // We need a lifetime as an ImageBuf can reference external memory (to the
    // ImageBuf) and me must make sure that this outlives `self`.
    _marker: PhantomData<*mut &'a ()>,
}

impl<'a> Drop for ImageBuf<'a> {
    fn drop(&mut self) {
        unsafe { oiio_ImageBuf_dtor(self.ptr) };
    }
}

impl<'a> ImageBuf<'a> {
    pub fn new() -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_ImageBuf_t>::uninit();

        Self {
            ptr: unsafe {
                oiio_ImageBuf_default(&mut ptr as *mut _ as *mut _);
                ptr.assume_init()
            },
            _marker: PhantomData,
        }
    }

    pub fn from_file(
        file: &Path,
        sub_image: Option<u32>,
        mip_level: Option<u32>,
        image_cache: Option<ImageCache>,
        image_spec: Option<impl Into<ImageSpec>>,
        //io_proxy: Option<IoProxy>
    ) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_ImageBuf_t>::uninit();

        Self {
            ptr: unsafe {
                oiio_ImageBuf_ctor_01(
                    StringView::from(file).as_raw_ptr_mut(),
                    sub_image.unwrap_or(0) as _,
                    mip_level.unwrap_or(0) as _,
                    image_cache
                        .map(|c| c.as_raw_ptr_mut())
                        .unwrap_or(ptr::null_mut()),
                    image_spec
                        .map(|s| s.into().as_raw_ptr_mut())
                        .unwrap_or(ptr::null_mut()),
                    ptr::null_mut() as _,
                    &mut ptr as *mut _ as *mut _,
                );
                ptr.assume_init()
            },
            _marker: PhantomData,
        }
    }

    pub fn reset(&mut self) {
        let mut ptr = MaybeUninit::<*mut oiio_ImageBuf_t>::uninit();

        self.ptr = unsafe {
            oiio_ImageBuf_reset_00(&mut ptr as *mut _ as *mut _);
            ptr.assume_init()
        };
    }

    pub fn storage(&self) -> ImageBufStorage {
        let mut storage = MaybeUninit::<ImageBufStorage>::uninit();
        unsafe {
            oiio_ImageBuf_storage(self.ptr, &mut storage as *mut _ as *mut _);
            storage.assume_init()
        }
    }

    pub fn channel_count(&self) -> usize {
        let mut count = std::mem::MaybeUninit::<c_int>::uninit();
        unsafe {
            oiio_ImageBuf_nchannels(self.ptr, &mut count as *mut _ as *mut _);
            count.assume_init() as _
        }
    }
}

// Getters.
impl<'a> ImageBuf<'a> {
    #[inline]
    pub fn roi(&self) -> Roi {
        let mut dst = MaybeUninit::<RegionOfInterest>::uninit();

        unsafe {
            oiio_ImageBuf_roi(self.ptr, &mut dst as *mut _ as *mut _);
            dst.assume_init()
        }
    }

    #[inline(always)]
    pub fn region_of_interest(&self) -> RegionOfInterest {
        self.roi()
    }

    #[inline]
    pub fn roi_full(&self) -> Roi {
        let mut dst = MaybeUninit::<RegionOfInterest>::uninit();

        unsafe {
            oiio_ImageBuf_roi_full(self.ptr, &mut dst as *mut _ as *mut _);
            dst.assume_init()
        }
    }

    #[inline(always)]
    pub fn region_of_interest_full(&self) -> RegionOfInterest {
        self.roi_full()
    }
}

impl<'a> ImageBuf<'a> {
    pub fn over(
        &mut self,
        other: &ImageBuf,
        roi: Option<Roi>,
        thread_count: Option<u16>,
    ) -> Result<()> {
        let mut is_error = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_over(
                self.ptr,
                self.ptr,
                other.ptr,
                unsafe {
                    std::mem::transmute::<Roi, oiio_ROI_t>(
                        roi.unwrap_or(self.roi()),
                    )
                },
                thread_count.unwrap_or_default() as _,
                &mut is_error as *mut _ as *mut _,
            );

            if is_error.assume_init() {
                Err(anyhow!(self
                    .error(None)
                    .unwrap_or("ImageBuf::over(): Unknown error".into())))
            } else {
                Ok(())
            }
        }
    }

    pub fn error(&self, clear: Option<bool>) -> Option<String> {
        let mut error = MaybeUninit::<*mut oiio_String_t>::uninit();

        if unsafe {
            oiio_ImageBuf_geterror(
                self.ptr,
                clear.unwrap_or(true),
                &mut error as *mut _ as *mut _,
            )
        } != 0
        {
            // Something went wrong.
            None
        } else {
            let error =
                OiioString::from(unsafe { error.assume_init() }).to_string();

            if error.is_empty() {
                None
            } else {
                Some(error)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use std::path::PathBuf;

        let image_cache = ImageCache::new_shared();

        let image_buf = ImageBuf::from_file(
            &PathBuf::from("assets/j0.3toD__F16_RGBA.exr"),
            None,
            None,
            Some(image_cache),
            None,
        );

        println!("Storage:      {:?}", image_buf.storage());
        println!("Channel Count: {:?}", image_buf.channel_count());
    }
}
