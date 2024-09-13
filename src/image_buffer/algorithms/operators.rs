//! # Operators
//!
//! ## Image Editing
//!
//! * [Channel Shuffling](ImageBuffer#channel-shuffling)
//!
//! ## Transformations
//! * [Rotate 90/180/270](ImageBuffer#rotate90-180-270)
//! * [Flip-Flop-Transpose](ImageBuffer#flip-flop-transpose)
//! * [Rotation](ImageBuffer#rotate)
//! * [Resize](ImageBuffer#resize)
//! * [Warping (2D Transformation)](ImageBuffer#warp)
//!
//! ## Compositing
//!
//! * [Over](ImageBuffer#over)
use crate::{image_buffer::algorithms::Options, *};
use anyhow::Result;
use core::{
    mem::{transmute, MaybeUninit},
    ptr,
};
use ustr::{ustr, Ustr};

#[derive(Clone, Copy, Default)]
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

pub struct ColorConvertOptions {
    pub unpremultiply: bool,
    pub context_key: Option<Ustr>,
    pub context_value: Option<Ustr>,
    pub config: Option<ColorConfig>,
    pub region_of_interest: RegionOfInterest,
    pub thread_count: u16,
}

impl Default for ColorConvertOptions {
    fn default() -> Self {
        Self {
            unpremultiply: true,
            context_key: None,
            context_value: None,
            config: None,
            region_of_interest: RegionOfInterest::default(),
            thread_count: 0,
        }
    }
}

