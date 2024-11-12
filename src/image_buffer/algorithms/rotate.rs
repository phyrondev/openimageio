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
    pub fn replace_by_rotate(&mut self, source: &ImageBuffer, angle: f32) -> Result<&mut Self> {
        let is_ok = self.rotate_ffi(source, angle, &RotateOptions::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_by_rotate_with(
        &mut self,
        source: &ImageBuffer,
        angle: f32,
        options: &RotateOptions,
    ) -> Result<&mut Self> {
        let is_ok = self.rotate_ffi(source, angle, options);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_by_rotate_around(
        &mut self,
        source: &ImageBuffer,
        angle: f32,
        center_x: f32,
        center_y: f32,
    ) -> Result<&mut Self> {
        let is_ok =
            self.rotate_around_ffi(source, angle, center_x, center_y, &RotateOptions::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_by_rotate_around_with(
        &mut self,
        source: &ImageBuffer,
        angle: f32,
        center_x: f32,
        center_y: f32,
        options: &RotateOptions,
    ) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.rotate_around_ffi(source, angle, center_x, center_y, options);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn rotate(&mut self, angle: f32) -> Result<&mut Self> {
        let mut rotated = ImageBuffer::new();
        let is_ok = rotated.rotate_ffi(self, angle, &RotateOptions::default());
        *self = rotated;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn rotate_with(&mut self, angle: f32, options: &RotateOptions) -> Result<&mut Self> {
        let mut rotated = ImageBuffer::new();
        let is_ok = rotated.rotate_ffi(self, angle, options);
        *self = rotated;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn rotate_around(&mut self, angle: f32, center_x: f32, center_y: f32) -> Result<&mut Self> {
        let mut rotated = ImageBuffer::new();
        let is_ok =
            rotated.rotate_around_ffi(self, angle, center_x, center_y, &RotateOptions::default());
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
        let is_ok = rotated.rotate_around_ffi(self, angle, center_x, center_y, options);
        *self = rotated;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

/// Optional parameters for [`ImageBuffer`]'s
/// [`rotate_with()`](ImageBuffer::rotate_with),
/// [`replace_by_rotate_with()`](ImageBuffer::replace_by_rotate_with)
/// methods.
#[derive(Clone, Default)]
pub struct RotateOptions {
    pub filter: Option<Filter2D>,
    pub recompute_region: bool,
    pub region: Region,
    pub thread_count: u16,
}

impl ImageBuffer {
    #[inline]
    fn rotate_ffi(&mut self, other: &ImageBuffer, angle: f32, options: &RotateOptions) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_rotate(
                self.as_raw_ptr_mut(),
                other.as_raw_ptr(),
                angle,
                options.filter.map_or(ptr::null(), |f| f.as_raw_ptr()) as _,
                options.recompute_region,
                options.region.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }

    #[inline]
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
                self.as_raw_ptr_mut(),
                other.as_raw_ptr(),
                angle,
                center_x,
                center_y,
                options.filter.map_or(ptr::null(), |f| f.as_raw_ptr()) as _,
                options.recompute_region,
                options.region.clone().into(),
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
        let image_buffer =
            ImageBuffer::from_file(Utf8Path::new("assets/wooden_lounge_2k__F32_RGBA.exr"))?;

        let mut rotated_image_buffer = ImageBuffer::new();

        rotated_image_buffer.replace_by_rotate_with(
            &image_buffer,
            42.0 * std::f32::consts::TAU / 360.0,
            &RotateOptions {
                // Use a Blackmann-Harris filter to avoid halos easily
                // introduced when operating on HDRs with the default one,
                // Lanczos3.
                filter: Some(PixelFilter2D::BlackmanHarris.into()),
                recompute_region: true,
                ..Default::default()
            },
        )?;

        rotated_image_buffer.set_display_to_data_window();

        rotated_image_buffer.write(Utf8Path::new("target/rotate.exr"))
    }
}
