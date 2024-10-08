use crate::*;
use anyhow::{Result, anyhow};

trait ImageBufferFromSlice<T> {
    fn from_slice(
        width: u32,
        height: u32,
        type_description: &TypeDescription,
        color_space: Option<&str>,
        slice: &[T],
    ) -> Result<ImageBuffer>;
}

impl ImageBufferFromSlice<u8> for ImageBuffer {
    #[named]
    fn from_slice(
        width: u32,
        height: u32,
        type_description: &TypeDescription,
        color_space: Option<&str>,
        slice: &[u8],
    ) -> Result<Self> {
        let min_size = width as usize * height as usize * type_description.size();

        if slice.len() < min_size {
            return Err(anyhow!("Slice length must be at least {min_size}"));
        }

        let mut image_spec = ImageSpecInternal::new_with(type_description);

        if let Some(color_space) = color_space {
            image_spec.set_color_space(color_space);
        }

        let mut image_buffer = ImageBuffer::new_empty_ffi(&image_spec, InitializePixels::No);

        let mut is_ok = std::mem::MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBuf_set_pixels(
                image_buffer.as_raw_ptr_mut(),
                ALL.clone().into(),
                (*type_description).into(),
                slice.as_ptr() as *const _ as _,
                0,
                0,
                0,
                &mut is_ok as *mut _ as _,
            );

            let is_ok = is_ok.assume_init();

            image_buffer.self_or_error(is_ok, function_name!())
        }
    }
}

#[cfg(feature = "tiny-skia")]
impl TryFrom<tiny_skia::Pixmap> for ImageBuffer {
    type Error = anyhow::Error;

    fn try_from(mut pix_map: tiny_skia::Pixmap) -> Result<Self> {
        let image_buffer = ImageBuffer::from_slice(
            pix_map.width(),
            pix_map.height(),
            &TypeDescription {
                base_type: Some(BaseType::U8),
                ..Default::default()
            },
            Some("sRGB"),
            pix_map.data(),
        )?;

        Ok(image_buffer)
    }
}

#[cfg(feature = "image")]
impl TryFrom<ImageBuffer> for image::RgbImage {
    type Error = anyhow::Error;

    fn try_from(mut image_buffer: ImageBuffer) -> Result<Self> {
        let mut region = image_buffer
            .data_window()
            .region()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        // Make sure we're in the expected color space.
        image_buffer.color_convert(None, "sRGB".into())?;

        // Strip the alpha channel from the image and/or fill missing channels
        // with 0.
        region.set_channel(0..3);

        image::ImageBuffer::from_vec(
            region.width(),
            region.height(),
            image_buffer.pixels(&RegionOfInterest::Region(region))?,
        )
        .ok_or(anyhow!("Failed to convert to RgbImage"))
    }
}

#[cfg(feature = "image")]
impl TryFrom<ImageBuffer> for image::RgbaImage {
    type Error = anyhow::Error;

    fn try_from(mut image_buffer: ImageBuffer) -> Result<Self> {
        let mut region = image_buffer
            .data_window()
            .region()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        // Make sure we're in the expected color space.
        image_buffer.color_convert(None, "sRGB".into())?;

        // Strip superfluous channels from the image and/or fill missing
        // channels with 0.
        region.set_channel(0..4);

        image::ImageBuffer::from_vec(
            region.width(),
            region.height(),
            image_buffer.pixels(&RegionOfInterest::Region(region))?,
        )
        .ok_or(anyhow!("Failed to convert to RgbaImage"))
    }
}

#[cfg(feature = "image")]
impl TryFrom<ImageBuffer> for image::DynamicImage {
    type Error = anyhow::Error;

    fn try_from(mut image_buffer: ImageBuffer) -> Result<Self> {
        let region = image_buffer
            .data_window()
            .region()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        // Make sure we're in the expected color space.
        image_buffer.color_convert(None, "sRGB".into())?;

        if region.channel_count() < 4 {
            Ok(image::DynamicImage::ImageRgb8(image_buffer.try_into()?))
        } else {
            Ok(image::DynamicImage::ImageRgba8(image_buffer.try_into()?))
        }
    }
}

#[cfg(feature = "egui")]
impl TryFrom<ImageBuffer> for egui::ColorImage {
    type Error = anyhow::Error;

    fn try_from(mut image_buffer: ImageBuffer) -> Result<Self> {
        let mut region = image_buffer
            .data_window()
            .region()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        let dimensions = [region.width() as usize, region.height() as _];

        let channel_count = region.channel_count();

        if 2 < channel_count {
            // Make sure we're in the expected color space.
            image_buffer.color_convert(None, "sRGB".into())?;
        }

        match channel_count {
            // Grayscale image.
            1 | 2 => {
                // We assume this is a grayscale image (alpha channel will be
                // dropped, if present).
                region.set_channel(0..1);
                let pixels: Vec<u8> = image_buffer.pixels(&RegionOfInterest::Region(region))?;

                Ok(egui::ColorImage::from_gray(dimensions, &pixels))
            }
            // RGB image.
            3 => {
                let pixels: Vec<u8> = image_buffer.pixels(&RegionOfInterest::All)?;

                Ok(egui::ColorImage::from_rgb(dimensions, &pixels))
            }
            // RGBA image.
            _ => {
                // Make sure `pixels()` returns a buffer with max. 4 channels.
                region.set_channel(0..4);
                let pixels: Vec<u8> = image_buffer.pixels(&RegionOfInterest::Region(region))?;

                Ok(egui::ColorImage::from_rgba_premultiplied(
                    dimensions, &pixels,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    
    

    #[cfg(feature = "image")]
    #[test]
    fn adapter() -> Result<()> {
        let image_buf = ImageBuffer::from_file(Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"))?;

        // This will convert to either Rgb8 or Rgba8 and apply
        // a conversion to sRGB
        let image: image::DynamicImage = image_buf.try_into()?;

        Ok(())
    }
}
