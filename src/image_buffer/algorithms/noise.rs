use crate::*;
use core::mem::MaybeUninit;

/// Optional parameters for [`ImageBuffer`]'s
/// [`replace_by_noise_with()`](ImageBuffer::replace_by_noise_with) and
/// [`noise_with()`](ImageBuffer::noise_with) methods.
#[derive(Clone, Default)]
pub struct NoiseOptions {
    /// If this flag is `true`, a single noise value will be applied to all
    /// channels specified by `region_of_interest`, but if it is `false`, a
    /// separate noise value will be computed for each channel in the
    /// region.
    pub monochromatic: bool,
    /// The random number generator is actually driven by a hash on the *image
    /// space* coordinates and channel, independently of the *pixel data
    /// window* of of the resp. [`ImageBuffer`] or the
    /// [`RegionOfInterest`].
    /// Choosing different seed values will result in a different pattern, but
    /// for the same seed value, the noise at a given pixel coordinate
    /// `(x, y, z)` in channel `c` will be completely deterministic and
    /// repeatable.
    pub seed: i32,
    /// See the [Region of Interest](#region-of-interest) section on
    /// [`ImageBuffer`].
    pub region_of_interest: RegionOfInterest,
    /// See the [Multithreading](#multithreading) section on [`ImageBuffer`].
    pub thread_count: u16,
}

pub enum NoiseType {
    Gaussian,
    White,
    Uniform,
    Blue,
    Salt,
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
impl ImageBuffer {
    /// Add noise.
    #[named]
    pub fn noise(&mut self, noise_type: NoiseType, a: f32, b: f32) -> Result<&mut Self> {
        let is_ok = self.noise_ffi(noise_type, a, b, &NoiseOptions::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    /// Add noise with [`NoiseOptions`].
    #[named]
    pub fn noise_with(
        &mut self,
        noise_type: NoiseType,
        a: f32,
        b: f32,
        options: &NoiseOptions,
    ) -> Result<&mut Self> {
        let is_ok = self.noise_ffi(noise_type, a, b, options);

        self.mut_self_or_error(is_ok, function_name!())
    }
}

impl From<NoiseType> for StringView<'static> {
    fn from(noise_type: NoiseType) -> Self {
        match noise_type {
            NoiseType::Gaussian => StringView::from("gaussian"),
            NoiseType::White => StringView::from("white"),
            NoiseType::Uniform => StringView::from("uniform"),
            NoiseType::Blue => StringView::from("blue"),
            NoiseType::Salt => StringView::from("salt"),
        }
    }
}

// Internal noise FFI call.
impl ImageBuffer {
    #[inline(always)]
    fn noise_ffi(&mut self, noise_type: NoiseType, a: f32, b: f32, options: &NoiseOptions) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_noise(
                self.as_raw_ptr_mut(),
                StringView::from(noise_type).as_raw_ptr() as _,
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
}
