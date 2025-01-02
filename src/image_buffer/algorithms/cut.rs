use crate::*;

/// # Cut
///
/// Return the designated bounds but repositioned to the image origin and with
/// the full/display window set to exactly cover the new pixel data window.
/// (Note the difference compared to [`crop()`](ImageBuffer::crop)).
///
/// Also see the [bounds](#bounds-of-interest) section on [`ImageBuffer`].
impl ImageBuffer {
    #[named]
    pub fn replace_by_cut(&mut self, src: &ImageBuffer, bounds: &Bounds) -> Result<&mut Self> {
        let is_ok = self.cut_ffi(src, bounds, None);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_by_cut_with(
        &mut self,
        src: &ImageBuffer,
        bounds: &Bounds,
        thread_count: Option<u16>,
    ) -> Result<&mut Self> {
        let is_ok = self.cut_ffi(src, bounds, thread_count);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn cut(&mut self, bounds: &Bounds) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.cut_ffi(self, bounds, None);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn cut_with(&mut self, bounds: &Bounds, thread_count: Option<u16>) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.cut_ffi(self, bounds, thread_count);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

impl ImageBuffer {
    #[inline]
    fn cut_ffi(&mut self, src: &ImageBuffer, bounds: &Bounds, thread_count: Option<u16>) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_cut(
                self.as_raw_ptr_mut(),
                src.as_raw_ptr(),
                bounds.clone().into(),
                thread_count.unwrap_or(0) as _,
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
    fn cut() -> Result<()> {
        let mut image_buffer =
            ImageBuffer::from_file(Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"))?;

        let bounds = Bounds::new_2d(0..80, 0..80);

        image_buffer.cut(&bounds)?;
        image_buffer.color_convert(None, "sRGB")?;

        //image_buffer.write(Utf8Path::new("cutd.png"))?;

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
