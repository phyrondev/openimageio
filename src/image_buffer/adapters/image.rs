use crate::*;
use anyhow::{anyhow, Result};

macro_rules! try_image_buffer_from_image {
    ($rust_type:ty, $base_type:expr, $channel_count:expr) => {
        impl TryFrom<&$rust_type> for ImageBuffer {
            type Error = anyhow::Error;

            fn try_from(image_buffer: &$rust_type) -> Result<ImageBuffer> {
                let slice = image_buffer.as_raw();
                let width = image_buffer.width();
                let height = image_buffer.height();
                let channel_count = $channel_count;

                let min_size = width as usize * height as usize * channel_count as usize;

                if slice.len() < min_size {
                    return Err(anyhow!("Slice length must be at least {min_size}"));
                }

                let mut image_buffer = ImageBuffer::from_dimensions_ffi(
                    width,
                    height,
                    channel_count,
                    TypeDesc {
                        base_type: Some($base_type),
                        ..Default::default()
                    },
                    None,
                );

                image_buffer.set_pixels(slice, &Region::All)?;

                Ok(image_buffer)
            }
        }

        impl TryFrom<$rust_type> for ImageBuffer {
            type Error = anyhow::Error;

            fn try_from(image_buffer: $rust_type) -> Result<Self> {
                (&image_buffer).try_into()
            }
        }
    };
}

try_image_buffer_from_image!(image::GrayImage, BaseType::U8, 1);
try_image_buffer_from_image!(image::GrayAlphaImage, BaseType::U8, 2);
try_image_buffer_from_image!(image::RgbImage, BaseType::U8, 3);
try_image_buffer_from_image!(image::RgbaImage, BaseType::U8, 4);
try_image_buffer_from_image!(
    image::ImageBuffer<image::Luma<u16>, Vec<u16>>,
    BaseType::U16,
    1
);
try_image_buffer_from_image!(
    image::ImageBuffer<image::LumaA<u16>, Vec<u16>>,
    BaseType::U16,
    2
);
try_image_buffer_from_image!(
    image::ImageBuffer<image::Rgb<u16>, Vec<u16>>,
    BaseType::U16,
    3
);
try_image_buffer_from_image!(
    image::ImageBuffer<image::Rgba<u16>, Vec<u16>>,
    BaseType::U16,
    4
);
try_image_buffer_from_image!(image::Rgb32FImage, BaseType::F32, 3);
try_image_buffer_from_image!(image::Rgba32FImage, BaseType::F32, 4);

macro_rules! try_image_from_image_buffer {
    ($rust_type:ty, $channel_count:expr) => {
        impl TryFrom<&ImageBuffer> for $rust_type {
            type Error = anyhow::Error;

            fn try_from(image_buffer: &ImageBuffer) -> Result<Self> {
                let mut bounds = image_buffer
                    .data_window()
                    .bounds()
                    .ok_or(anyhow!("Image is empty"))?
                    .clone();

                // Strip superfluous channels from the image and/or fill missing
                // channels with 0.
                bounds.set_channel(0..$channel_count);

                image::ImageBuffer::from_vec(
                    bounds.width(),
                    bounds.height(),
                    image_buffer.pixels(&Region::Bounds(bounds))?,
                )
                .ok_or(anyhow!("Failed to convert to GrayImage"))
            }
        }

        impl TryFrom<ImageBuffer> for $rust_type {
            type Error = anyhow::Error;

            fn try_from(image_buffer: ImageBuffer) -> Result<Self> {
                (&image_buffer).try_into()
            }
        }
    };
}

try_image_from_image_buffer!(image::GrayImage, 1);
try_image_from_image_buffer!(image::GrayAlphaImage, 2);
try_image_from_image_buffer!(image::ImageBuffer<image::Luma<u16>, Vec<u16>>, 1);
try_image_from_image_buffer!(image::ImageBuffer<image::LumaA<u16>, Vec<u16>>, 2);
try_image_from_image_buffer!(image::ImageBuffer<image::Rgb<u16>, Vec<u16>>, 3);
try_image_from_image_buffer!(image::ImageBuffer<image::Rgba<u16>, Vec<u16>>, 4);
try_image_from_image_buffer!(image::Rgb32FImage, 3);
try_image_from_image_buffer!(image::Rgba32FImage, 4);

impl TryFrom<&ImageBuffer> for image::RgbImage {
    type Error = anyhow::Error;

    fn try_from(image_buffer: &ImageBuffer) -> Result<Self> {
        let mut bounds = image_buffer
            .data_window()
            .bounds()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        // Make sure we're in the expected color space.
        let image_buffer = ImageBuffer::from_color_convert(image_buffer, None, "sRGB")?;

        // Strip superfluous channels from the image and/or fill missing
        // channels with 0.
        bounds.set_channel(0..3);

        image::ImageBuffer::from_vec(
            bounds.width(),
            bounds.height(),
            image_buffer.pixels(&Region::Bounds(bounds))?,
        )
        .ok_or(anyhow!("Failed to convert to RgbImage"))
    }
}

