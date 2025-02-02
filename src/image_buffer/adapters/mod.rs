use crate::*;
use anyhow::{anyhow, Result};

#[cfg(feature = "egui")]
mod egui;

#[cfg(feature = "image")]
mod image;

#[cfg(feature = "tiny-skia")]
mod tiny_skia;

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
