use crate::{String as OiioString, *};
use anyhow::{anyhow, Result};
use camino::Utf8Path;
use std::{
    ffi::c_int, marker::PhantomData, mem::MaybeUninit, ptr, string::String,
};

#[cfg(feature = "algorithms")]
mod algorithms;

mod internal;

/// Convenience type alias for developers familiar with the OpenImageIO C++ API.
pub type ImageBuf<'a> = ImageBuffer<'a>;

/// This is a placeholder for now.
pub struct IoProxy;

#[derive(Default, Debug)]
pub enum WrapMode {
    #[default]
    Default,
    Black,
    Clamp,
    Periodic,
    Mirror,
}

#[repr(C)]
#[derive(Debug)]
pub enum ImageBufferStorage {
    /// An [`ImageBuffer`] that doesn't represent any image at all (either
    /// because it is newly constructed with the default constructor,
    /// or had an error during construction).
    Uninitialized = oiio_IBStorage::oiio_IBStorage_UNINITIALIZED.0 as _,
    /// "Local storage" is allocated to hold the image pixels internal to the
    /// [`ImageBuffer`]. This memory will be freed when the `ImageBuffer` is
    /// destroyed.
    Local = oiio_IBStorage::oiio_IBStorage_LOCALBUFFER.0 as _,
    /// The [`ImageBuffer`] 'wraps' pixel memory already allocated and owned by
    /// the calling application. The caller will continue to own that
    /// memory and be responsible for freeing it after the `ImageBuffer` is
    /// destroyed.
    App = oiio_IBStorage::oiio_IBStorage_APPBUFFER.0 as _,
    /// The [`ImageBuffer`] is 'backed' by an [`ImageCache`], which will
    /// automatically be used to retrieve pixels when requested, but the
    /// `ImageBuffer` will not allocate separate storage for it. This brings
    /// all the advantages of the `ImageCache`, but can only be used for
    /// read-only `ImageBuffer`'s that reference a stored image file.
    ImageCache = oiio_IBStorage::oiio_IBStorage_IMAGECACHE.0 as _,
}

/// Stores an entire image.
///
/// Provides an API for reading, writing, and manipulating images as a single
/// unit, without the need to worry about any details of storage or I/O.
///
/// All calls that read or write are implemented underneath in terms of
/// [`ImageCache`], [`ImageInput`], and [`ImageOutput`]. I.e. they work with all
/// of the image file formats supported by this crate.
///
/// This has an explicit lifetime so we that lifetimes of optional dependencies
/// can be tied to it. E.g. that of an `ImageCache`.
///
/// # For C++ Developers
///
/// The name was changed to not contain abbreviations. The original name,
/// [`ImageBuf`] is available behind a `type` alias.
///
/// [C++ Documentation](https://openimageio.readthedocs.io/en/latest/index.html)
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ImageBuffer<'a> {
    ptr: *mut oiio_ImageBuf_t,
    _marker: PhantomData<*mut &'a ()>,
}

impl<'a> Default for ImageBuffer<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Clone for ImageBuffer<'a> {
    fn clone(&self) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_ImageBuf_t>::uninit();
        unsafe {
            oiio_ImageBuf_copy_01(
                self.ptr,
                // Will copy format from `self` as `oiio_TypeDesc_t::default()`
                // (Rust) maps to `TypeDesc::UNKNOWN` (C++).
                oiio_TypeDesc_t::default(),
                &mut ptr as *mut _ as *mut _,
            );

            Self {
                ptr: ptr.assume_init(),
                _marker: PhantomData,
            }
        }
    }
}

impl<'a> Drop for ImageBuffer<'a> {
    fn drop(&mut self) {
        unsafe { oiio_ImageBuf_dtor(self.ptr) };
    }
}

impl<'a> ImageBuffer<'a> {
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
        file: &Utf8Path,
        sub_image: Option<u32>,
        mip_level: Option<u32>,
        image_cache: Option<&'a ImageCache>,
        image_spec: Option<impl Into<ImageSpec>>,
        _io_proxy: Option<IoProxy>,
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
                        .map(|s| s.into().as_raw_ptr())
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

        unsafe {
            oiio_ImageBuf_reset_00(&mut ptr as *mut _ as *mut _);
            self.ptr = ptr.assume_init();
        };
    }

    pub fn write(
        &self,
        file: &Utf8Path,
        type_desc: Option<TypeDesc>,
        file_format: Option<&str>,
    ) -> Result<()> {
        let mut is_ok = std::mem::MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBuf_write(
                self.ptr,
                StringView::from(file).as_raw_ptr(),
                match file_format {
                    Some(file_format) => StringView::from(file_format),
                    None => StringView::default(),
                }
                .as_raw_ptr(),
                &mut is_ok as *mut _ as *mut _,
            );

            if !is_ok.assume_init() || self.is_error() {
                Err(anyhow!(self
                    .error(Some(true))
                    .unwrap_or("ImageBuffer::write(): unknown error".into())))
            } else {
                Ok(())
            }
        }
    }
}

/// # Getters
impl<'a> ImageBuffer<'a> {
    #[inline]
    pub fn storage(&self) -> ImageBufferStorage {
        let mut storage = MaybeUninit::<ImageBufferStorage>::uninit();
        unsafe {
            oiio_ImageBuf_storage(self.ptr, &mut storage as *mut _ as *mut _);
            storage.assume_init()
        }
    }

    #[inline]
    pub fn channel_count(&self) -> usize {
        let mut count = std::mem::MaybeUninit::<c_int>::uninit();
        unsafe {
            oiio_ImageBuf_nchannels(self.ptr, &mut count as *mut _ as *mut _);
            count.assume_init() as _
        }
    }

    #[inline(always)]
    pub fn roi(&self) -> RegionOfInterest {
        self.region_of_interest()
    }

    #[inline]
    pub fn region_of_interest(&self) -> RegionOfInterest {
        let mut dst = MaybeUninit::<RegionOfInterest>::uninit();

        unsafe {
            oiio_ImageBuf_roi(self.ptr, &mut dst as *mut _ as *mut _);
            dst.assume_init()
        }
    }

    #[inline]
    pub fn roi_full(&self) -> RegionOfInterest {
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

    /*
    pub fn spec(&self) -> ImageSpec {
        let mut dst = MaybeUninit::<ImageSpec>::uninit();
    }*/

    pub fn is_error(&self) -> bool {
        let mut is_error = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBuf_has_error(
                self.ptr,
                &mut is_error as *mut _ as *mut _,
            );
            is_error.assume_init()
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
    fn load() {
        use camino::Utf8Path;

        //let image_cache = ImageCache::shared(false);

        let image_buf = ImageBuffer::from_file(
            &Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"),
            None,
            None,
            None, //Some(image_cache),
            None::<ImageSpec>,
            None,
        );

        println!("Storage:       {:?}", image_buf.storage());
        println!("Channel Count: {:?}", image_buf.channel_count());
    }
}
