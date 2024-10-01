use crate::{algorithms::*, *};
use core::mem::{MaybeUninit, transmute};

/// # Compare
///
/// Numerically compare two images.
///
/// The comparison will be for all channels, on the union of the defined
/// pixel windows of the two images (for either image, undefined pixels
/// will be assumed to be black).
///
/// The difference threshold (for any individual color channel in any pixel) for
/// a 'failure' is `failure_threshold`, and for a 'warning' is
/// `warning_threshold`.
impl ImageBuffer {
    pub fn compare(
        &self,
        other: &ImageBuffer,
        warning_threshold: f32,
        error_threshold: f32,
    ) -> CompareResult {
        self.compare_ffi(
            other,
            warning_threshold,
            error_threshold,
            Options::default(),
        )
    }

    /// If `options.region_of_interest` is supplied, pixels will be compared
    /// for the pixel and channel range that is specified.
    ///
    /// If `options.region_of_interest` is [`RegionOfInterest::All`] the
    /// comparison will be for all channels, on the union of the defined
    /// pixel windows of the two images.
    ///
    /// For either image, undefined pixels will be assumed to be black/zero.
    pub fn compare_with(
        &self,
        other: &ImageBuffer,
        warning_threshold: f32,
        error_threshold: f32,
        options: Options,
    ) -> CompareResult {
        self.compare_ffi(other, warning_threshold, error_threshold, options)
    }
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
#[repr(C)]
pub struct CompareResult {
    pub mean_error: f64,
    pub root_mean_square_error: f64,
    pub peak_signal_to_noise_ratio: f64,
    pub max_error: f64,
    pub max_x: i32,
    pub max_y: i32,
    pub max_z: i32,
    pub max_c: u32,
    pub warning_count: u64,
    pub failure_count: u64,
    pub is_error: bool,
}

impl From<oiio_CompareResults_t> for CompareResult {
    fn from(compare_result: oiio_CompareResults_t) -> Self {
        unsafe { transmute(compare_result) }
    }
}

// Actual implementations.
impl ImageBuffer {
    fn compare_ffi(
        &self,
        other: &ImageBuffer,
        warning_threshold: f32,
        error_threshold: f32,
        options: Options,
    ) -> CompareResult {
        let mut compare_result = MaybeUninit::<CompareResult>::uninit();

        unsafe {
            oiio_ImageBufAlgo_compare(
                self.ptr,
                other.ptr,
                warning_threshold,
                error_threshold,
                options.region_of_interest.clone().into(),
                options.thread_count as _,
                &mut compare_result as *mut _ as _,
            );

            compare_result.assume_init()
        }
    }
}
