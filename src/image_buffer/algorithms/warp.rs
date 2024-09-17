use crate::{algorithms::*, *};
use core::{mem::transmute, ptr};

/// # Warp
/// Warp the src image using the supplied 3x3 transformation matrix.
///
/// Only the pixels (and channels) of dst that are specified by the optional
/// `region_of_interest` will be copied from the warped src; the default roi is
/// to alter all the pixels in dst. If dst is uninitialized, it will be sized to
/// be an ImageBuf large enough to hold the warped image if recompute_roi is
/// true, or will have the same ROI as src if recompute_roi is false. It is an
/// error to pass both an uninitialized dst and an undefined roi.
///
/// The caller may explicitly pass a reconstruction filter, or specify one by
/// name and size, or if the name is the empty string resize() will choose a
/// reasonable high-quality default if nullptr is passed. The filter is used to
/// weight the src pixels falling underneath it for each dst pixel; the filter’s
/// size is expressed in pixel units of the dst image.
impl ImageBuffer {
    #[named]
    pub fn from_warp(
        src: &ImageBuffer,
        matrix_2d: impl Into<Matrix3F32>,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok =
            image_buffer.warp_ffi(src, matrix_2d, &WarpOptions::default());
        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_warp_with(
        src: &ImageBuffer,
        matrix_2d: impl Into<Matrix3F32>,
        options: &WarpOptions,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.warp_ffi(src, matrix_2d, options);

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn warp(
        &mut self,
        matrix_2d: impl Into<Matrix3F32>,
    ) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok =
            image_buffer.warp_ffi(self, matrix_2d, &WarpOptions::default());
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn warp_with(
        &mut self,
        matrix_2d: impl Into<Matrix3F32>,
        options: &WarpOptions,
    ) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.warp_ffi(self, matrix_2d, options);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

#[derive(Clone, Default)]
pub struct WarpOptions {
    pub filter: Option<Filter2D>,
    pub recompute_region_of_interest: bool,
    pub wrap_mode: WrapMode,
    pub edge_clamp: bool,
    pub region_of_interest: RegionOfInterest,
    pub thread_count: u16,
}

#[derive(Clone, Copy)]
pub struct Matrix3F32(glam::f32::Mat3);

impl From<glam::f32::Mat3> for Matrix3F32 {
    fn from(matrix: glam::f32::Mat3) -> Self {
        Self(matrix)
    }
}

impl From<Matrix3F32> for &[f32; 9] {
    fn from(matrix: Matrix3F32) -> Self {
        unsafe { transmute(&matrix.0) }
    }
}

struct Matrix33fHelper(*const [f32; 9]);

impl ImageBuffer {
    fn warp_ffi(
        &mut self,
        other: &ImageBuffer,
        matrix_2d: impl Into<Matrix3F32>,
        options: &WarpOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        let matrix_2d: Matrix3F32 = matrix_2d.into();
        let matrix_2d = Matrix33fHelper(&matrix_2d as *const _ as _);

        unsafe {
            oiio_ImageBufAlgo_warp(
                self.ptr,
                other.ptr,
                &matrix_2d as *const _ as _,
                options.filter.map_or(ptr::null(), |f| f.as_raw_ptr()) as _,
                options.recompute_region_of_interest,
                options.wrap_mode.clone().into(),
                options.region_of_interest.clone().into(),
                options.thread_count as _,
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
    fn warp() -> Result<()> {
        use camino::Utf8Path as Path;

        let mut image_buf = ImageBuffer::from_file(Path::new(
            "assets/wooden_lounge_2k__F32_RGBA.exr",
        ))?;

        let matrix = glam::f32::Mat3::from_cols_array(&[
            0.7071068, 0.7071068, 0.0, -0.7071068, 0.7071068, 0.0, 20.0,
            -8.284271, 1.0,
        ]);

        image_buf.warp_with(
            matrix,
            &WarpOptions {
                recompute_region_of_interest: true,
                ..Default::default()
            },
        )?;

        image_buf.write(Path::new("warped.exr"))?;

        Ok(())
    }
}
