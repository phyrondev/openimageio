use crate::*;

/// # Re-Orient
///
/// Apply whatever series of rotations, flips, or flops are necessary to
/// transform the pixels into the configuration suggested by the `Orientation`
/// metadata of the image.
///
/// The `Orientation` metadata is then set to 1, "ordinary orientation".
impl ImageBuffer {
    #[named]
    pub fn replace_by_re_orient(&mut self, source: &ImageBuffer) -> Result<&mut Self> {
        let is_ok = self.re_orient_ffi(source, 0);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_re_orient_with(
        &mut self,
        source: &ImageBuffer,
        thread_count: u16,
    ) -> Result<&mut Self> {
        let is_ok = self.re_orient_ffi(source, thread_count);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn re_orient(&mut self) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.re_orient_ffi(self, 0);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn re_orient_with(&mut self, thread_count: u16) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.re_orient_ffi(self, thread_count);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

impl ImageBuffer {
    #[inline]
    fn re_orient_ffi(&mut self, source: &ImageBuffer, thread_count: u16) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_reorient(
                self.as_raw_ptr_mut(),
                source.as_raw_ptr(),
                thread_count as _,
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
    fn re_orient() -> Result<()> {
        let mut image_buffer =
            ImageBuffer::from_file(Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"))?;

        image_buffer.re_orient()?;

        Ok(())
    }
}
