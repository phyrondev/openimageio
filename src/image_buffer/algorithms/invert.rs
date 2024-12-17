use crate::{algorithms::*, *};
use anyhow::Result;
use core::mem::MaybeUninit;

/// # Invert
///
/// Compute per-pixel value inverse `1.0 - A` (which you can think of as roughly
/// meaning switching white and black).
///
/// Tips for callers:
///
/// 1. You probably want to set `region_of_interest` to restrict the operation
///    to only the color channels, and not accidentally include alpha, z, or
///    others.
///
/// 2. There may be situations where you want to
///    [`unpremultiply()`](ImageBuffer::unpremultiply) invert, then
///    [`premultiply()`](ImageBuffer::premultiply) the result, so that you are
///    computing the inverse of the unmasked color.
impl ImageBuffer {
    #[named]
    pub fn replace_by_invert(&mut self, source: &ImageBuffer) -> Result<&mut Self> {
        let is_ok = self.invert_ffi(source, &Options::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_by_invert_with(
        &mut self,
        source: &ImageBuffer,
        options: &Options,
    ) -> Result<&mut Self> {
        let is_ok = self.invert_ffi(source, options);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn invert(&mut self) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.invert_ffi(self, &Options::default());
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn invert_with(&mut self, options: &Options) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.invert_ffi(self, options);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

impl ImageBuffer {
    #[inline]
    fn invert_ffi(&mut self, source: &ImageBuffer, options: &Options) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_invert(
                self.as_raw_ptr_mut(),
                source.as_raw_ptr(),
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
    use crate::*;

    #[test]
    fn invert() -> Result<()> {
        let mut image_buffer =
            ImageBuffer::from_file(Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"))?;

        image_buffer.resize(&Bounds::new_2d(0..80, 0..80))?;
        image_buffer.invert()?;
        image_buffer.color_convert(None, "sRGB")?;

        //image_buffer.write(Utf8Path::new("resized.png"))?;

        #[cfg(feature = "image")]
        {
            let image: image::DynamicImage = image_buffer.try_into()?;

            viuer::print(
                &image,
                &viuer::Config {
                    width: Some(80),
                    height: Some(40),
                    ..Default::default()
                },
            )?;
        }

        Ok(())
    }
}
