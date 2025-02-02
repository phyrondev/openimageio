use crate::*;
use anyhow::Result;

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
