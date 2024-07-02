use crate::{String as OiioString, *};
use anyhow::{anyhow, Result};
use camino::Utf8Path;
use log::error;
use std::{
    ffi::c_int, marker::PhantomData, mem::MaybeUninit, ptr, string::String,
};

#[derive(Default, Debug)]
pub enum WrapMode {
    #[default]
    Default,
    Black,
    Clamp,
    Periodic,
    Mirror,
}

static UNKNOWN_ERROR: &str = "unknown error";

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

/// Stores an entire image.
///
/// Provides an API for reading, writing, and manipulating images as a single
/// unit, without the need to worry about any details of storage or I/O.
///
/// All calls that read or write are implemented underneath in terms of
/// [`ImageCache`], [`ImageInput`], and [`ImageOutput`]. I.e. they work with all
/// of the image file formats supported by this crate.
///
/// This has a lifetime so we can tie lifetimes of optional dependencies to it.
/// E.g. an that of an `ImageCache`.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct ImageBuf<'a> {
    ptr: *mut oiio_ImageBuf_t,
    _marker: PhantomData<*mut &'a ()>,
}

impl<'a> Default for ImageBuf<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Clone for ImageBuf<'a> {
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
        file: &Utf8Path,
        sub_image: Option<u32>,
        mip_level: Option<u32>,
        image_cache: Option<&'a ImageCache>,
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
                    .unwrap_or("ImageBuf::write(): unknown error".into())))
            } else {
                Ok(())
            }
        }
    }
}

// Getters.
impl<'a> ImageBuf<'a> {
    #[inline]
    pub fn storage(&self) -> ImageBufStorage {
        let mut storage = MaybeUninit::<ImageBufStorage>::uninit();
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

#[cfg(feature = "compositing")]
impl<'a> ImageBuf<'a> {
    fn do_zero(&mut self, roi: Option<Roi>, thread_count: Option<u16>) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_zero(
                self.ptr,
                std::mem::transmute::<Roi, oiio_ROI_t>(
                    roi.unwrap_or(self.roi()),
                ),
                thread_count.unwrap_or_default() as _,
                &mut is_ok as *mut _ as *mut _,
            );

            is_ok.assume_init()
        }
    }

    pub fn zero(&mut self, roi: Option<Roi>, thread_count: Option<u16>) {
        if !self.do_zero(roi, thread_count) || self.is_error() {
            error!("{}", self.error(Some(true)).unwrap_or(UNKNOWN_ERROR.into()))
        }
    }

    pub fn try_zero(
        &mut self,
        roi: Option<Roi>,
        thread_count: Option<u16>,
    ) -> Result<()> {
        if !self.do_zero(roi, thread_count) || self.is_error() {
            Err(anyhow!(self
                .error(Some(true))
                .unwrap_or(UNKNOWN_ERROR.into())))
        } else {
            Ok(())
        }
    }

    fn do_noise(
        &mut self,
        noise_type: &str,
        a: f32,
        b: f32,
        monochromatic: Option<bool>,
        seed: Option<i32>,
        roi: Option<Roi>,
        thread_count: Option<u16>,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_noise(
                self.ptr,
                StringView::from(noise_type).as_raw_ptr_mut(),
                a,
                b,
                monochromatic.unwrap_or_default(),
                seed.unwrap_or_default(),
                std::mem::transmute::<Roi, oiio_ROI_t>(
                    roi.unwrap_or(self.roi()),
                ),
                thread_count.unwrap_or_default() as _,
                &mut is_ok as *mut _ as *mut _,
            );

            is_ok.assume_init()
        }
    }

    fn do_over(
        &mut self,
        other: &ImageBuf,
        roi: Option<Roi>,
        thread_count: Option<u16>,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_over(
                self.ptr,
                self.ptr,
                other.ptr,
                std::mem::transmute::<Roi, oiio_ROI_t>(
                    roi.unwrap_or(self.roi()),
                ),
                thread_count.unwrap_or_default() as _,
                &mut is_ok as *mut _ as *mut _,
            );

            is_ok.assume_init()
        }
    }

    pub fn over(
        &mut self,
        other: &ImageBuf,
        roi: Option<Roi>,
        thread_count: Option<u16>,
    ) {
        if !self.do_over(other, roi, thread_count) || self.is_error() {
            error!("{}", self.error(Some(true)).unwrap_or(UNKNOWN_ERROR.into()))
        }
    }

    pub fn try_over(
        &mut self,
        other: &ImageBuf,
        roi: Option<Roi>,
        thread_count: Option<u16>,
    ) -> Result<()> {
        if !self.do_over(other, roi, thread_count) || self.is_error() {
            Err(anyhow!(self
                .error(Some(true))
                .unwrap_or(UNKNOWN_ERROR.into())))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use camino::Utf8Path;

        //let image_cache = ImageCache::shared(false);

        let image_buf = ImageBuf::from_file(
            &Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"),
            None,
            None,
            None, //Some(image_cache),
            None::<ImageSpec>,
        );

        println!("Storage:       {:?}", image_buf.storage());
        println!("Channel Count: {:?}", image_buf.channel_count());
    }
}
