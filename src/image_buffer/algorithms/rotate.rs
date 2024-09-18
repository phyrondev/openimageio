use crate::{algorithms::*, *};
use core::ptr;

/// # Rotate
///
/// Rotate the image.
///
/// ## Parameters
///
/// * `angle` -- The angle (in radians, with positive angles rotating
///   clockwise).
///
/// * `center_x`, `center_y` -- The center of rotation. The variants that lack
///   these parameters rotate around the center of the image's *display window*.
///
/// Only the pixels (and channels) of dst that are specified by roi will be
/// copied from the rotated src; the default roi is to alter all the pixels in
/// dst. If dst is uninitialized, it will be resized to be an ImageBuf large
/// enough to hold the rotated image if recompute_roi is true, or will have the
/// same ROI as src if recompute_roi is false. It is an error to pass both an
/// uninitialized dst and an undefined roi. The filter is used to weight the src
/// pixels falling underneath it for each dst pixel. The caller may specify a
/// reconstruction filter by name and width (expressed in pixels units of the
/// dst image), or rotate() will choose a reasonable default high-quality
/// default filter ([`Lanczos3`](PixelFilter::Lanczos3)) if the empty string is
/// passed, and a reasonable filter width if filterwidth is 0. (Note that some
/// filter choices only make sense with particular width, in which case this
/// filterwidth parameter may be ignored.)
impl ImageBuffer {
    #[named]
    pub fn from_rotate(source: &ImageBuffer, angle: f32) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok =
            image_buffer.rotate_ffi(source, angle, &RotateOptions::default());
        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_rotate_with(
        source: &ImageBuffer,
        angle: f32,
        options: &RotateOptions,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.rotate_ffi(source, angle, options);

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_rotate_around(
        source: &ImageBuffer,
        angle: f32,
        center_x: f32,
        center_y: f32,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.rotate_around_ffi(
            source,
            angle,
            center_x,
            center_y,
            &RotateOptions::default(),
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_rotate_around_with(
        source: &ImageBuffer,
        angle: f32,
        center_x: f32,
        center_y: f32,
        options: &RotateOptions,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer
            .rotate_around_ffi(source, angle, center_x, center_y, options);

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn rotate(&mut self, angle: f32) -> Result<&mut Self> {
        let mut rotated = ImageBuffer::new();
        let is_ok = rotated.rotate_ffi(self, angle, &RotateOptions::default());
        *self = rotated;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn rotate_with(
        &mut self,
        angle: f32,
        options: &RotateOptions,
    ) -> Result<&mut Self> {
        let mut rotated = ImageBuffer::new();
        let is_ok = rotated.rotate_ffi(self, angle, options);
        *self = rotated;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn rotate_around(
        &mut self,
        angle: f32,
        center_x: f32,
        center_y: f32,
    ) -> Result<&mut Self> {
        let mut rotated = ImageBuffer::new();
        let is_ok = rotated.rotate_around_ffi(
            self,
            angle,
            center_x,
            center_y,
            &RotateOptions::default(),
        );
        *self = rotated;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn rotate_around_with(
        &mut self,
        angle: f32,
        center_x: f32,
        center_y: f32,
        options: &RotateOptions,
    ) -> Result<&mut Self> {
        let mut rotated = ImageBuffer::new();
        let is_ok =
            rotated.rotate_around_ffi(self, angle, center_x, center_y, options);
        *self = rotated;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

#[derive(Clone, Default)]
pub struct RotateOptions {
    pub filter: Option<Filter2D>,
    pub recompute_region_of_interest: bool,
    pub region_of_interest: RegionOfInterest,
    pub thread_count: u16,
}

impl ImageBuffer {
    fn rotate_ffi(
        &mut self,
        other: &ImageBuffer,
        angle: f32,
        options: &RotateOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_rotate(
                self.ptr,
                other.ptr,
                angle,
                options.filter.map_or(ptr::null(), |f| f.as_raw_ptr()) as _,
                options.recompute_region_of_interest,
                options.region_of_interest.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }

    fn rotate_around_ffi(
        &mut self,
        other: &ImageBuffer,
        angle: f32,
        center_x: f32,
        center_y: f32,
        options: &RotateOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_rotate_around(
                self.ptr,
                other.ptr,
                angle,
                center_x,
                center_y,
                options.filter.map_or(ptr::null(), |f| f.as_raw_ptr()) as _,
                options.recompute_region_of_interest,
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
    use crate::{algorithms::*, *};

    #[test]
    fn rotate() -> Result<()> {
        let image_buf = ImageBuffer::from_file(Utf8Path::new(
            "assets/wooden_lounge_2k__F32_RGBA.exr",
        ))?;

        let image_buf = ImageBuffer::from_rotate_with(
            &image_buf,
            42.0 * std::f32::consts::TAU / 360.0,
            &RotateOptions {
                // Use a Blackmann-Harris filter to avoid halos easily
                // introduced when operating on HDRs with the default one,
                // Lanczos3.
                filter: Some(PixelFilter::BlackmanHarris.into()),
                recompute_region_of_interest: true,
                ..Default::default()
            },
        )?;

        image_buf.write(Utf8Path::new("rotated.exr"))?;

        Ok(())
    }
}
