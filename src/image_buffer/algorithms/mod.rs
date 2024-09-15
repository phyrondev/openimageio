//! A set of image processing functions that operate on [`ImageBuffer`]s.
//!
//! # Common Principles
//!
//! This section explains the general rules common to all ImageBufAlgo
//! functions. Only exceptions to these rules will be explained in the
//! subsequent listings of all the individual ImageBufAlgo functions.
//!
//! ## Return Values and Error Messages
//!
//! Most functions that produce image data come in two forms:
//!
//! 1. Return an `ImageBuffer`.
//!
//!    The return value is a new `ImageBuffer` containing the result image. In
//!    this case, an entirely new image will be created to hold the result.
//!
//!    In case of error, the result image returned can have any error conditions
//!    checked with [`is_ok()`](ImageBuffer::is_ok) and
//!    [`error()`](ImageBuffer::error).
//!
//!    ```ignore
//!    // Method 1: Return an image result
//!    let dest = ImageBuffer::from_over(fg, bg)?;
//!    ```
//!
//! 2. Modify an existing `ImageBuffer`.
//!
//!    The function is called on a destination `ImageBuffer` where the results
//!    will be stored. These variants return `Result<&mut Self>` which allows
//!    chaining calls together and checking for errors at the same time (see
//!    next section).
//!
//!    ```ignore
//!    // Method 2: Write into an existing image
//!    fg.over(&bg)?;
//!    ```
//!
//! ## Chaining
//!
//! Most functions that take &mut self as an argument will return this from
//! the function call. This allows you to chain calls together.
//!
//! ```ignore
//! // Chaining methods
//! let mut dest = ImageBuffer::new_with(100, 100, TypeDesc::FLOAT)?;
//!
//! // Compose a 42° rotated text over the `dest` buffer.
//! dest.over(
//!     &ImageBuffer::from_render_text(0, 0, "HelloWorld!")?
//!         .rotate(42.0)?
//! )?;
//! ```
//!
//! For a small minority of functions, there are only input images, and no image
//! outputs (e.g., `is_monochrome()`). In such cases, the error message should
//! be retrieved from the first input image.
//!
//! ## Region of Interest
//!
//! Most functions take an optional [`Options::region_of_interest`] parameter
//! that restricts the operation to a range in x, y, z, and channels.
//! [`RegionOfInterest::default()`] (also known as [`RegionOfInterest::All`])
//! means no region restriction -- the whole image will be copied or altered.
//!
//! For functions with that modify an exisiting destination `ImageBuffer` that
//! is already initialized (i.e. allocated with a particular size and
//! data type), the operation will be performed on the pixels in the destination
//! that overlap the `RegionOfInterest`, leaving pixels in the destination which
//! are outside the `RegionOfInterest` unaltered.
//!
//! The the [`RegionOfInterest`] (if set) determines the size of the result
//! image. If the ROI is the default All, the result image size will be the
//! union of the pixel data windows of the input images and have a data type
//! determined by the data types of the input images.
//!
//! Most functions also respect the `channel` member of the `RegionOfInterest`,
//! thus restricting the channel range on which the operation is performed. The
//! `RegionOfInterest::default()` sets up the `RegionOfInterest` to specify
//! that the operation should be performed on all channels of the input
//! image(s).
//!
//! ## Constant And Per-Channel Values
//!
//! Many ImageBuffer methods take per-channel constant-valued arguments (for
//! example, a fill color). These parameters are passed as `[f32]` slices.
//! These are generally expected to have length equal to the number of channels.
//! But you may also pass a slice containing only single `f32` which will be
//! used as the value for allchannels. More generally, what is happening is that
//! the last value supplied is replicated for any missing channel.
//!
//! Some functions have parameters of type `ImageOrConst`, which may take either
//! an `ImageBuffer` reference, or a per-channel constant, or a single constant
//! to be used for all channels.
//!
//! ## Multithreading
//!
//! All functions take an optional [`thread_count`](Options::thread_count)
//! parameter that signifies the maximum number of threads to use to parallelize
//! the operation. The default value for `thread_count` is `0`, which signifies
//! that the number of thread should be the OIIO global default set by
//! [`set_attribute()`], which itself defaults to be the detected level of
//! hardware concurrency (number of cores available).
//!
//! Generally you can ignore this parameter (or pass `0`), meaning to use *all*
//! the cores available in order to perform the computation as quickly as
//! possible. The main reason to explicitly pass a different number (generally
//! `1`) is if the application is multi-threaded at a high level, and the thread
//! calling the function just wants to continue doing the computation without
//! spawning additional threads, which might tend to crowd out the other
//! application threads.
use crate::*;
use ahash::AHashMap as HashMap;
use core::{cell::OnceCell, mem::MaybeUninit};
use float_derive::{FloatEq, FloatHash, FloatPartialEq};

