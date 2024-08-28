use crate::*;
use anyhow::{anyhow, Result};
use std::{
    error::Error, ffi::c_int, marker::PhantomData, mem::MaybeUninit, ptr,
    string::String,
};

#[cfg(feature = "algorithms")]
mod algorithms;
#[cfg(feature = "algorithms")]
pub use algorithms::*;

mod internal;

/// Convenience type alias for developers familiar with the OpenImageIO C++ API.
pub type ImageBuf<'a> = ImageBuffer<'a>;

/// This is a placeholder for now.
//pub struct IoProxy;

#[derive(Default, Debug)]
pub enum WrapMode {
    #[default]
    Default,
    Black,
    Clamp,
    Periodic,
    Mirror,
}

#[derive(Debug)]
#[repr(C)]
pub enum ImageBufferStorage {
    /// An [`ImageBuffer`] that doesn't represent any image at all (either
    /// because it is newly constructed with the default constructor,
    /// or had an error during construction).
    Uninitialized = oiio_IBStorage::oiio_IBStorage_UNINITIALIZED.0 as _,
    /// "Local storage" is allocated to hold the image pixels internal to the
    /// [`ImageBuffer`]. This memory will be freed when the `ImageBuffer` is
    /// destroyed.
    LocalBuffer = oiio_IBStorage::oiio_IBStorage_LOCALBUFFER.0 as _,
    /// The [`ImageBuffer`] 'wraps' pixel memory already allocated and owned by
    /// the calling application. The caller will continue to own that
    /// memory and be responsible for freeing it after the `ImageBuffer` is
    /// destroyed.
    AppBuffer = oiio_IBStorage::oiio_IBStorage_APPBUFFER.0 as _,
    /// The [`ImageBuffer`] is 'backed' by an [`ImageCache`], which will
    /// automatically be used to retrieve pixels when requested, but the
    /// `ImageBuffer` will not allocate separate storage for it.
    /// This brings all the advantages of the `ImageCache`, but can only be
    /// used for read-only `ImageBuffer`s that reference a stored image file.
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
    /// Default constructor makes an empty/uninitialized `ImageBuffer`. There
    /// isn't much you can do with an uninitialized buffer until you call
    /// [`reset()`](ImageBuffer::reset). The storage type of a
    /// default-constructed `ImageBuffer` is
    /// [`ImageBufferStorage::Uninitialized`].
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

/// Optional parameters for the `from_file_with()`/`try_from_file_with()`
/// methods.
#[derive(Default)]
pub struct FromFileOptions<'a> {
    /// The subimage to read (defaults to the first subimage of the file).
    pub sub_image: u32,
    /// The miplevel to read (defaults to the highest-res miplevel of the
    /// file).
    pub mip_level: u32,
    /// Optionally, an `ImageCache` to use, if possible, rather than reading
    /// the entire image file into memory.
    pub image_cache: Option<&'a ImageCache>,
    /// Optionally, a pointer to an `ImageSpec` whose metadata contains
    /// configuration hints that set options related to the opening and reading
    /// of the file.
    pub image_spec: Option<ImageSpec>,
    // Optional pointer to an `IoProxy` to use when reading from the file.
    // The lifetime of the proxy will be tied to the given `ImageBuffer`.
    // TODO io_proxy:
    // Option<&'a IoProxy>,
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
}

pub trait FnProgress<'a>: Fn(f32) + 'a {}

impl<'a> ImageBuffer<'a> {
    #[inline(always)]
    pub fn from_file(name: &Utf8Path) -> Self {
        Self::from_file_with(name, &FromFileOptions::default())
    }

    pub fn from_file_with(
        name: &Utf8Path,
        options: &FromFileOptions<'a>,
    ) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_ImageBuf_t>::uninit();

