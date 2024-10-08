use crate::{algorithms::*, *};
use core::ptr;

/// # Transform
///
/// Transform the image using the supplied 3×3 transformation matrix.
///
/// ## For C++ Developers
///
/// The C++ version of this is called `warp()`.
impl ImageBuffer {
    #[named]
    pub fn replace_by_transform<'a>(
        &mut self,
        source: &ImageBuffer,
        matrix_2d: impl Into<Matrix3Ref<'a, f32>>,
    ) -> Result<&mut Self> {
        let is_ok = self.transform_ffi(source, matrix_2d, &TransformOptions::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_by_transform_with<'a>(
        &mut self,
        source: &ImageBuffer,
        matrix_2d: impl Into<Matrix3Ref<'a, f32>>,
        transform_options: &TransformOptions,
    ) -> Result<&mut Self> {
        let is_ok = self.transform_ffi(source, matrix_2d, transform_options);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn transform<'a>(
        &mut self,
        matrix_2d: impl Into<Matrix3Ref<'a, f32>>,
    ) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.transform_ffi(self, matrix_2d, &TransformOptions::default());
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn transform_with<'a>(
        &mut self,
        matrix_2d: impl Into<Matrix3Ref<'a, f32>>,
        transform_options: &TransformOptions,
    ) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.transform_ffi(self, matrix_2d, transform_options);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

#[derive(Clone, Default)]
pub struct TransformOptions {
    /// The reconstruction filter is used to weight the source image pixels
    /// falling underneath it for each destination pixel.
    ///
    /// If this is `None` a reasonable high-quality default filter is chosen.
    ///
    /// The filter's width is expressed in pixel units of the destination
    /// image. If the width is zero the default width of the resp. filter will
    /// be used.
    pub filter: Option<Filter2D>,
    /// If this is true the image will (re-)sized to be an large enough to
    /// hold the transformed image, or will have the same region of
    /// interest as the source image otherwise.
    ///
    /// If `true` and the modified `ImageBuffer` is uninitialized or if a new
    /// ImageBuffer is created by the `replace_by_transform_with()`, variant,
    /// the buffer will be sized to be large enough to hold the transformed
    /// image.
    ///
    /// If `false` the image will have the same region_of_interest as the
    /// source image.
    ///
    /// If the `tranform_with()` variant is used and the destination image is
    /// already is initialized, its size will not be changed and this
    /// option will be ignored.
    pub recompute_region_of_interest: bool,
    /// The wrap mode controlling the value of pixel lookups that need to occur
    /// beyond the boundary of the source image.
    pub wrap_mode: WrapMode,
    /// If `true`, will enable special edge clamp behavior to reduce artifacts
    /// at the image edges.
    pub edge_clamp: bool,
    /// Only the pixels (and channels) of the resulting image that are
    /// specified by this will be copied from the transformed image.
    ///
    /// The default is to alter all the pixels in the image.
    pub region_of_interest: RegionOfInterest,
    pub thread_count: u16,
}

impl ImageBuffer {
    #[inline]
    fn transform_ffi<'a>(
        &mut self,
        source: &ImageBuffer,
        matrix_2d: impl Into<Matrix3Ref<'a, f32>>,
        transform_options: &TransformOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        let matrix_2d = matrix_2d.into();
        let matrix_2d: *const [f32; 9] = matrix_2d.0 as *const _ as _;

        unsafe {
            oiio_ImageBufAlgo_warp(
                self.as_raw_ptr_mut(),
                source.as_raw_ptr(),
                &matrix_2d as *const _ as _,
                transform_options
                    .filter
                    .map_or(ptr::null(), |f| f.as_raw_ptr()) as _,
                transform_options.recompute_region_of_interest,
                transform_options.wrap_mode.clone().into(),
                transform_options.region_of_interest.clone().into(),
                transform_options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{algorithms::*, *};

    #[test]
    fn transform() -> Result<()> {
        use camino::Utf8Path as Path;

        let mut image_buf =
            ImageBuffer::from_file(Path::new("assets/wooden_lounge_2k__F32_RGBA.exr"))?;

        use core::f32::consts::FRAC_1_SQRT_2;
        let matrix = &[
            FRAC_1_SQRT_2,
            FRAC_1_SQRT_2,
            0.0,
            -FRAC_1_SQRT_2,
            FRAC_1_SQRT_2,
            0.0,
            20.0,
            -8.284271,
            1.0,
        ];

        let matrix = &[1., 0., 0.5, 0., 1., 0., 0., 0., 1.];

        image_buf.transform_with(matrix, &TransformOptions {
            recompute_region_of_interest: true,
            ..Default::default()
        })?;

        image_buf.set_display_to_data_window();

        image_buf.write(Path::new("target/transformed.exr"))?;

        Ok(())
    }
}
