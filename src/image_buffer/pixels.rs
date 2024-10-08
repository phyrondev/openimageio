use crate::*;
use anyhow::{Result, anyhow};

/*
pub struct PixelsOptions {
    x_stride: Option<u32>,
    y_stride: Option<u32>,
    z_stride: Option<u32>,
}*/

/// Retrieve a region of pixels.
///
/// The region is the [`RegionOfInterest`] specified by the current subimage and
/// MIP-map level, and converting into the data type implied by the requested
/// [`Primitive`] type.
///
/// Returns a `Vec` of the chosen type if successful, or an error if the reading
/// of the pixels failed.
///
/// # Examples
///
/// This is probably the preferred way to get pixels into a format you need for
/// display or processing outside of this crate.
///
/// Below is an example of how to get the pixels of an `ImageBuffer` buffer into
/// a [`image::ImageBuffer`](https://docs.rs/image/latest/image/struct.ImageBuffer.html).
///
/// Note that this is readily available behind the `image` feature as
/// [`image::ImageBuffer::TryFrom<ImageBuffer>`](ImageBuffer::TryInto::<image::ImageBuffer>).
///
/// ```ignore
/// let image_buffer = openimageio::ImageBuffer::from_file(Utf8Path::new(
///     "assets/j0.3toD__F16_RGBA.exr",
/// ))?;
///
/// let mut region = image_buffer
///     .data_window()
///     .region()
///     .ok_or(anyhow!("Image is empty"))?
///     .clone();
///
/// // Make sure we're in the expected color space.
/// image_buffer.color_convert(None, "sRGB".into())?;
///
/// // Strip the alpha channel from the image and/or fill missing channels
/// // with 0.
/// region.set_channel(0..3);
///
/// let image_buffer: image::RgbImage = image::ImageBuffer::from_vec(
///     region.width(),
///     region.height(),
///     image_buffer.pixels(&RegionOfInterest::Region(region))?,
/// )?;
/// ```
///
/// # C++
///
/// The C++ version of this is called `get_pixels()`.
pub trait Pixels<T: Primitive> {
    fn pixels(&self, region_of_interest: &RegionOfInterest) -> Result<Vec<T>>;
}

macro_rules! pixels {
    ($rust_type:ty, $base_type:expr) => {
        impl Pixels<$rust_type> for ImageBuffer {
            /// Get a region of pixels from the image buffer.
            fn pixels(&self, region_of_interest: &RegionOfInterest) -> Result<Vec<$rust_type>> {
                if ImageBufferStorage::Uninitialized == self.storage() {
                    // An uninitialized image buffer has no pixels but it's not
                    // an error to ask for them.
                    return Ok(Vec::new());
                }

                let region = match region_of_interest {
                    RegionOfInterest::All => match self.data_window() {
                        RegionOfInterest::All => {
                            // If this image buffer is uninitialized, we can't
                            // get here because
                            // `self.storage()` will return
                            // `ImageBufferStorage::Uninitialized`.
                            unreachable!()
                        }
                        RegionOfInterest::Region(roi) => roi,
                    },
                    RegionOfInterest::Region(roi) => roi.clone(),
                };

                let size = region.pixel_count() * region.channel_count() as usize;
                let mut data = Vec::<$rust_type>::with_capacity(size);
                let mut is_ok = std::mem::MaybeUninit::<bool>::uninit();

                unsafe {
                    oiio_ImageBuf_get_pixels(
                        self.ptr,
                        region_of_interest.clone().into(),
                        $base_type,
                        data.as_mut_ptr() as _,
                        &mut is_ok as *mut _ as _,
                    );

                    if is_ok.assume_init() {
                        data.set_len(size);
                        Ok(data)
                    } else {
                        Err(anyhow!(
                            self.error(true)
                                .unwrap_or("ImageBuffer::pixels(): unknown error".into())
                        ))
                    }
                }
            }
        }
    };
}

pixels!(u8, oiio_BASETYPE::oiio_BASETYPE_UINT8);
pixels!(u16, oiio_BASETYPE::oiio_BASETYPE_UINT16);
pixels!(u32, oiio_BASETYPE::oiio_BASETYPE_UINT32);
pixels!(u64, oiio_BASETYPE::oiio_BASETYPE_UINT64);
pixels!(i8, oiio_BASETYPE::oiio_BASETYPE_INT8);
pixels!(i16, oiio_BASETYPE::oiio_BASETYPE_INT16);
pixels!(i32, oiio_BASETYPE::oiio_BASETYPE_INT32);
pixels!(i64, oiio_BASETYPE::oiio_BASETYPE_INT64);
pixels!(f32, oiio_BASETYPE::oiio_BASETYPE_FLOAT);
pixels!(f64, oiio_BASETYPE::oiio_BASETYPE_DOUBLE);
