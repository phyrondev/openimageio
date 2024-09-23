use crate::{algorithms::*, *};
use anyhow::Result;
use core::{mem::MaybeUninit, ptr};

/// # Cut
///
/// Return the designated region but repositioned to the image origin and with
/// the full/display window set to exactly cover the new pixel data window.
/// (Note the difference compared to [`crop()`](ImageBuffer::crop)).
///
/// Also see the [Region of Interest](#region-of-interest) section on
/// [`ImageBuffer`].
impl ImageBuffer {
    #[named]
    pub fn from_cut(src: &ImageBuffer, region: &Region) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.cut_ffi(src, region, None);

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_cut_with(
        src: &ImageBuffer,
        region: &Region,
        thread_count: Option<u16>,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.cut_ffi(src, region, thread_count);

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn cut(&mut self, region: &Region) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.cut_ffi(self, region, None);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn cut_with(
        &mut self,
        region: &Region,
        thread_count: Option<u16>,
    ) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.cut_ffi(self, region, thread_count);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

impl ImageBuffer {
    #[inline(always)]
    fn cut_ffi(
        &mut self,
        src: &ImageBuffer,
        region: &Region,
        thread_count: Option<u16>,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_cut(
                self.as_raw_ptr_mut(),
                src.as_raw_ptr(),
                region.clone().into(),
                thread_count.unwrap_or(0) as _,
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
    fn cut() -> Result<()> {
        let mut image_buffer = ImageBuffer::from_file(Utf8Path::new(
            "assets/j0.3toD__F16_RGBA.exr",
        ))?;

        let region = Region::new_2d(0..80, 0..80);

        image_buffer.cut(&region)?;
        image_buffer.color_convert(None, ustr("sRGB"))?;

        //image_buffer.write(Utf8Path::new("cutd.png"))?;

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
