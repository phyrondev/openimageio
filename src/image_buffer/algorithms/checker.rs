use crate::*;
use core::{mem::MaybeUninit, num::NonZeroU32};

/// Optional parameters for [`ImageBuffer`]'s
/// [`replace_by_checker()`](ImageBuffer::replace_by_checker) and
/// [`checker_with()`](ImageBuffer::checker_with) methods.
#[derive(Clone, Default)]
pub struct CheckerOptions {
    /// The offset of the checker pattern in the x direction.
    pub offset_x: i32,
    /// The offset of the checker pattern in the y direction.
    pub offset_y: i32,
    /// The offset of the checker pattern in the z direction.
    pub offset_z: i32,
    /// See the [Region of Interest](module@algorithms#region-of-interest)
    /// section in the [module@algorithms] module. .
    pub region: Region,
    /// See the [Multithreading](module@algorithms#multithreading) section
    /// in the [module@algorithms] module.
    pub thread_count: u16,
}

/// # Fill
///
/// Create a checkerboard pattern of size given by `Region`, with origin given
/// by the offset values, checker size given by the `width`, `height`, `depth`
/// values, and alternating between `color_1` and `color_2`.
///
/// The pattern is defined in abstract “image space” independently of the pixel
/// data_window of the destination image or the `region`.
impl ImageBuffer {
    #[named]
    pub fn from_checker(
        width: NonZeroU32,
        height: NonZeroU32,
        depth: NonZeroU32,
        color_1: &[f32],
        color_2: &[f32],
        bounds: &Bounds,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.checker_ffi(
            width,
            height,
            depth,
            color_1,
            color_2,
            &CheckerOptions {
                region: Region::Bounds(bounds.clone()),
                ..Default::default()
            },
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    /// Create a checkerboard pattern of size given by `bounds`.
    ///
    /// If `options.region` is [`Bounds`](Region::Bounds), the intersection of
    /// `bounds` and the `options.region` is used.
    #[named]
    pub fn from_checker_with(
        width: NonZeroU32,
        height: NonZeroU32,
        depth: NonZeroU32,
        color_1: &[f32],
        color_2: &[f32],
        bounds: &Bounds,
        options: &CheckerOptions,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.checker_ffi(
            width,
            height,
            depth,
            color_1,
            color_2,
            &CheckerOptions {
                region: Region::from_intersection(&Region::Bounds(bounds.clone()), &options.region),
                ..*options
            },
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn checker(
        &mut self,
        width: NonZeroU32,
        height: NonZeroU32,
        depth: NonZeroU32,
        color_1: &[f32],
        color_2: &[f32],
    ) -> Result<&mut Self> {
        let is_ok = self.checker_ffi(
            width,
            height,
            depth,
            color_1,
            color_2,
            &CheckerOptions::default(),
        );

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn checker_with(
        &mut self,
        width: NonZeroU32,
        height: NonZeroU32,
        depth: NonZeroU32,
        color_1: &[f32],
        color_2: &[f32],
        options: &CheckerOptions,
    ) -> Result<&mut Self> {
        let is_ok = self.checker_ffi(width, height, depth, color_1, color_2, options);

        self.mut_self_or_error(is_ok, function_name!())
    }
}

impl ImageBuffer {
    #[inline]
    fn checker_ffi(
        &mut self,
        width: NonZeroU32,
        height: NonZeroU32,
        depth: NonZeroU32,
        color_1: &[f32],
        color_2: &[f32],
        options: &CheckerOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_checker(
                self.ptr,
                width.get() as _,
                height.get() as _,
                depth.get() as _,
                CspanF32::new(color_1).as_raw_ptr() as _,
                CspanF32::new(color_2).as_raw_ptr() as _,
                options.offset_x,
                options.offset_y,
                options.offset_z,
                options.region.clone().into(),
                options.thread_count as _,
                &raw mut is_ok as _,
            );

            is_ok.assume_init()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn checker() -> Result<()> {
        let pink = [1.0, 0.7, 0.7, 1.0];
        let blue = [0.0, 0.1, 0.8, 1.0];

        // Create a new 640x480 RGB image, with a pink and blue checkerboard pattern.
        let image_buf = ImageBuffer::from_checker(
            NonZeroU32::new(32).unwrap(),
            NonZeroU32::new(32).unwrap(),
            NonZeroU32::new(1).unwrap(),
            &pink,
            &blue,
            &Bounds::new(0..640, 0..480, 0..1, Some(0..4)),
        )?;

        image_buf.write(Utf8Path::new("target/checker.exr"))?;

        Ok(())
    }
}
