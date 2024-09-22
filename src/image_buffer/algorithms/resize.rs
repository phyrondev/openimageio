use crate::{algorithms::*, *};
use anyhow::Result;
use core::{mem::MaybeUninit, ptr};

/// # Resize
///
/// Set the image over the `region_of_interest`, to be a resized version of the
/// corresponding portion of src (mapping such that the 'full' image window of
/// each correspond to each other, regardless of resolution).
///
/// Also see the [Region of Interest](#region-of-interest) section on
/// [`ImageBuffer`].
impl ImageBuffer {
    #[named]
    pub fn from_resize(src: &ImageBuffer, region: &Region) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok =
            image_buffer.resize_ffi(src, region, &ResizeOptions::default());

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_resize_with(
        src: &ImageBuffer,
        region: &Region,
        options: &ResizeOptions,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.resize_ffi(src, region, options);

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn resize(&mut self, region: &Region) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok =
            image_buffer.resize_ffi(self, region, &ResizeOptions::default());
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn resize_with(
        &mut self,
        region: &Region,
        options: &ResizeOptions,
    ) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.resize_ffi(self, region, options);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

#[derive(Clone, Default)]
pub struct ResizeOptions {
    /// The pixel filter is used to weight the pixels falling underneath
    /// it for each final pixel; the filter’s size is expressed in pixel units
    /// of the the destination image.
    ///
    /// If this is `None` the `resize()` variant will choose a reasonable
    /// high-quality default ([`BlackmanHarris`](PixelFilter::BlackmanHarris)
    /// when upsizing, [`Lanczos3`](PixelFilter::Lanczos3) when
    /// downsizing).
    filter: Option<Filter2D>,
    /// See the [Multithreading](#multithreading) section on [`ImageBuffer`].
    thread_count: u16,
}

impl ImageBuffer {
    #[inline(always)]
    fn resize_ffi(
        &mut self,
        src: &ImageBuffer,
        region: &Region,
        options: &ResizeOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_resize(
                self.as_raw_ptr_mut(),
                src.as_raw_ptr(),
                options.filter.map_or(ptr::null(), |f| f.as_raw_ptr()) as _,
                region.clone().into(),
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
    fn resize() -> Result<()> {
        let mut image_buffer = ImageBuffer::from_file(Utf8Path::new(
            "assets/j0.3toD__F16_RGBA.exr",
        ))?;

        let region = Region::new_2d(0..80, 0..80);

        image_buffer.resize(&region)?;
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
