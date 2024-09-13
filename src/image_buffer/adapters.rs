use crate::*;
use anyhow::{anyhow, Result};

#[cfg(feature = "image")]
impl TryFrom<ImageBuffer> for image::RgbImage {
    type Error = anyhow::Error;

    fn try_from(mut image_buffer: ImageBuffer) -> Result<Self> {
        let mut region = image_buffer
            .region_of_interest()
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
            .region_of_interest()
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
        let mut region = image_buffer
            .region_of_interest()
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
            .region_of_interest()
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
                let pixels: Vec<u8> =
                    image_buffer.pixels(&RegionOfInterest::Region(region))?;

                Ok(egui::ColorImage::from_gray(dimensions, &pixels))
            }
            // RGB image.
            3 => {
                let pixels: Vec<u8> =
                    image_buffer.pixels(&RegionOfInterest::All)?;

                Ok(egui::ColorImage::from_rgb(dimensions, &pixels))
            }
            // RGBA image.
            _ => {
                // Make sure `pixels()` returns a buffer with max. 4 channels.
                region.set_channel(0..4);
                let pixels: Vec<u8> =
                    image_buffer.pixels(&RegionOfInterest::Region(region))?;

                Ok(egui::ColorImage::from_rgba_premultiplied(
                    dimensions, &pixels,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[cfg(feature = "image")]
    #[test]
    fn test_image_crate() -> Result<()> {
        let image_buf = ImageBuffer::from_file(Utf8Path::new(
            "assets/j0.3toD__F16_RGBA.exr",
        ))?;

        // This will convert to either Rgb8 or Rgba8 and apply
        // a conversion to sRGB
        let image: image::DynamicImage = image_buf.try_into()?;

        image.save("test.png")?;

        Ok(())
    }
}