        Self {
            ptr: unsafe {
                oiio_ImageBuf_ctor_01(
                    StringView::from(name).as_raw_ptr_mut(),
                    options.sub_image as _,
                    options.mip_level as _,
                    options
                        .image_cache
                        .map(|c| c.as_raw_ptr_mut())
                        .unwrap_or(ptr::null_mut()),
                    options
                        .image_spec
                        .as_ref()
                        .map(|s| s.as_raw_ptr())
                        .unwrap_or(ptr::null_mut()),
                    ptr::null_mut() as _,
                    &mut ptr as *mut _ as *mut _,
                );
                ptr.assume_init()
            },
            _marker: PhantomData,
        }
    }

    /// Destroy any previous contents of the `ImageBuffer` and re-initialize it
    /// to resemble a freshly constructed one using the default constructor
    /// (holding no image, with storage [`ImageBufferStorage::Uninitialized`]).
    pub fn reset(&mut self) {
        let mut ptr = MaybeUninit::<*mut oiio_ImageBuf_t>::uninit();

        unsafe {
            oiio_ImageBuf_reset_00(&mut ptr as *mut _ as *mut _);
            self.ptr = ptr.assume_init();
        };
    }

    /// Write the image to the named file, converted to the specified pixel data
    /// type dtype (TypeUnknown signifies to use the data type of the buffer),
    /// and file format (an empty fileformat means to infer the type from the
    /// filename extension). By default, it will always try to write a
    /// scanline-oriented file, unless the set_write_tiles() method has been
    /// used to override this.
    ///
    /// # Arguments
    ///
    /// * `file_name` -– The filename to write to.
    ///
    /// * dtype – Optional override of the pixel data format to use in the file
    ///   being written. The default (UNKNOWN) means to try writing the same
    ///   data format that as pixels are stored within the ImageBuf memory (or
    ///   whatever type was specified by a prior call to set_write_format()). In
    ///   either case, if the file format does not support that data type,
    ///   another will be automatically chosen that is supported by the file
    ///   type and loses as little precision as possible.
    ///
    /// fileformat – Optional override of the file format to write. The default
    /// (empty string) means to infer the file format from the extension of the
    /// filename (for example, “foo.tif” will write a TIFF file).
    ///
    /// progress_callback – If progress_callback is
    /// non-NULL, the underlying write, if expensive, may make several calls to
    /// progress_callback(progress_callback_data, portion_done) which allows you
    /// to implement some sort of progress meter.
    ///
    /// Returns [`Ok`] upon success, or an erro false if the write failed (in
    /// which case, you should be able to retrieve an error message via
    /// geterror()).
    pub fn write<'b>(
        &self,
        file: &Utf8Path,
        type_desc: Option<TypeDesc>,
        file_format: Option<&str>,
        progress_callback: impl FnProgress<'b>,
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

            if !is_ok.assume_init() || !self.is_ok() {
                Err(anyhow!(self
                    .error(true)
                    .unwrap_or("ImageBuffer::write(): unknown error".into())))
            } else {
                Ok(())
            }
        }
    }
}

/// # Getters
impl<'a> ImageBuffer<'a> {
    /// Returns `true` if the `ImageBuffer` is initialized, `false` otherwise.
    ///
    /// # For C++ Developers
    ///
    /// The C++ version of this is called `initialized()`.
    pub fn is_initialized(&self) -> bool {
        let mut is_initialized = MaybeUninit::<bool>::uninit();
        unsafe {
            oiio_ImageBuf_initialized(
                self.ptr,
                &mut is_initialized as *mut _ as *mut _,
            );
            is_initialized.assume_init()
        }
    }

    /// Returns the name of the buffer (name of the file, for an `ImageBuffer`
    /// read from disk).
    ///
    /// Returns an `None` for an `ImageBuffer` that was not constructed as a
    /// direct reference to a file.
    pub fn name(&self) -> Option<String> {
        let mut name = MaybeUninit::<StringView>::uninit();
        let name = unsafe {
            oiio_ImageBuf_name(self.ptr, &mut name as *mut _ as *mut _);
            name.assume_init()
        }
        .to_string();

        if name.is_empty() {
            None
        } else {
            Some(name)
        }
    }

    /// Return the name of the image file format of the file this ImageBuf
    /// refers to (for example "openexr").
    ///
    /// Returns an `None` for an `ImageBuffer` that was not constructed
    /// as a direct reference to a file.
    pub fn file_format_name(&self) -> Option<String> {
        let mut file_format_name = MaybeUninit::<StringView>::uninit();
        let file_format_name = unsafe {
            oiio_ImageBuf_file_format_name(
                self.ptr,
                &mut file_format_name as *mut _ as *mut _,
            );
            file_format_name.assume_init()
        }
        .to_string();

        if file_format_name.is_empty() {
            None
        } else {
            Some(file_format_name)
        }
    }

    /// Return the index of the subimage within the file that the `ImageBuffer`
    /// refers to. This will always be `0` for an `ImageBuffer` that was not
    /// constructed as a direct reference to a file, or if the file contained
    /// only one image.
    pub fn sub_image(&self) -> u32 {
        let mut sub_image = MaybeUninit::<u32>::uninit();
        unsafe {
            oiio_ImageBuf_subimage(
                self.ptr,
                &mut sub_image as *mut _ as *mut _,
            );
            sub_image.assume_init()
        }
    }

    /// Return the number of subimages in the file this `ImageBuffer` refers to,
    /// if it can be determined efficiently. This will always be `1` for an
    /// `ImageBuffer` that was not constructed as a direct reference to a file,
    /// or for an `ImageBuffer` that refers to a file type that is not capable
    /// of containing multiple subimages.
    ///
    /// Note that a return value of `0` indicates that the number of subimages
    /// cannot easily be known without reading the entire image file to discover
    /// the total. To compute this yourself, you would need check every subimage
    /// successively until you get an error.
    pub fn sub_image_count(&self) -> u32 {
        let mut sub_image_count = MaybeUninit::<u32>::uninit();
        unsafe {
            oiio_ImageBuf_nsubimages(
                self.ptr,
                &mut sub_image_count as *mut _ as *mut _,
            );
            sub_image_count.assume_init()
        }
    }

