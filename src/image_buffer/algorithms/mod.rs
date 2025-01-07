//! A set of image processing functions that operate on [`ImageBuffer`]s.
//!
//! # Common Principles
//!
//! This section explains the general rules common to all ImageBufAlgo
//! functions. Only exceptions to these rules will be explained in the
//! subsequent listings of all the individual ImageBufAlgo functions.
//!
//! ## Return Values
//!
//! Most functions that produce image data come in two forms:
//!
//! 1. Overwrite an `ImageBuffer` with the result of the operation.
//!
//!    When the ImageBuffer that gets overwritten is uninitialized, it will be
//!    resized to be an `ImageBuffer` large enough to hold the result.
//!
//!    If the `ImageBuffer` is already initialized, the operation will be
//!    performed on the pixels in the destination that overlap the
//!    `Region`, leaving pixels in the destination which are outside
//!    unaltered.
//!
//!    In case of error, the result image returned can have any error conditions
//!    checked with [`is_ok()`](ImageBuffer::is_ok) and
//!    [`error()`](ImageBuffer::error).
//!
//!     Method 1: overwrite an empty buffer with a new image of the required
//!    dimensions.
//!
//!    After the operation the `dest.region()` is of size
//!    `fg.region().union(bg.region())`
//!
//!    ```ignore
//!    let mut empty_destination = ImageBuffer::new();
//!    empty_destination.replace_by_over(fg, bg)?;
//!    ```
//!
//!    Method 2: overwrite an existing buffer with the result of the operation.
//!    The difference is that `fg` _over_ `bg` will be fit inside the `Region`
//!    of `destination`.
//!
//!    ```ignore
//!    destination.replace_by_over(fg, bg)?;
//!    ```
//!
//! 2. Modify an existing `ImageBuffer`.
//!
//!    The function is called on a destination `ImageBuffer` where the results
//!    will be stored.
//!
//!    ```ignore
//!    // Note that for the `over` operation the order is reversed for as the
//!    // common case is to keep the dimensions of the background image.
//!    bg.over(&fg)?;
//!    ```
//!
//! ## Chaining
//!
//! Most functions that take `&mut self` as an argument will return this from
//! the function call wrapped in a `Result`.
//!
//! This allows to chain calls together and handle errors or break the chain if
//! one occurs.
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
//! For a small set of functions, there are only input images, and no image
//! outputs (e.g., `is_monochrome()`).
//!
//! ## Region (of Interest)
//!
//! Most functions take an optional [`Region`] parameter that restricts the
//! operation to a range in x, y, z, and channels.
//!
//! [`Region::default()`] (also known as [`Region::All`]) means no region
//! restriction -- the whole image will be copied or altered.
//!
//! For functions that modify an exisiting destination `ImageBuffer` that
//! is already initialized (i.e. allocated with a particular size and
//! data type), the operation will be performed on the pixels in the destination
//! that overlap the `Region`, leaving pixels in the destination which are
//! outside the `Region` unaltered.
//!
//! If the `Region` is set (not `None`) it determines the size of the result
//! image. If the `Region` is the default, `All`, the result image size will be
//! the union of the pixel data windows of the input images and have a data type
//! determined by the data types of the input images.
//!
//! Most functions also respect the `channel` member of the `Region`, thus
//! restricting the channel range on which the operation is performed. The
//! `Region::default()` sets up the `Region` to specify that the operation
//! should be performed on all channels of the input image(s).
//!
//! ## Constant And Per-Channel Values
//!
//! Many `ImageBuffer` methods take per-channel constant-valued arguments (for
//! example, a fill color). These parameters are passed as `[f32]` slices.
//! These are generally expected to have length equal to the number of channels.
//! But you may also pass a slice containing only a single `f32` which will be
//! used as the value for all channels. More generally, what is happening is
//! that the last value supplied is replicated for any missing channel.
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

pub mod channels;
pub use channels::*;
pub mod checker;
pub use checker::*;
pub mod color_convert;
pub use color_convert::*;
pub mod compare;
pub mod convolve;
pub mod cut;
pub mod fill;
pub mod filter;
pub use filter::*;
pub mod kernel;
pub use kernel::*;
pub mod invert;
pub mod noise;
pub mod over;
pub use noise::*;
pub mod pixel_hash;
pub mod premult;
pub mod render_text;
pub use render_text::*;
pub mod re_orient;
pub mod resize;
pub use resize::*;
pub mod rotate;
pub use rotate::*;
pub mod transform;
pub use transform::*;
pub mod warp;
pub use warp::*;
pub mod zero;

/// Generic options accepted by most [`ImageBuffer`]
/// [algorithms](module@algorithms).
#[derive(Clone, Default)]
pub struct Options {
    /// See the [Region of Interest](module@algorithms#region-of-interest)
    /// section in the [module@algorithms] module. .
    pub region: Region,
    /// See the [Multithreading](module@algorithms#multithreading) section
    /// in the [module@algorithms] module.
    pub thread_count: u16,
}
