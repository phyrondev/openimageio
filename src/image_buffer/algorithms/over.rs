use crate::{algorithms::*, *};

/// # Over
///
/// These implement [Porter-Duff compositing](https://en.wikipedia.org/wiki/Alpha_compositing).
impl ImageBuffer {
    #[named]
    pub fn replace_by_over(
        &mut self,
        foreground: &ImageBuffer,
        background: &ImageBuffer,
    ) -> Result<&mut Self> {
        let is_ok = self.over_ffi(foreground, background, &Options::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_by_over_with(
        &mut self,
        foreground: &ImageBuffer,
        background: &ImageBuffer,
        options: &Options,
    ) -> Result<&mut Self> {
        let is_ok = self.over_ffi(foreground, background, options);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn over(&mut self, foreground: &ImageBuffer) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.over_ffi(foreground, self, &Options::default());
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn over_with(&mut self, foreground: &ImageBuffer, options: &Options) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.over_ffi(foreground, self, options);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

impl ImageBuffer {
    #[inline]
    fn over_ffi(
        &mut self,
        foreground: &ImageBuffer,
        background: &ImageBuffer,
        options: &Options,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_over(
                self.as_raw_ptr_mut(),
                foreground.as_raw_ptr(),
                background.as_raw_ptr(),
                options.region.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn over() -> Result<()> {
        // Load fg image. This is 1024×1024
        let mut image_buf_a =
            ImageBuffer::from_file(Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"))?;

        // Load bg image. This is 2048×1024.
        let image_buf_b =
            ImageBuffer::from_file(Utf8Path::new("assets/wooden_lounge_2k__F32_RGBA.exr"))?;

        // Compose fg over bg, replacing the data window of fg  with the result.
        // I.e. the result will be cropped at fg's original dimensions of
        // 1024×1024.
        image_buf_a.over(&image_buf_b)?;

        image_buf_a.write(Utf8Path::new("target/over.exr"))
    }
}
