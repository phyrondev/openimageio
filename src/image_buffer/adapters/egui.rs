use crate::*;
use anyhow::{anyhow, Result};

impl TryFrom<ImageBuffer> for egui::ColorImage {
    type Error = anyhow::Error;

    fn try_from(mut image_buffer: ImageBuffer) -> Result<Self> {
        let mut bounds = image_buffer
            .data_window()
            .bounds()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        let dimensions = [bounds.width() as usize, bounds.height() as _];

        let channel_count = bounds.channel_count();

        if 2 < channel_count {
            // Make sure we're in the expected color space.
            image_buffer.color_convert(None, "sRGB".into())?;
        }

        match channel_count {
            // Grayscale image.
            1 | 2 => {
                // We assume this is a grayscale image (alpha channel will be
                // dropped, if present).
                bounds.set_channel(0..1);
                let pixels: Vec<u8> = image_buffer.pixels(&Region::Bounds(bounds))?;

                Ok(egui::ColorImage::from_gray(dimensions, &pixels))
            }
            // RGB image.
            3 => {
                let pixels: Vec<u8> = image_buffer.pixels(&Region::All)?;

                Ok(egui::ColorImage::from_rgb(dimensions, &pixels))
            }
            // RGBA image.
            _ => {
                // Make sure `pixels()` returns a buffer with max. 4 channels.
                bounds.set_channel(0..4);
                let pixels: Vec<u8> = image_buffer.pixels(&Region::Bounds(bounds))?;

                Ok(egui::ColorImage::from_rgba_premultiplied(
                    dimensions, &pixels,
                ))
            }
        }
    }
}