/// # Color Conversion
impl ImageBuffer {
    #[named]
    pub fn color_convert(
        &mut self,
        from_space: Option<Ustr>,
        to_space: Ustr,
    ) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = Self::color_convert_ffi(
            &mut image_buffer,
            self,
            from_space,
            to_space,
            &ColorConvertOptions::default(),
        );
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn color_convert_with(
        &mut self,
        from_space: Option<Ustr>,
        to_space: Ustr,
        options: &ColorConvertOptions,
    ) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = Self::color_convert_ffi(
            &mut image_buffer,
            self,
            from_space,
            to_space,
            options,
        );
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_color_convert(
        source: &ImageBuffer,
        from_space: Option<Ustr>,
        to_space: Ustr,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = Self::color_convert_ffi(
            &mut image_buffer,
            source,
            from_space,
            to_space,
            &ColorConvertOptions::default(),
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_color_convert_with(
        source: &ImageBuffer,
        from_space: Option<Ustr>,
        to_space: Ustr,
        options: &ColorConvertOptions,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = Self::color_convert_ffi(
            &mut image_buffer,
            source,
            from_space,
            to_space,
            options,
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }
}

/// # Compare
///
/// Numerically compare two images.
///
/// The difference threshold (for any individual color channel in any pixel) for
/// a 'failure' is `failure_threshold`, and for a 'warning' is
/// `warning_threshold`.
impl ImageBuffer {
    /// The comparison will be for all channels, on the union of the defined
    /// pixel windows of the two images (for either image, undefined pixels
    /// will be assumed to be black).
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
    /// If `options.region_of_interest` is not supplied, the comparison will be
    /// for all channels, on the union of the defined pixel windows of the
    /// two images (for either image, undefined pixels will be assumed to be
    /// black).
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

// Actual implementations.
impl ImageBuffer {
    fn color_convert_ffi(
        dest: &mut ImageBuffer,
        source: &ImageBuffer,
        from_space: Option<Ustr>,
        to_space: Ustr,
        options: &ColorConvertOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_colorconvert(
                dest.ptr,
                source.ptr,
                from_space.map_or(StringView::default().ptr, |s| {
                    StringView::from(s).ptr
                }),
                StringView::from(to_space).ptr,
                options.unpremultiply,
                options.context_key.map_or(StringView::default().ptr, |s| {
                    StringView::from(s).ptr
                }),
                options
                    .context_value
                    .map_or(StringView::default().ptr, |s| {
                        StringView::from(s).ptr
                    }),
                options
                    .config
                    .as_ref()
                    .map_or(ptr::null_mut(), |s| *s.read_arc()),
                options.region_of_interest.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }

    fn over_ffi(&mut self, other: &ImageBuffer, options: Options) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_over(
                self.ptr,
                self.ptr,
                other.ptr,
                options.region_of_interest.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }

    fn from_over_ffi(
        a: &ImageBuffer,
        b: &ImageBuffer,
        options: Options,
    ) -> ImageBuffer {
        let mut ptr = MaybeUninit::<*mut oiio_ImageBuf_t>::uninit();

        unsafe {
            oiio_ImageBufAlgo_from_over(
                a.ptr,
                b.ptr,
                options.region_of_interest.clone().into(),
                options.thread_count as _,
                &mut ptr as *mut _ as _,
            );

            Self {
                ptr: ptr.assume_init(),
                image_cache: {
                    let cache_a = a.cache();
                    let cache_b = b.cache();

                    // If both buffers have the same cache (or no cache), use
                    // that.
                    if cache_a == cache_b {
                        cache_a
                    } else {
                        // Otherwise, use the cache that is not `None` (if any).
                        if cache_a.is_some() {
                            cache_a
                        } else {
                            cache_b
                        }
                    }
                }, //_marker: PhantomData,
            }
        }
    }

    fn rotate_ffi(
        &mut self,
        other: &ImageBuffer,
        angle: f32,
        options: &RotateOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_rotate(
                self.ptr,
                other.ptr,
                angle,
                StringView::from(Into::<Ustr>::into(options.pixel_filter)).ptr,
                0.0,
                options.recompute_region_of_interest,
                options.region_of_interest.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }

    fn rotate_around_ffi(
        &mut self,
        other: &ImageBuffer,
        angle: f32,
        center_x: f32,
        center_y: f32,
        options: &RotateOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_rotate_around(
                self.ptr,
                other.ptr,
                angle,
                center_x,
                center_y,
                StringView::from(Into::<&str>::into(options.pixel_filter)).ptr,
                0.0,
                options.recompute_region_of_interest,
                options.region_of_interest.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }
}

#[derive(Clone, Default)]
pub struct RotateOptions {
    pub pixel_filter: PixelFilter,
    pub recompute_region_of_interest: bool,
    pub region_of_interest: RegionOfInterest,
    pub thread_count: u16,
}

/// # Rotate
///
/// Rotate the src image by the angle (in radians, with positive angles
/// clockwise). When `center_x` and `center_y` are supplied, they denote the
/// center of rotation; in their absence, the rotation will be about the center
/// of the image's *display window*.
///
/// Only the pixels (and channels) of dst that are specified by roi will be
/// copied from the rotated src; the default roi is to alter all the pixels in
/// dst. If dst is uninitialized, it will be resized to be an ImageBuf large
/// enough to hold the rotated image if recompute_roi is true, or will have the
/// same ROI as src if recompute_roi is false. It is an error to pass both an
/// uninitialized dst and an undefined roi. The filter is used to weight the src
/// pixels falling underneath it for each dst pixel. The caller may specify a
/// reconstruction filter by name and width (expressed in pixels units of the
/// dst image), or rotate() will choose a reasonable default high-quality
/// default filter ([`Lanczos3`](PixelFilter::Lanczos3)) if the empty string is
/// passed, and a reasonable filter width if filterwidth is 0. (Note that some
/// filter choices only make sense with particular width, in which case this
/// filterwidth parameter may be ignored.)
impl ImageBuffer {
    #[named]
    pub fn from_rotate(src: &ImageBuffer, angle: f32) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok =
            image_buffer.rotate_ffi(src, angle, &RotateOptions::default());
        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_rotate_with(
        src: &ImageBuffer,
        angle: f32,
        options: &RotateOptions,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.rotate_ffi(src, angle, options);

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_rotate_around(
        src: &ImageBuffer,
        angle: f32,
        center_x: f32,
        center_y: f32,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.rotate_around_ffi(
            src,
            angle,
            center_x,
            center_y,
            &RotateOptions::default(),
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_rotate_around_with(
        src: &ImageBuffer,
        angle: f32,
        center_x: f32,
        center_y: f32,
        options: &RotateOptions,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer
            .rotate_around_ffi(src, angle, center_x, center_y, options);

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn rotate(&mut self, angle: f32) -> Result<&mut Self> {
        let mut rotated = ImageBuffer::new();
        let is_ok = rotated.rotate_ffi(self, angle, &RotateOptions::default());
        *self = rotated;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn rotate_with(
        &mut self,
        angle: f32,
        options: &RotateOptions,
    ) -> Result<&mut Self> {
        let mut rotated = ImageBuffer::new();
        let is_ok = rotated.rotate_ffi(self, angle, options);
        *self = rotated;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn rotate_around(
        &mut self,
        angle: f32,
        center_x: f32,
        center_y: f32,
    ) -> Result<&mut Self> {
        let mut rotated = ImageBuffer::new();
        let is_ok = rotated.rotate_around_ffi(
            self,
            angle,
            center_x,
            center_y,
            &RotateOptions::default(),
        );
        *self = rotated;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn rotate_around_with(
        &mut self,
        angle: f32,
        center_x: f32,
        center_y: f32,
        options: &RotateOptions,
    ) -> Result<&mut Self> {
        let mut rotated = ImageBuffer::new();
        let is_ok =
            rotated.rotate_around_ffi(self, angle, center_x, center_y, options);
        *self = rotated;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

/// # Over
///
/// These implement [Porter-Duff compositing](https://en.wikipedia.org/wiki/Alpha_compositing).
impl ImageBuffer {
    #[named]
    pub fn from_over(a: &ImageBuffer, b: &ImageBuffer) -> Result<Self> {
        let image_buffer = ImageBuffer::from_over_ffi(a, b, Options::default());

        image_buffer.self_or_error(true, function_name!())
    }

    #[named]
    pub fn from_over_with(
        a: &ImageBuffer,
        b: &ImageBuffer,
        options: Options,
    ) -> Result<Self> {
        let image_buffer = ImageBuffer::from_over_ffi(a, b, options);

        image_buffer.self_or_error(true, function_name!())
    }

    #[named]
    pub fn over(&mut self, other: &ImageBuffer) -> Result<&mut Self> {
        let is_ok = self.over_ffi(other, Options::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn over_with(
        &mut self,
        other: &ImageBuffer,
        options: Options,
    ) -> Result<&mut Self> {
        let is_ok = self.over_ffi(other, options);

        self.mut_self_or_error(is_ok, function_name!())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn over() -> Result<()> {
        let mut image_buf_a = ImageBuffer::new();
        let mut image_buf_b = ImageBuffer::new();
        let image_buf_c = ImageBuffer::new();

        //println!("Over test");
        image_buf_a.over(image_buf_b.over(&image_buf_c)?)?;

        //println!("Over test done");
        Ok(())
    }

    #[test]
    fn rotate() -> Result<()> {
        use crate::algorithms::PixelFilter::BlackmanHarris;
        use camino::Utf8Path as Path;

        let mut image_buf = ImageBuffer::from_file(Path::new(
            "assets/wooden_lounge_2k__F32_RGBA.exr",
        ))?;

        image_buf.rotate_with(
            42.0 * std::f32::consts::TAU / 360.0,
            &RotateOptions {
                // Use a Blackmann-Harris filter to avoid halos easily
                // introduced when operating on HDRs with the default one,
                // Lanczos3.
                pixel_filter: BlackmanHarris,
                recompute_region_of_interest: true,
                ..Default::default()
            },
        )?;

        image_buf.write(Path::new("rotated.exr"))?;

        Ok(())
    }
}
