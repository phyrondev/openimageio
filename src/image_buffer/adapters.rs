use crate::*;
use anyhow::{anyhow, Result};
use log::trace;

pub trait ImageBufferFromSlice<T> {
    fn from_slice(
        width: u32,
        height: u32,
        channels: u16,
        base_type: BaseType,
        color_space: Option<&str>,
        slice: &[T],
    ) -> Result<ImageBuffer>;
}

impl ImageBufferFromSlice<u8> for ImageBuffer {
    #[named]
    fn from_slice(
        width: u32,
        height: u32,
        channel_count: u16,
        base_type: BaseType,
        color_space: Option<&str>,
        slice: &[u8],
    ) -> Result<Self> {
        let min_size = width as usize * height as usize * channel_count as usize;

        if slice.len() < min_size {
            return Err(anyhow!("Slice length must be at least {min_size}"));
        }

        /*
        let image_spec =
            ImageSpec::new_with_dimensions(width, height, channel_count as _, base_type);

        let mut image_spec: ImageSpecInternal = image_spec.into();

        if let Some(color_space) = color_space {
            image_spec.set_color_space(color_space);
        }*/

        let mut image_buffer = ImageBuffer::from_dimensions_ffi(
            width,
            height,
            channel_count,
            TypeDesc {
                base_type: Some(base_type),
                ..Default::default()
            },
            color_space,
        );

        //let mut image_buffer = ImageBuffer::new_empty_ffi(&image_spec,
        // InitializePixels::Yes);

        trace!(
            "image_buffer initialized: {:?}",
            image_buffer.is_initialized()
        );

        let mut is_ok = std::mem::MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBuf_set_pixels_u8(
                image_buffer.as_raw_ptr_mut(),
                ALL.clone().into(),
                CspanU8::new(slice).as_raw_ptr() as *const _ as _,
                &raw mut is_ok as _,
            );

            let is_ok = is_ok.assume_init();

            image_buffer.self_or_error(is_ok, function_name!())
        }
    }
}

#[cfg(feature = "tiny-skia")]
impl TryFrom<tiny_skia::Pixmap> for ImageBuffer {
    type Error = anyhow::Error;

    fn try_from(pix_map: tiny_skia::Pixmap) -> Result<Self> {
        let image_buffer = ImageBuffer::from_slice(
            pix_map.width(),
            pix_map.height(),
            4,
            BaseType::U8,
            Some("sRGB"),
            pix_map.data(),
        )?;

        Ok(image_buffer)
    }
}

#[cfg(feature = "image")]
impl TryFrom<&ImageBuffer> for image::GrayImage {
    type Error = anyhow::Error;

    fn try_from(image_buffer: &ImageBuffer) -> Result<Self> {
        let mut bounds = image_buffer
            .data_window()
            .bounds()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        // Strip superfluous channels from the image and/or fill missing
        // channels with 0.
        bounds.set_channel(0..1);

        image::ImageBuffer::from_vec(
            bounds.width(),
            bounds.height(),
            image_buffer.pixels(&Region::Bounds(bounds))?,
        )
        .ok_or(anyhow!("Failed to convert to GrayImage"))
    }
}

#[cfg(feature = "image")]
impl TryFrom<ImageBuffer> for image::GrayImage {
    type Error = anyhow::Error;

    fn try_from(image_buffer: ImageBuffer) -> Result<Self> {
        (&image_buffer).try_into()
    }
}

#[cfg(feature = "image")]
impl TryFrom<&ImageBuffer> for image::GrayAlphaImage {
    type Error = anyhow::Error;

    fn try_from(image_buffer: &ImageBuffer) -> Result<Self> {
        let mut bounds = image_buffer
            .data_window()
            .bounds()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        // Strip superfluous channels from the image and/or fill missing
        // channels with 0.
        bounds.set_channel(0..2);

        image::ImageBuffer::from_vec(
            bounds.width(),
            bounds.height(),
            image_buffer.pixels(&Region::Bounds(bounds))?,
        )
        .ok_or(anyhow!("Failed to convert to GrayAlphaImage"))
    }
}

#[cfg(feature = "image")]
impl TryFrom<ImageBuffer> for image::GrayAlphaImage {
    type Error = anyhow::Error;

    fn try_from(image_buffer: ImageBuffer) -> Result<Self> {
        (&image_buffer).try_into()
    }
}

#[cfg(feature = "image")]
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

#[cfg(feature = "image")]
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

#[cfg(feature = "image")]
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

#[cfg(feature = "image")]
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

#[cfg(feature = "image")]
impl TryFrom<&ImageBuffer> for image::ImageBuffer<image::Luma<u16>, Vec<u16>> {
    type Error = anyhow::Error;

    fn try_from(image_buffer: &ImageBuffer) -> Result<Self> {
        let mut bounds = image_buffer
            .data_window()
            .bounds()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        // Strip superfluous channels from the image and/or fill missing
        // channels with 0.
        bounds.set_channel(0..1);

        image::ImageBuffer::from_vec(
            bounds.width(),
            bounds.height(),
            image_buffer.pixels(&Region::Bounds(bounds))?,
        )
        .ok_or(anyhow!("Failed to convert to ImageLuma16"))
    }
}

#[cfg(feature = "image")]
impl TryFrom<ImageBuffer> for image::ImageBuffer<image::Luma<u16>, Vec<u16>> {
    type Error = anyhow::Error;

    fn try_from(image_buffer: ImageBuffer) -> Result<Self> {
        (&image_buffer).try_into()
    }
}