impl TryFrom<ImageBuffer> for image::RgbImage {
    type Error = anyhow::Error;

    fn try_from(mut image_buffer: ImageBuffer) -> Result<Self> {
        let mut bounds = image_buffer
            .data_window()
            .bounds()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        // Make sure we're in the expected color space.
        image_buffer.color_convert(None, "sRGB")?;

        // Strip superfluous channels from the image and/or fill missing
        // channels with 0.
        bounds.set_channel(0..3);

        image::ImageBuffer::from_vec(
            bounds.width(),
            bounds.height(),
            image_buffer.pixels(&Region::Bounds(bounds))?,
        )
        .ok_or(anyhow!("Failed to convert to RgbImage"))
    }
}

impl TryFrom<&ImageBuffer> for image::RgbaImage {
    type Error = anyhow::Error;

    fn try_from(image_buffer: &ImageBuffer) -> Result<Self> {
        let mut bounds = image_buffer
            .data_window()
            .bounds()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        // Make sure we're in the expected color space.
        let image_buffer = ImageBuffer::from_color_convert(image_buffer, None, "sRGB")?;

        // Strip superfluous channels from the image and/or fill missing
        // channels with 0.
        bounds.set_channel(0..4);

        image::ImageBuffer::from_vec(
            bounds.width(),
            bounds.height(),
            image_buffer.pixels(&Region::Bounds(bounds))?,
        )
        .ok_or(anyhow!("Failed to convert to RgbaImage"))
    }
}

impl TryFrom<ImageBuffer> for image::RgbaImage {
    type Error = anyhow::Error;

    fn try_from(mut image_buffer: ImageBuffer) -> Result<Self> {
        let mut bounds = image_buffer
            .data_window()
            .bounds()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        // Make sure we're in the expected color space.
        image_buffer.color_convert(None, "sRGB")?;

        // Strip superfluous channels from the image and/or fill missing
        // channels with 0.
        bounds.set_channel(0..4);

        image::ImageBuffer::from_vec(
            bounds.width(),
            bounds.height(),
            image_buffer.pixels(&Region::Bounds(bounds))?,
        )
        .ok_or(anyhow!("Failed to convert to RgbaImage"))
    }
}

impl TryFrom<&ImageBuffer> for image::DynamicImage {
    type Error = anyhow::Error;

    fn try_from(image_buffer: &ImageBuffer) -> Result<Self> {
        let bounds = image_buffer
            .data_window()
            .bounds()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        if let Some(base_type) = image_buffer.type_desc().base_type {
            match base_type {
                BaseType::U8 => match bounds.channel_count() {
                    1 => Ok(image::DynamicImage::ImageLuma8(image_buffer.try_into()?)),
                    2 => Ok(image::DynamicImage::ImageLumaA8(image_buffer.try_into()?)),
                    3 => Ok(image::DynamicImage::ImageRgb8(image_buffer.try_into()?)),
                    _ => Ok(image::DynamicImage::ImageRgba8(image_buffer.try_into()?)),
                },
                BaseType::U16 => match bounds.channel_count() {
                    1 => Ok(image::DynamicImage::ImageLuma16(image_buffer.try_into()?)),
                    2 => Ok(image::DynamicImage::ImageLumaA16(image_buffer.try_into()?)),
                    3 => Ok(image::DynamicImage::ImageRgb16(image_buffer.try_into()?)),
                    _ => Ok(image::DynamicImage::ImageRgba16(image_buffer.try_into()?)),
                },
                // We promote all other types to RGB(A) `f32`.
                _ => {
                    if bounds.channel_count() < 4 {
                        Ok(image::DynamicImage::ImageRgb32F(image_buffer.try_into()?))
                    } else {
                        Ok(image::DynamicImage::ImageRgba32F(image_buffer.try_into()?))
                    }
                }
            }
        } else {
            Err(anyhow!("ImageBuffer has no BaseType"))
        }
    }
}

impl TryFrom<ImageBuffer> for image::DynamicImage {
    type Error = anyhow::Error;

    fn try_from(image_buffer: ImageBuffer) -> Result<Self> {
        (&image_buffer).try_into()
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::*;

    #[test]
    fn adapter() -> Result<()> {
        let image_buf = ImageBuffer::from_file(Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"))?;
        let image_buf_ref = &image_buf;

        // This will convert to RgbaF32.
        let _dynamic_image: image::DynamicImage = image_buf_ref.try_into()?;

        // This will convert to RgbaU8 and also to sRGB.
        let _rgba_u8_image: image::RgbaImage = image_buf_ref.try_into()?;

        //let mut writer = File::create("j0.3toD__F16_RGBA.png")?;

        //rgba_u8_image.write_to(&mut writer, image::ImageFormat::Png)?;

        Ok(())
    }
}