pub mod generators;
pub use generators::*;

pub mod operators;
pub use operators::*;

pub mod resize;
pub use resize::*;

/// Generic options accepted by most compositing operators.
#[derive(Clone, Default)]
pub struct Options {
    /// See the [Region of Interest](#region-of-interest) section on
    /// [`ImageBuffer`].
    pub region_of_interest: RegionOfInterest,
    /// See the [Multithreading](#multithreading) section on [`ImageBuffer`].
    pub thread_count: u16,
}

// Global 2D pixel filter registry.
// FIXME: This is never freed over the lifetime of the program.
const FILTER_2D_MAP: OnceCell<HashMap<Filter2DInfo, Filter2D>> =
    OnceCell::new();

// This is only used for hashing the filter name & dimensions to lookup the
// FILTER_2D_MAP.
#[derive(FloatPartialEq, FloatEq, FloatHash)]
struct Filter2DInfo {
    name: PixelFilter,
    x_width: f32,
    y_width: f32,
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum PixelFilter {
    Gaussian,
    SharpGaussian,
    Box,
    Triangle,
    Mitchell,
    BlackmanHarris,
    Bspline,
    CatmullRom,
    #[default]
    Lanczos3,
    Cubic,
    Keys,
    Simon,
    Rifman,
    Disk,
    Binomial,
    Laplacian,
}

impl From<PixelFilter> for &str {
    fn from(pf: PixelFilter) -> Self {
        match pf {
            PixelFilter::Gaussian => "gaussian",
            PixelFilter::SharpGaussian => "sharp-gaussian",
            PixelFilter::Box => "box",
            PixelFilter::Triangle => "triangle",
            PixelFilter::Mitchell => "mitchell",
            PixelFilter::BlackmanHarris => "blackman-harris",
            PixelFilter::Bspline => "b-spline",
            PixelFilter::CatmullRom => "catmull-rom",
            PixelFilter::Lanczos3 => "lanczos3",
            PixelFilter::Cubic => "cubic",
            PixelFilter::Keys => "keys",
            PixelFilter::Simon => "simon",
            PixelFilter::Rifman => "rifman",
            PixelFilter::Disk => "disk",
            PixelFilter::Binomial => "binomial",
            PixelFilter::Laplacian => "laplacian",
            //_ => "unknown",
        }
    }
}

impl From<PixelFilter> for Ustr {
    fn from(pf: PixelFilter) -> Self {
        ustr(Into::<&str>::into(pf))
    }
}

#[derive(Clone, Copy)]
pub struct Filter2D {
    ptr: *mut oiio_Filter2D_t,
}

impl Filter2D {
    fn new_ffi(name: &str, x_width: f32, y_width: f32) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_Filter2D_t>::uninit();

        unsafe {
            oiio_Filter2D_create(
                StringView::from(name).as_raw_ptr() as _,
                x_width,
                y_width,
                &mut ptr as *mut _ as _,
            );

            Self {
                ptr: ptr.assume_init(),
            }
        }
    }

    /// Create a new 2D pixel filter.
    ///
    /// The filter is cached and reused if you call new() with the same
    /// parameters.
    pub fn new(name: PixelFilter, x_width: f32, y_width: f32) -> Self {
        let mut filter_2d_map = FILTER_2D_MAP;
        let filter_2d_map = filter_2d_map.get_mut_or_init(|| HashMap::new());

        let filter = Filter2DInfo {
            name,
            x_width,
            y_width,
        };

        if let Some(filter_2d) = filter_2d_map.get(&filter) {
            filter_2d.clone()
        } else {
            let filter_2d = Filter2D::new_ffi(name.into(), x_width, y_width);
            filter_2d_map.insert(filter, filter_2d.clone());

            filter_2d
        }
    }

    /// Clear the global cache of 2D pixel filters.
    pub fn clear_cache() {
        if let Some(filter_2d_map) = FILTER_2D_MAP.take() {
            filter_2d_map.iter().for_each(|(_, filter_2d)| {
                unsafe { oiio_Filter2D_destroy(filter_2d.ptr) };
            });
        }
    }
}

impl Filter2D {
    pub(crate) fn as_raw_ptr(&self) -> *const oiio_Filter2D_t {
        self.ptr
    }

    pub(crate) fn as_raw_ptr_mut(&mut self) -> *mut oiio_Filter2D_t {
        self.ptr
    }
}