    /// Return the index of the miplevel with a file’s subimage that the
    /// `ImageBuffer` is currently holding.
    ///
    /// This will always be 0 for an `ImageBuffer` that was not constructed as a
    /// direct reference to a file, or if the subimage within that file was
    /// not mipmapped.
    pub fn mip_level(&self) -> u32 {
        let mut mip_level = MaybeUninit::<u32>::uninit();
        unsafe {
            oiio_ImageBuf_miplevel(
                self.ptr,
                &mut mip_level as *mut _ as *mut _,
            );
            mip_level.assume_init()
        }
    }

    /// Return the number of miplevels of the current subimage within the file
    /// this `ImageBuffer` refers to.
    ///
    /// This will always be `1` for an `ImageBuffer` that was not constructed as
    /// a direct reference to a file, or if this subimage within the file
    /// was not mipmapped.
    pub fn mip_level_count(&self) -> u32 {
        let mut mip_level_count = MaybeUninit::<u32>::uninit();
        unsafe {
            oiio_ImageBuf_nmiplevels(
                self.ptr,
                &mut mip_level_count as *mut _ as *mut _,
            );
            mip_level_count.assume_init()
        }
    }

    /// Returns the [`ImageBufferStorage`] used.
    #[inline]
    pub fn storage(&self) -> ImageBufferStorage {
        let mut storage = MaybeUninit::<ImageBufferStorage>::uninit();
        unsafe {
            oiio_ImageBuf_storage(self.ptr, &mut storage as *mut _ as *mut _);
            storage.assume_init()
        }
    }

    /// Returns the number of channels.
    #[inline]
    pub fn channel_count(&self) -> usize {
        let mut count = std::mem::MaybeUninit::<c_int>::uninit();
        unsafe {
            oiio_ImageBuf_nchannels(self.ptr, &mut count as *mut _ as *mut _);
            count.assume_init() as _
        }
    }

    /// Return pixel data window for this `ImageBuffer` as a
    /// [`RegionOfInterest`].
    #[inline]
    pub fn region_of_interest(&self) -> RegionOfInterest {
        let mut dst = MaybeUninit::<RegionOfInterest>::uninit();

        unsafe {
            oiio_ImageBuf_roi(self.ptr, &mut dst as *mut _ as *mut _);
            dst.assume_init()
        }
    }

    /// Alias for [`region_of_interest()`](Self::region_of_interest).
    #[inline(always)]
    pub fn roi(&self) -> RegionOfInterest {
        self.region_of_interest()
    }

    /// Return full/display window for this `ImageBuffer` as a
    /// [`RegionOfInterest`].
    #[inline]
    pub fn region_of_interest_full(&self) -> RegionOfInterest {
        let mut dst = MaybeUninit::<RegionOfInterest>::uninit();

        unsafe {
            oiio_ImageBuf_roi_full(self.ptr, &mut dst as *mut _ as *mut _);
            dst.assume_init()
        }
    }

    /// Alias for [`region_of_interest_full()`](Self::region_of_interest_full).
    #[inline(always)]
    pub fn roi_full(&self) -> RegionOfInterest {
        self.region_of_interest_full()
    }

    /*
    pub fn spec(&self) -> ImageSpec {
        let mut dst = MaybeUninit::<ImageSpec>::uninit();
    }*/

    /// Returns `true` if the `ImageBuffer` has had an error and has an error
    /// message ready to retrieve via [`error()`](self::error()).
    pub fn is_ok(&self) -> bool {
        let mut is_error = MaybeUninit::<bool>::uninit();

        !unsafe {
            oiio_ImageBuf_has_error(
                self.ptr,
                &mut is_error as *mut _ as *mut _,
            );
            is_error.assume_init()
        }
    }

    /// Return the text of all pending error messages issued against this
    /// `ImageBuffer`, and clear the pending error message unless clear is
    /// `false`.
    ///
    /// If no error message is pending, this will return `None`.
    pub fn error(&self, clear: bool) -> Option<String> {
        let mut error = MaybeUninit::<*mut oiio_String_t>::uninit();

        if unsafe {
            oiio_ImageBuf_geterror(
                self.ptr,
                clear,
                &mut error as *mut _ as *mut _,
            )
        } != 0
        {
            // Something went wrong.
            None
        } else {
            let error =
                crate::String::from(unsafe { error.assume_init() }).to_string();

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
    use anyhow::Result;

    #[test]
    fn load() -> Result<()> {
        use camino::Utf8Path;

        //let image_cache = ImageCache::shared(false);

        let image_buf = ImageBuffer::from_file(Utf8Path::new(
            "assets/j0.3toD__F16_RGBA.exr",
        ));

        println!(
            "Name:          {}",
            image_buf.name().ok_or(anyhow!("No name"))?
        );
        println!("Storage:       {:?}", image_buf.storage());
        println!("Channel Count: {:?}", image_buf.channel_count());

        Ok(())
    }
}
