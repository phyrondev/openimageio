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

        image_buffer.color_convert(None, "sRGB".into())?;

        // Strip the alpha channel from the image and/or fill missing channels
        // with 0.
        region.set_channel(0..3);

        image::ImageBuffer::from_vec(
            region.width(),
            region.height(),
            image_buffer.pixels(&RegionOfInterest::Region(region))?,
        )
        .ok_or(anyhow!("Failed to convert image to RgbImage"))
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

        image_buffer.color_convert(None, "sRGB".into())?;

        // Strip missing channels resp. fill with 0.
        region.set_channel(0..4);


        image::ImageBuffer::from_vec(
            region.width(),
            region.height(),
            image_buffer.pixels(&RegionOfInterest::Region(region))?,
        )
        .ok_or(anyhow!("Failed to convert image to RgbaImage"))
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

        let image: image::RgbaImage = image_buf.try_into()?;

        image.save("test.png")?;

        Ok(())
    }
}
