use crate::{algorithms::*, *};
use core::ptr;

/// # Warp
///
/// Warp the image using `st` coordinates from a `warp` image.
///
/// Each pixel in the `warp` image is used as a normalized image-space
/// coordinate in the source image, which is then sampled at that position using
/// the given reconstruction filter to produce an output pixel.
///
/// The transform is only defined over the area of the `stbuf` image, and thus
/// the given `roi` argument will be intersected with its geometry.
///
/// > The current behavior of this transform is modeled to match Nuke's
/// **STMap** node.
///
/// ## For C++ Developers
///
/// The C++ version of this is called `st_warp()`.
impl ImageBuffer {
    #[named]
    pub fn from_warp(source: &ImageBuffer, warp: &ImageBuffer) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok =
            image_buffer.warp_ffi(source, warp, &WarpOptions::default());
        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_warp_with(
        source: &ImageBuffer,
        warp: &ImageBuffer,
        options: &WarpOptions,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.warp_ffi(source, warp, options);

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn warp(&mut self, warp: &ImageBuffer) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.warp_ffi(self, warp, &WarpOptions::default());
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn warp_with(
        &mut self,
        warp: &ImageBuffer,
        options: &WarpOptions,
    ) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.warp_ffi(self, warp, options);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

#[derive(Clone, Default)]
pub struct WarpOptions {
    pub filter: Option<Filter2D>,
    channel_s: Option<u32>,
    channel_t: Option<u32>,
    flip: bool,
    flop: bool,
    pub region_of_interest: RegionOfInterest,
    pub thread_count: u16,
}

impl ImageBuffer {
    fn warp_ffi(
        &mut self,
        other: &ImageBuffer,
        warp: &ImageBuffer,
        options: &WarpOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_st_warp(
                self.ptr,
                other.ptr,
                warp.ptr,
                options.filter.map_or(ptr::null(), |f| f.as_raw_ptr()) as _,
                options.channel_s.unwrap_or(0).try_into().unwrap_or(0),
                options.channel_t.unwrap_or(1).try_into().unwrap_or(1),
                options.flip as _,
                options.flop as _,
                options.region_of_interest.clone().into(),
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
    fn warp() -> Result<()> {
        use camino::Utf8Path as Path;

        let mut image_buf = ImageBuffer::from_file(Path::new(
            "assets/wooden_lounge_2k__F32_RGBA.exr",
        ))?;

        let warp =
            ImageBuffer::from_file(Path::new("assets/warp__U8_RGB.png"))?;

        // Resize the source image to match the warp image.
        image_buf.resize(&warp.region_of_interest_full().region().unwrap())?;

        image_buf.warp(&warp)?;

        image_buf.write(Path::new("warped.exr"))?;

        Ok(())
    }
}
