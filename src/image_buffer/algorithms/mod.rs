//! # Region of Interest
//!
//! Most functions take an optional [`Options::region_of_interest`] parameter
//! that restricts the operation to a range in x, y, z, and channels. The
//! default-constructed [`RegionOfInterest`] (also known as
//! [`RegionOfInterest::all()`]) means no region restriction -- the whole image
//! will be copied or altered.
//!
//! For ImageBufAlgo functions that write into a destination ImageBuf parameter
//! and it is already initialized (i.e. allocated with a particular size and
//! data type), the operation will be performed on the pixels in the destination
//! that overlap the ROI, leaving pixels in the destination which are outside
//! the ROI unaltered.
//!
//! The the [`RegionOfInterest`] (if set) determines the size of the result
//! image. If the ROI is the default All, the result image size will be the
//! union of the pixel data windows of the input images and have a data type
//! determined by the data types of the input images.
//!
//! Most ImageBufAlgo functions also respect the `channel` member of the
//! `RegionOfInterest`, thus restricting the channel range on which the
//! operation is performed. The [`RegionOfInterest::default()`] sets up the
//! `RegionOfInterest` to specify that the operation should be performed on all
//! channels of the input image(s).
//!
//! # Constant And Per-Channel Values
//!
//! Many ImageBuffer methods take per-channel constant-valued arguments (for
//! example, a fill color). These parameters are passed as `&[f32]` slices.
//! These are generally expected to have length equal to the number of channels.
//! But you may also pass a single `f32` which will be used as the value for all
//! channels. (More generally, what is happening is that the last value supplied
//! is replicated for any missing channel.)
//!
//! Some ImageBufAlgo functions have parameters of type Image_or_Const, which
//! may take either an ImageBuf reference, or a per-channel constant, or a
//! single constant to be used for all channels.
//!
//! # Multithreading
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
//! `1`) is if the application is multithreaded at a high level, and the thread
//! calling the function just wants to continue doing the computation without
//! spawning additional threads, which might tend to crowd out the other
//! application threads.

pub mod generators;
pub mod operators;

pub struct Filter2D {
    filter_name: String,
}
