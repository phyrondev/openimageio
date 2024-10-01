use crate::{algorithms::*, *};
use anyhow::Result;
use core::mem::MaybeUninit;

/// # Un-Premultiplication
///
/// Premultiply or un-premultiply color by alpha.
///
/// The `unpremultiply` operations returns (or copies into `self`) the pixels of
/// `source` within the `region_of_interest`, and in the process divides all
/// color `alpha` or `z`) by the `alpha` value, to 'un-premultiply' them. This
/// presumes that the image starts of as “associated alpha” a.k.a.
/// “premultipled,” and you are converting to “unassociated alpha.” For pixels
/// with `alpha == 0`, the color values are not modified.
///
/// If there is no identified alpha channel the operations are simply a copy or
/// a no-op for the variants not taking a `source`.
impl ImageBuffer {
    #[named]
    pub fn replace_by_unpremultiply(&mut self, source: &ImageBuffer) -> Result<&mut Self> {
        let is_ok = self.unpremult_ffi(source, &Options::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_by_unpremultiply_with(
        &mut self,
        source: &ImageBuffer,
        options: &Options,
    ) -> Result<&mut Self> {
        let is_ok = self.unpremult_ffi(source, options);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn unpremultiply(&mut self) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.unpremult_ffi(self, &Options::default());
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn unpremultiply_with(&mut self, options: &Options) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.unpremult_ffi(self, options);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

/// # Premultiplication
///
/// The operation returns (or copies into `self`) the pixels of `source` within
/// the `region_of_interest`, and in the process multiplies all color channels
/// (those not `alpha` or `z`) by the `alpha` value, to 'premultiply' them. This
/// presumes that the image starts of as 'unassociated alpha' a.k.a.
/// 'non-premultipled' and converts it to 'associated alpha'/'premultipled'.
impl ImageBuffer {
    #[named]
    pub fn replace_by_premultiply(&mut self, source: &ImageBuffer) -> Result<&mut Self> {
        let is_ok = self.premult_ffi(source, &Options::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_by_premultiply_with(
        &mut self,
        source: &ImageBuffer,
        options: &Options,
    ) -> Result<&mut Self> {
        let is_ok = self.premult_ffi(source, options);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn premultiply(&mut self) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.premult_ffi(self, &Options::default());
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn premultiply_with(&mut self, options: &Options) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.premult_ffi(self, options);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

/// # Re-Premultiplication
///
/// The operation is like [`premultiply()`](self::premultiply), but preserves
/// the color values of pixels whose `alpha` is `0`.
///
/// This is intended for cases where you
/// [`unpremultiply()`](self::unpremultiply), do an operation (such as color
/// transforms), then want to return to associated/premultiplied alpha. In that
/// case, you want to make sure that 'glow' pixels (those with an `alpha` of `0`
/// but RGB > `0`) are preserved for the round trip, and not crushed to black.
///
/// This use case is distinct from a simple premult that is a one-time
/// conversion from unassociated to associated alpha.
impl ImageBuffer {
    #[named]
    pub fn replace_by_repremultiply(&mut self, source: &ImageBuffer) -> Result<&mut Self> {
        let is_ok = self.repremult_ffi(source, &Options::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_by_repremultiply_with(
        &mut self,
        source: &ImageBuffer,
        options: &Options,
    ) -> Result<&mut Self> {
        let is_ok = self.repremult_ffi(source, options);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn repremultiply(&mut self) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.repremult_ffi(self, &Options::default());
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn repremultiply_with(&mut self, options: &Options) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.repremult_ffi(self, options);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

impl ImageBuffer {
    #[inline(always)]
    fn unpremult_ffi(&mut self, source: &ImageBuffer, options: &Options) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_unpremult(
                self.as_raw_ptr_mut(),
                source.as_raw_ptr(),
                options.region_of_interest.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }

    #[inline(always)]
    fn premult_ffi(&mut self, source: &ImageBuffer, options: &Options) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_premult(
                self.as_raw_ptr_mut(),
                source.as_raw_ptr(),
                options.region_of_interest.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }

    #[inline(always)]
    fn repremult_ffi(&mut self, source: &ImageBuffer, options: &Options) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_repremult(
                self.as_raw_ptr_mut(),
                source.as_raw_ptr(),
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
    use crate::*;

    #[test]
    fn premult() -> Result<()> {
        let mut image_buffer =
            ImageBuffer::from_file(Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"))?;

        let region = Region::new_2d(0..80, 0..80);

        image_buffer.unpremultiply()?;
        image_buffer.color_convert(None, ustr("sRGB"))?;

        //image_buffer.write(Utf8Path::new("resized.png"))?;

        #[cfg(feature = "image")]
        {
            let image: image::DynamicImage = image_buffer.try_into()?;

            viuer::print(&image, &viuer::Config {
                width: Some(80),
                height: Some(40),
                ..Default::default()
            })?;
        }

        Ok(())
    }
}
