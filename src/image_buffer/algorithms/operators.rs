//! # Operators
//!
//! ## Image Editing
//!
//! * [Channel Shuffling](ImageBuffer#channel-shuffling)
//! *
//!
//! ## Transformations
//! * [rotate90/180/270](ImageBuffer#rotate90-180-270)
//! * [Flip-Flop-Transpose](ImageBuffer#flip-flop-transpose)
//! * [Rotation](ImageBuffer#rotate)
//! * [Resize](ImageBuffer#resize)
//! * [Warping (2D Transformation)](ImageBuffer#warp)
//!
//! ## Compositing
//!
//! * [Over](ImageBuffer#over)
use crate::*;
use anyhow::Result;
use std::{marker::PhantomData, mem::MaybeUninit};

#[derive(Clone, Copy, Default)]
#[non_exhaustive]
pub enum PixelFilter {
    Gaussian,
    SharpGaussian,
    Box,
    Triangle,
    Mitchell,
    BlackmanHarris,
    Bspline,
    CatmullRom,
    #[default]
    Lanczos3,
    Cubic,
    Keys,
    Simon,
    Rifman,
    Disk,
    Binomial,
    Laplacian,
}

impl From<PixelFilter> for &str {
    fn from(pf: PixelFilter) -> Self {
        match pf {
            PixelFilter::Gaussian => "gaussian",
            PixelFilter::SharpGaussian => "sharp-gaussian",
            PixelFilter::Box => "box",
            PixelFilter::Triangle => "triangle",
            PixelFilter::Mitchell => "mitchell",
            PixelFilter::BlackmanHarris => "blackman-harris",
            PixelFilter::Bspline => "b-spline",
            PixelFilter::CatmullRom => "catmull-rom",
            PixelFilter::Lanczos3 => "lanczos3",
            PixelFilter::Cubic => "cubic",
            PixelFilter::Keys => "keys",
            PixelFilter::Simon => "simon",
            PixelFilter::Rifman => "rifman",
            PixelFilter::Disk => "disk",
            PixelFilter::Binomial => "binomial",
            PixelFilter::Laplacian => "laplacian",
            //_ => "unknown",
        }
    }
}

// Actual implementations.
impl<'a> ImageBuffer<'a> {
    fn over_ffi(&mut self, other: &ImageBuffer, options: Options) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_over(
                self.ptr,
                self.ptr,
                other.ptr,
                std::mem::transmute::<RegionOfInterest, oiio_ROI_t>(
                    options.region_of_interest,
                ),
                options.thread_count as _,
                &mut is_ok as *mut _ as *mut _,
            );

            is_ok.assume_init()
        }
    }

    fn from_over_ffi(
        a: &ImageBuffer,
        b: &ImageBuffer,
        options: Options,
    ) -> ImageBuffer<'a> {
        let mut ptr = MaybeUninit::<*mut oiio_ImageBuf_t>::uninit();

        unsafe {
            oiio_ImageBufAlgo_from_over(
                a.ptr,
                b.ptr,
                std::mem::transmute::<RegionOfInterest, oiio_ROI_t>(
                    options.region_of_interest,
                ),
                options.thread_count as _,
                &mut ptr as *mut _ as *mut _,
            );

            Self {
                ptr: ptr.assume_init(),
                _marker: PhantomData,
            }
        }
    }

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
                StringView::from(Into::<&str>::into(options.pixel_filter))
                    .as_raw_ptr_mut(),
                0.0,
                options.recompute_region_of_interest,
                std::mem::transmute::<Roi, oiio_ROI_t>(
                    options.region_of_interest.clone(),
                ),
                options.thread_count as _,
                &mut is_ok as *mut _ as *mut _,
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
                StringView::from(Into::<&str>::into(options.pixel_filter))
                    .as_raw_ptr_mut(),
                0.0,
                options.recompute_region_of_interest,
                std::mem::transmute::<Roi, oiio_ROI_t>(
                    options.region_of_interest.clone(),
                ),
                options.thread_count as _,
                &mut is_ok as *mut _ as *mut _,
            );

            is_ok.assume_init()
        }
    }
}

/// Generic options accepted by most compositing operators.
#[derive(Clone, Default)]
pub struct Options {
    pub region_of_interest: RegionOfInterest,
    pub thread_count: u16,
}

#[derive(Clone, Default)]
pub struct RotateOptions {
    pub pixel_filter: PixelFilter,
    pub recompute_region_of_interest: bool,
    pub region_of_interest: RegionOfInterest,
    pub thread_count: u16,
}

