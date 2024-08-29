use crate::{operators::Options, *};
use anyhow::Result;
use std::mem::MaybeUninit;

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
                std::mem::transmute::<RegionOfInterest, oiio_ROI_t>(
                    options.region_of_interest.clone(),
                ),
                options.thread_count as _,
                &mut is_ok as *mut _ as *mut _,
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
                std::mem::transmute::<RegionOfInterest, oiio_ROI_t>(
                    options.region_of_interest.clone(),
                ),
                options.thread_count as _,
                &mut is_ok as *mut _ as *mut _,
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
