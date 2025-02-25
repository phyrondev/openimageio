use crate::algorithms::*;

/// Optional parameters for [`ImageBuffer`]'s
/// [`noise_with()`](ImageBuffer::noise_with) method.
#[derive(Clone, Default)]
pub struct NoiseOptions {
    /// If this flag is `true`, a single noise value will be applied to all
    /// channels specified by `region`, but if it is `false`, a separate noise
    /// value will be computed for each channel in the region.
    pub monochromatic: bool,
    /// The random number generator is actually driven by a hash of the *image
    /// space* coordinates and channel, independently of the *pixel data
    /// window* of of the resp. [`ImageBuffer`] or the [`Region`].
    ///
    /// Choosing different seed values will result in a different pattern, but
    /// for the same seed value, the noise at a given pixel coordinate
    /// `(x, y, z)` in channel `c` will be completely deterministic and
    /// repeatable.
    pub seed: i32,
    /// See the [Region of Interest](module@algorithms#region-of-interest)
    /// section in the [module@algorithms] module. .
    pub region: Region,
    /// See the [Multithreading](module@algorithms#multithreading) section
    /// in the [module@algorithms] module.
    pub thread_count: u16,
}

/// The type of noise used by [`ImageBuffer`]'s
/// [`noise_with()`](ImageBuffer::noise_with) method.
#[derive(Debug, Clone)]
pub enum NoiseType {
    /// Gaussian (*normal* distribution) noise values with `standard_deviation`
    /// around `mean`.
    Gaussian { mean: f32, standard_deviation: f32 },
    /// Independent (*uniform* distribution) noise values on range *[`min`,
    /// `max`)*.
    White { min: f32, max: f32 },
    /// 'Blue noise' uniformly distributed on range *[`min`, `max`)* but not
    /// independent; rather, they are chosen for good spectral properties
    /// for sampling and dither.
    Blue { min: f32, max: f32 },
    /// Changes to value `salt` the portion of pixels given by
    /// `percentage_salted`.
    Salt { salt: f32, percentage_salted: f32 },
}

impl Default for NoiseType {
    fn default() -> Self {
        NoiseType::Gaussian {
            mean: 0.0,
            standard_deviation: 0.1,
        }
    }
}

/// # Noise
///
/// Return an image of 'noise' in every pixel and channel.
///
/// There are several [`NoiseType`]s to choose from, and each behaves
/// differently and has a different interpretation of the `a` and `b`
/// parameters:
impl ImageBuffer {
    /// Add noise.
    #[named]
    pub fn noise(&mut self, noise_type: NoiseType) -> Result<&mut Self> {
        let is_ok = self.noise_ffi(noise_type, &NoiseOptions::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    /// Add noise with [`NoiseOptions`].
    #[named]
    pub fn noise_with(
        &mut self,
        noise_type: NoiseType,
        options: &NoiseOptions,
    ) -> Result<&mut Self> {
        let is_ok = self.noise_ffi(noise_type, options);

        self.mut_self_or_error(is_ok, function_name!())
    }
}

impl From<NoiseType> for StringView<'static> {
    fn from(noise_type: NoiseType) -> Self {
        match noise_type {
            NoiseType::Gaussian { .. } => StringView::from("gaussian"),
            NoiseType::White { .. } => StringView::from("white"),
            NoiseType::Blue { .. } => StringView::from("blue"),
            NoiseType::Salt { .. } => StringView::from("salt"),
        }
    }
}

impl From<NoiseType> for (f32, f32) {
    fn from(noise_type: NoiseType) -> Self {
        match noise_type {
            NoiseType::Gaussian {
                mean,
                standard_deviation,
            } => (mean, standard_deviation),
            NoiseType::White { min, max } => (min, max),
            NoiseType::Blue { min, max } => (min, max),
            NoiseType::Salt {
                salt,
                percentage_salted,
            } => (salt, percentage_salted),
        }
    }
}

// Internal noise FFI call.
impl ImageBuffer {
    #[inline]
    fn noise_ffi(&mut self, noise_type: NoiseType, options: &NoiseOptions) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        let (a, b) = noise_type.clone().into();

        unsafe {
            oiio_ImageBufAlgo_noise(
                self.as_raw_ptr_mut(),
                StringView::from(noise_type).as_raw_ptr() as _,
                a,
                b,
                options.monochromatic,
                options.seed,
                options.region.clone().into(),
                options.thread_count as _,
                &raw mut is_ok as _,
            );

            is_ok.assume_init()
        }
    }
}