#[cfg(feature = "image")]
impl TryFrom<&ImageBuffer> for image::ImageBuffer<image::LumaA<u16>, Vec<u16>> {
    type Error = anyhow::Error;

    fn try_from(image_buffer: &ImageBuffer) -> Result<Self> {
        let mut bounds = image_buffer
            .data_window()
            .bounds()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        // Strip superfluous channels from the image and/or fill missing
        // channels with 0.
        bounds.set_channel(0..2);

        image::ImageBuffer::from_vec(
            bounds.width(),
            bounds.height(),
            image_buffer.pixels(&Region::Bounds(bounds))?,
        )
        .ok_or(anyhow!("Failed to convert to ImageLumaA16"))
    }
}

#[cfg(feature = "image")]
impl TryFrom<ImageBuffer> for image::ImageBuffer<image::LumaA<u16>, Vec<u16>> {
    type Error = anyhow::Error;

    fn try_from(image_buffer: ImageBuffer) -> Result<Self> {
        (&image_buffer).try_into()
    }
}

#[cfg(feature = "image")]
impl TryFrom<&ImageBuffer> for image::ImageBuffer<image::Rgb<u16>, Vec<u16>> {
    type Error = anyhow::Error;

    fn try_from(image_buffer: &ImageBuffer) -> Result<Self> {
        let mut bounds = image_buffer
            .data_window()
            .bounds()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        // Strip superfluous channels from the image and/or fill missing
        // channels with 0.
        bounds.set_channel(0..3);

        image::ImageBuffer::from_vec(
            bounds.width(),
            bounds.height(),
            image_buffer.pixels(&Region::Bounds(bounds))?,
        )
        .ok_or(anyhow!("Failed to convert to ImageRgb16"))
    }
}

#[cfg(feature = "image")]
impl TryFrom<ImageBuffer> for image::ImageBuffer<image::Rgb<u16>, Vec<u16>> {
    type Error = anyhow::Error;

    fn try_from(image_buffer: ImageBuffer) -> Result<Self> {
        (&image_buffer).try_into()
    }
}

#[cfg(feature = "image")]
impl TryFrom<&ImageBuffer> for image::ImageBuffer<image::Rgba<u16>, Vec<u16>> {
    type Error = anyhow::Error;

    fn try_from(image_buffer: &ImageBuffer) -> Result<Self> {
        let mut bounds = image_buffer
            .data_window()
            .bounds()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        // Strip superfluous channels from the image and/or fill missing
        // channels with 0.
        bounds.set_channel(0..4);

        image::ImageBuffer::from_vec(
            bounds.width(),
            bounds.height(),
            image_buffer.pixels(&Region::Bounds(bounds))?,
        )
        .ok_or(anyhow!("Failed to convert to ImageRgba16"))
    }
}

#[cfg(feature = "image")]
impl TryFrom<ImageBuffer> for image::ImageBuffer<image::Rgba<u16>, Vec<u16>> {
    type Error = anyhow::Error;

    fn try_from(image_buffer: ImageBuffer) -> Result<Self> {
        (&image_buffer).try_into()
    }
}

#[cfg(feature = "image")]
impl TryFrom<&ImageBuffer> for image::Rgb32FImage {
    type Error = anyhow::Error;

    fn try_from(image_buffer: &ImageBuffer) -> Result<Self> {
        let mut bounds = image_buffer
            .data_window()
            .bounds()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        // Strip superfluous channels from the image and/or fill missing
        // channels with 0.
        bounds.set_channel(0..3);

        image::ImageBuffer::from_vec(
            bounds.width(),
            bounds.height(),
            image_buffer.pixels(&Region::Bounds(bounds))?,
        )
        .ok_or(anyhow!("Failed to convert to Rgb32FImage"))
    }
}

#[cfg(feature = "image")]
impl TryFrom<ImageBuffer> for image::Rgb32FImage {
    type Error = anyhow::Error;

    fn try_from(image_buffer: ImageBuffer) -> Result<Self> {
        (&image_buffer).try_into()
    }
}

#[cfg(feature = "image")]
impl TryFrom<&ImageBuffer> for image::Rgba32FImage {
    type Error = anyhow::Error;

    fn try_from(image_buffer: &ImageBuffer) -> Result<Self> {
        let mut bounds = image_buffer
            .data_window()
            .bounds()
            .ok_or(anyhow!("Image is empty"))?
            .clone();

        // Strip superfluous channels from the image and/or fill missing
        // channels with 0.
        bounds.set_channel(0..4);

        image::ImageBuffer::from_vec(
            bounds.width(),
            bounds.height(),
            image_buffer.pixels(&Region::Bounds(bounds))?,
        )
        .ok_or(anyhow!("Failed to convert to Rgba32FImage"))
    }
}

#[cfg(feature = "image")]
impl TryFrom<ImageBuffer> for image::Rgba32FImage {
    type Error = anyhow::Error;

    fn try_from(image_buffer: ImageBuffer) -> Result<Self> {
        (&image_buffer).try_into()
    }
}

#[cfg(feature = "image")]
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

#[cfg(feature = "image")]
impl TryFrom<ImageBuffer> for image::DynamicImage {
    type Error = anyhow::Error;

    fn try_from(image_buffer: ImageBuffer) -> Result<Self> {
        (&image_buffer).try_into()
    }
}

#[cfg(feature = "egui")]
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

#[cfg(feature = "image")]
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn adapter() -> Result<()> {
        let image_buf = ImageBuffer::from_file(Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"))?;

        // This will convert to either RgbaF32.
        let _image: image::DynamicImage = image_buf.try_into()?;

        Ok(())
    }
}
