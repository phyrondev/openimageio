use crate::{algorithms::*, *};
use anyhow::Result;
use core::{mem::MaybeUninit, ptr};

/// # Resize
///
/// Set the image over the `region`, to be a resized version of the
/// corresponding portion of `source` (mapping such that the 'full' image window
/// of each correspond to each other, regardless of resolution).
///
/// Also see the [Region of Interest](#region-of-interest) section on
/// [`ImageBuffer`].
impl ImageBuffer {
    #[named]
    pub fn replace_by_resize(
        &mut self,
        source: &ImageBuffer,
        region: &Bounds,
    ) -> Result<&mut Self> {
        let is_ok = self.resize_ffi(source, region, &ResizeOptions::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_by_resize_with(
        &mut self,
        source: &ImageBuffer,
        region: &Bounds,
        resize_options: &ResizeOptions,
    ) -> Result<&mut Self> {
        let is_ok = self.resize_ffi(source, region, resize_options);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn resize(&mut self, region: &Bounds) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.resize_ffi(self, region, &ResizeOptions::default());
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn resize_with(
        &mut self,
        region: &Bounds,
        resize_options: &ResizeOptions,
    ) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.resize_ffi(self, region, resize_options);
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
    #[inline]
    fn resize_ffi(
        &mut self,
        source: &ImageBuffer,
        region: &Bounds,
        resize_options: &ResizeOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_resize(
                self.as_raw_ptr_mut(),
                source.as_raw_ptr(),
                resize_options
                    .filter
                    .map_or(ptr::null(), |f| f.as_raw_ptr()) as _,
                region.clone().into(),
                resize_options.thread_count as _,
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
        let mut image_buffer =
            ImageBuffer::from_file(Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"))?;

        image_buffer.resize(&Bounds::new_2d(0..80, 0..80))?;

        #[cfg(feature = "image")]
        {
            image_buffer.color_convert(None, ustr("sRGB"))?;
            let image: image::DynamicImage = image_buffer.try_into()?;

            viuer::print(&image, &viuer::Config {
                width: Some(80),
                height: Some(40),
                ..Default::default()
            })?;
        }

        #[cfg(not(feature = "image"))]
        image_buffer.write(Utf8Path::new("target/resize.exr"))?;

        Ok(())
    }
}
