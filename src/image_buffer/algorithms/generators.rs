use crate::*;
use anyhow::Result;
use std::mem::MaybeUninit;

impl<'a> ImageBuffer<'a> {
    fn fill_ffi(&mut self, values: &[f32], options: &Options) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_fill(
                self.ptr,
                CspanF32::new(values).ptr as _,
                *options.region_of_interest.as_raw_ptr(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }

    fn fill_vertical_ffi(
        &mut self,
        start: &[f32],
        end: &[f32],
        options: &Options,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_fill_vertical(
                self.ptr,
                CspanF32::new(start).ptr as _,
                CspanF32::new(end).ptr as _,
                *options.region_of_interest.as_raw_ptr(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }

    fn fill_corners_ffi(
        &mut self,
        top_left: &[f32],
        top_right: &[f32],
        bottom_left: &[f32],
        bottom_right: &[f32],
        options: &Options,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_fill_corners(
                self.ptr,
                CspanF32::new(top_left).ptr as _,
                CspanF32::new(top_right).ptr as _,
                CspanF32::new(bottom_left).ptr as _,
                CspanF32::new(bottom_right).ptr as _,
                *options.region_of_interest.as_raw_ptr(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }
}

/// Fill an image region with given channel values,
///
///  Note that the value slices start with channel 0, even if the
/// `RegionOfInterest` indicates that a later channel is the first to be
/// changed.
///
/// Three varieties of `fill...()` exist:
///
/// * [A single set](#uniform-fill) of channel values that will apply to the
///   whole `RegionOfInterest`.
///
/// * Two sets of values that will create a linearly interpolated gradient from
///   top to bottom of the `RegionOfInterest`.
///
/// * Four sets of values that will be bilinearly interpolated across all four
///   corners of the `RegionOfInterest`.
///
/// # Uniform Fill
impl<'a> ImageBuffer<'a> {
    #[inline]
    pub fn from_fill(values: &[f32], options: &Options) -> Self {
        let mut image_buffer = ImageBuffer::new();
        image_buffer.fill_with(values, options);
        image_buffer
    }

    pub fn fill(&mut self, values: &[f32]) -> &mut Self {
        let is_ok = self.fill_ffi(values, &Options::default());

        self.ok_or_log_error(is_ok);
        self
    }

    pub fn fill_with(
        &mut self,
        values: &[f32],
        options: &Options,
    ) -> &mut Self {
        let is_ok = self.fill_ffi(values, options);

        self.ok_or_log_error(is_ok);
        self
    }
}

/// # Vertical Gradient Fill
impl<'a> ImageBuffer<'a> {
    #[inline]
    pub fn from_fill_vertical(
        start: &[f32],
        end: &[f32],
        options: &Options,
    ) -> Self {
        let mut image_buffer = ImageBuffer::new();
        image_buffer.fill_vertical_with(start, end, options);
        image_buffer
    }

    pub fn fill_vertical(&mut self, start: &[f32], end: &[f32]) -> &mut Self {
        let is_ok = self.fill_vertical_ffi(start, end, &Options::default());

        self.ok_or_log_error(is_ok);
        self
    }

    pub fn fill_vertical_with(
        &mut self,
        start: &[f32],
        end: &[f32],
        options: &Options,
    ) -> &mut Self {
        let is_ok = self.fill_vertical_ffi(start, end, options);

        self.ok_or_log_error(is_ok);
        self
    }
}

/// # Four Corner Gradient Fill
impl<'a> ImageBuffer<'a> {
    #[inline]
    pub fn from_fill_corners(
        top_left: &[f32],
        top_right: &[f32],
        bottom_left: &[f32],
        bottom_right: &[f32],
        options: &Options,
    ) -> Self {
        let mut image_buffer = ImageBuffer::new();
        image_buffer.fill_corners_with(
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            options,
        );
        image_buffer
    }

    pub fn fill_corners(
        &mut self,
        top_left: &[f32],
        top_right: &[f32],
        bottom_left: &[f32],
        bottom_right: &[f32],
    ) -> &mut Self {
        let is_ok = self.fill_corners_ffi(
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            &Options::default(),
        );

        self.ok_or_log_error(is_ok);
        self
    }

    pub fn fill_corners_with(
        &mut self,
        top_left: &[f32],
        top_right: &[f32],
        bottom_left: &[f32],
        bottom_right: &[f32],
        options: &Options,
    ) -> &mut Self {
        let is_ok = self.fill_corners_ffi(
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            options,
        );

        self.ok_or_log_error(is_ok);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn fill() -> Result<()> {
        let pink = [1.0, 0.7, 0.7];
        let red = [1.0, 0.0, 0.0];
        let blue = [0.0, 0.1, 0.8];
        let yellow = [0.5, 0.5, 0.0];

        // Create a new 640x480 RGB image, with a top-to-bottom gradient
        // from red to pink

        let mut image_buf = ImageBuffer::from_fill_vertical(
            &pink,
            &red,
            &Options {
                region_of_interest: Roi::Region(Region::new(
                    0..640,
                    0..480,
                    Some(0..1),
                    Some(0..3),
                )),
                ..Default::default()
            },
        );

        // Draw a filled red rectangle overtop existing image A.
        image_buf.fill_with(
            &red,
            &Options {
                region_of_interest: Roi::Region(Region::new_2d(
                    50..100,
                    75..175,
                )),
                ..Default::default()
            },
        );

        // Draw a filled red rectangle overtop existing image A.
        image_buf.fill_corners_with(
            &red,
            &blue,
            &yellow,
            &pink,
            &Options {
                region_of_interest: Roi::Region(Region::new_2d(
                    100..160,
                    175..275,
                )),
                ..Default::default()
            },
        );

        compare_images(&image_buf, "test_fill.png")?;

        Ok(())
    }
}

/// Optional parameters for [`ImageBuffer`]'s
/// [`noise_with()`](ImageBuffer::noise_with) and
/// [`try_noise_with()`](ImageBuffer::try_noise_with) methods.
#[derive(Clone, Default)]
pub struct NoiseOptions {
    /// If this flag is `true`, a single noise value will be applied to all
    /// channels specified by `region_of_interest`, but if it is `false`, a
    /// separate noise value will be computed for each channel in the
    /// region.
    monochromatic: bool,
    /// The random number generator is actually driven by a hash on the *image
    /// space* coordinates and channel, independently of the *pixel data
    /// window* of of the resp. [`ImageBuffer`] or the
    /// [`RegionOfInterest`].
    /// Choosing different seed values will result in a different pattern, but
    /// for the same seed value, the noise at a given pixel coordinate
    /// `(x, y, z)` in channel `c` will be completely deterministic and
    /// repeatable.
    seed: i32,
    region_of_interest: RegionOfInterest,
    thread_count: u16,
}

impl<'a> ImageBuffer<'a> {
    fn zero_ffi(&mut self, options: &Options) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_zero(
                self.ptr,
                options.region_of_interest.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }

    /// Set all channels to black.
    ///
    /// Errors will be logged.
    pub fn zero(&mut self) -> &mut Self {
        let is_ok = self.zero_ffi(&Options::default());
        self.ok_or_log_error(is_ok)
    }

    /// Set all channels as described by the [`RegionOfInterest`] to black.
    ///
    /// Errors will be logged.
    pub fn zero_with(&mut self, options: &Options) -> &mut Self {
        let is_ok = self.zero_ffi(options);
        self.ok_or_log_error(is_ok)
    }

    /// Try setting all channels as described by the [`RegionOfInterest`] to
    /// black.
    pub fn try_zero(&mut self) -> Result<&mut Self> {
        let is_ok = self.zero_ffi(&Options::default());
        self.ok_or_error(is_ok)
    }

    /// Try setting all channels as described by the [`RegionOfInterest`] to
    /// black.
    pub fn try_zero_with(&mut self, options: &Options) -> Result<&mut Self> {
        let is_ok = self.zero_ffi(options);
        self.ok_or_error(is_ok)
    }
}

/// # Noise
///
/// Return an image of 'noise' in every pixel and channel.
///
/// There are several `noise_type`s to choose from, and each behaves differently
/// and has a different interpretation of the A and B parameters:
///
/// * `gaussian` -- adds Gaussian (normal distribution) noise values with mean
///   value `a` and standard deviation `b`.
///
/// * `white` -- adds independent uniformly distributed values on range `[a,b)`.
///
/// * `uniform` -- synonym for `white`
///
/// * `blue` adds "blue noise" uniformly distributed on range `[a,b)` but not
///   independent; rather, they are chosen for good spectral properties for
///   sampling and dither.
///
/// * `salt` changes to value `a` the portion of pixels given by `b`.
impl<'a> ImageBuffer<'a> {
    fn noise_ffi(
        &mut self,
        noise_type: &str,
        a: f32,
        b: f32,
        options: &NoiseOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_noise(
                self.ptr,
                StringView::from(noise_type).ptr,
                a,
                b,
                options.monochromatic,
                options.seed,
                options.region_of_interest.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }

    /// Add noise.
    ///
    /// Errors will be logged.
    pub fn noise(&mut self, noise_type: &str, a: f32, b: f32) -> &mut Self {
        let is_ok = self.noise_ffi(noise_type, a, b, &NoiseOptions::default());
        self.ok_or_log_error(is_ok)
    }

    /// Add noise with [`NoiseOptions`].
    ///
    /// Errors will be logged.
    pub fn noise_with(
        &mut self,
        noise_type: &str,
        a: f32,
        b: f32,
        options: &NoiseOptions,
    ) -> &mut Self {
        let is_ok = self.noise_ffi(noise_type, a, b, options);
        self.ok_or_log_error(is_ok)
    }

    /// Try adding noise.
    pub fn try_noise(
        &mut self,
        noise_type: &str,
        a: f32,
        b: f32,
    ) -> Result<&mut Self> {
        let is_ok = self.noise_ffi(noise_type, a, b, &NoiseOptions::default());
        self.ok_or_error(is_ok)
    }

    /// Try adding noise with [`NoiseOptions`].
    pub fn try_noise_with(
        &mut self,
        noise_type: &str,
        a: f32,
        b: f32,
        options: &NoiseOptions,
    ) -> Result<&mut Self> {
        let is_ok = self.noise_ffi(noise_type, a, b, options);
        self.ok_or_error(is_ok)
    }
}