/// # Rotate
///
/// Rotate the src image by the angle (in radians, with positive angles
/// clockwise). When `center_x` and `center_y` are supplied, they denote the
/// center of rotation; in their absence, the rotation will be about the center
/// of the image's *display window*.
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
impl<'a> ImageBuffer<'a> {
    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        let mut rotated = ImageBuffer::new();
        let is_ok = rotated.rotate_ffi(self, angle, &RotateOptions::default());
        *self = rotated;

        self.ok_or_log_error(is_ok);
        self
    }

    pub fn rotate_with(
        &mut self,
        angle: f32,
        options: &RotateOptions,
    ) -> &mut Self {
        let mut rotated = ImageBuffer::new();
        let is_ok = rotated.rotate_ffi(self, angle, options);
        *self = rotated;

        self.ok_or_log_error(is_ok);
        self
    }

    pub fn rotate_around(
        &mut self,
        angle: f32,
        center_x: f32,
        center_y: f32,
    ) -> &mut Self {
        let mut rotated = ImageBuffer::new();
        let is_ok = rotated.rotate_around_ffi(
            self,
            angle,
            center_x,
            center_y,
            &RotateOptions::default(),
        );
        *self = rotated;

        self.ok_or_log_error(is_ok);
        self
    }

    pub fn rotate_around_with(
        &mut self,
        angle: f32,
        center_x: f32,
        center_y: f32,
        options: &RotateOptions,
    ) -> &mut Self {
        let mut rotated = ImageBuffer::new();
        let is_ok =
            rotated.rotate_around_ffi(self, angle, center_x, center_y, options);
        *self = rotated;

        self.ok_or_log_error(is_ok);
        self
    }
}

/// # Over
///
/// These implement [Porter-Duff compositing](https://en.wikipedia.org/wiki/Alpha_compositing).
impl<'a> ImageBuffer<'a> {
    pub fn from_over(a: &ImageBuffer, b: &ImageBuffer) -> Self {
        let mut image_buffer =
            ImageBuffer::from_over_ffi(a, b, Options::default());
        image_buffer.ok_or_log_error(image_buffer.is_ok());
        image_buffer
    }

    pub fn from_over_with(
        a: &ImageBuffer,
        b: &ImageBuffer,
        options: Options,
    ) -> Self {
        let mut image_buffer = ImageBuffer::from_over_ffi(a, b, options);
        image_buffer.ok_or_log_error(image_buffer.is_ok());
        image_buffer
    }

    pub fn try_from_over(a: &ImageBuffer, b: &ImageBuffer) -> Result<Self> {
        let mut image_buffer =
            ImageBuffer::from_over_ffi(a, b, Options::default());
        image_buffer.self_or_error()
    }

    pub fn try_from_over_with(
        a: &ImageBuffer,
        b: &ImageBuffer,
        options: Options,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::from_over_ffi(a, b, options);
        image_buffer.self_or_error()
    }

    pub fn over(&mut self, other: &ImageBuffer) -> &mut Self {
        let is_ok = self.over_ffi(other, Options::default());
        self.ok_or_log_error(is_ok);
        self
    }

    pub fn over_with(
        &mut self,
        other: &ImageBuffer,
        options: Options,
    ) -> &mut Self {
        let is_ok = self.over_ffi(other, options);
        self.ok_or_log_error(is_ok)
    }

    pub fn try_over(&mut self, other: &ImageBuffer) -> Result<&mut Self> {
        let is_ok = self.over_ffi(other, Options::default());
        self.ok_or_error(is_ok)
    }

    pub fn try_over_with(
        &mut self,
        other: &ImageBuffer,
        options: Options,
    ) -> Result<&mut Self> {
        let is_ok = self.over_ffi(other, options);
        self.ok_or_error(is_ok)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn over() {
        let mut image_buf_a = ImageBuffer::new();
        let mut image_buf_b = ImageBuffer::new();
        let image_buf_c = ImageBuffer::new();

        image_buf_a.over(image_buf_b.over(&image_buf_c));
    }

    #[test]
    fn rotate() -> Result<()> {
        use crate::operators::PixelFilter::BlackmanHarris;
        use camino::Utf8Path as Path;

        let mut image_buf = ImageBuffer::from_file(Path::new(
            "assets/wooden_lounge_2k__F32_RGBA.exr",
        ));

        image_buf.rotate_with(
            42.0 * std::f32::consts::TAU / 360.0,
            &RotateOptions {
                // Use a Blackmann-Harris filter to avoid halos easily
                // introduced when operating on HDRs with the default one,
                // Lanczos3.
                pixel_filter: BlackmanHarris,
                ..Default::default()
            },
        );

        image_buf.write(Path::new("rotated.exr"), None, None)?;

        Ok(())
    }
}
