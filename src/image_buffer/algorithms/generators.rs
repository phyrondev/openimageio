use crate::{image_buffer::algorithms::Options, *};
use anyhow::Result;
use core::{mem::MaybeUninit, ptr};
use ustr::Ustr;

impl ImageBuffer {
    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
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

/// # Fill
///
/// Fill an image region with given channel values,
///
///  Note that the value slices start with channel 0, even if the
/// `RegionOfInterest` indicates that a later channel is the first to be
/// changed.
///
/// Three varieties of `fill...()` exist:
///
/// * [Uniform fill](#uniform-fill)
///
/// * [Vertical gradient fill](#vertical-gradient-fill)
///
/// * [Four corner gradient fill](#four-corner-gradient-fill)
///
/// ## Uniform Fill
///
/// A single set of channel values that will apply to the whole
/// `RegionOfInterest`.
impl ImageBuffer {
    #[named]
    pub fn from_fill(values: &[f32], region: &Region) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.fill_ffi(
            values,
            &Options {
                region_of_interest: RegionOfInterest::Region(region.clone()),
                ..Default::default()
            },
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_fill_with(
        values: &[f32],
        region: &Region,
        thread_count: Option<u16>,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.fill_ffi(
            values,
            &Options {
                region_of_interest: RegionOfInterest::Region(region.clone()),
                thread_count: thread_count.unwrap_or(0),
            },
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn fill(&mut self, values: &[f32]) -> Result<&mut Self> {
        let is_ok = self.fill_ffi(values, &Options::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn fill_with(
        &mut self,
        values: &[f32],
        options: &Options,
    ) -> Result<&mut Self> {
        let is_ok = self.fill_ffi(values, options);

        self.mut_self_or_error(is_ok, function_name!())
    }
}

/// ## Vertical Gradient Fill
///
/// Two sets of valuesthat will create a linearly interpolated gradient from top
/// to bottom of the `RegionOfInterest`.
impl ImageBuffer {
    #[named]
    pub fn from_fill_vertical(
        start: &[f32],
        end: &[f32],
        region: &Region,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.fill_vertical_ffi(
            start,
            end,
            &Options {
                region_of_interest: RegionOfInterest::Region(region.clone()),
                ..Default::default()
            },
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_fill_vertical_with(
        start: &[f32],
        end: &[f32],
        region: &Region,
        thread_count: Option<u16>,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.fill_vertical_ffi(
            start,
            end,
            &Options {
                region_of_interest: RegionOfInterest::Region(region.clone()),
                thread_count: thread_count.unwrap_or(0),
            },
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn fill_vertical(
        &mut self,
        start: &[f32],
        end: &[f32],
    ) -> Result<&mut Self> {
        let is_ok = self.fill_vertical_ffi(start, end, &Options::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn fill_vertical_with(
        &mut self,
        start: &[f32],
        end: &[f32],
        options: &Options,
    ) -> Result<&mut Self> {
        let is_ok = self.fill_vertical_ffi(start, end, options);

        self.mut_self_or_error(is_ok, function_name!())
    }
}

/// ## Four Corner Gradient Fill
///
/// Four sets of values that will be bilinearly interpolated across all four
/// corners of the `RegionOfInterest`.
impl ImageBuffer {
    #[named]
    pub fn from_fill_corners(
        top_left: &[f32],
        top_right: &[f32],
        bottom_left: &[f32],
        bottom_right: &[f32],
        region: &Region,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.fill_corners_ffi(
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            &Options {
                region_of_interest: RegionOfInterest::Region(region.clone()),
                ..Default::default()
            },
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_fill_corners_with(
        top_left: &[f32],
        top_right: &[f32],
        bottom_left: &[f32],
        bottom_right: &[f32],
        region: &Region,
        thread_count: Option<u16>,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.fill_corners_ffi(
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            &Options {
                region_of_interest: RegionOfInterest::Region(region.clone()),
                thread_count: thread_count.unwrap_or(0),
            },
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn fill_corners(
        &mut self,
        top_left: &[f32],
        top_right: &[f32],
        bottom_left: &[f32],
        bottom_right: &[f32],
    ) -> Result<&mut Self> {
        let is_ok = self.fill_corners_ffi(
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            &Options::default(),
        );

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn fill_corners_with(
        &mut self,
        top_left: &[f32],
        top_right: &[f32],
        bottom_left: &[f32],
        bottom_right: &[f32],
        options: &Options,
    ) -> Result<&mut Self> {
        let is_ok = self.fill_corners_ffi(
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            options,
        );

        self.mut_self_or_error(is_ok, function_name!())
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
            &Region::new(0..640, 0..480, Some(0..1), Some(0..3)),
        )?;

        // Draw a filled red rectangle overtop existing image A.
        image_buf.fill_with(
            &red,
            &Options {
                region_of_interest: RegionOfInterest::Region(Region::new_2d(
                    50..100,
                    75..175,
                )),
                ..Default::default()
            },
        )?;

        // Draw a filled red rectangle overtop existing image A.
        image_buf.fill_corners(&red, &blue, &yellow, &pink)?;

        compare_images(&image_buf, "test_fill.png")?;

        Ok(())
    }
}

/// Optional parameters for [`ImageBuffer`]'s
/// [`from_noise_with()`](ImageBuffer::from_noise_with) and
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

impl ImageBuffer {
    #[inline(always)]
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
}

impl ImageBuffer {
    #[named]
    pub fn from_zero(region: &Region) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.zero_ffi(&Options {
            region_of_interest: RegionOfInterest::Region(region.clone()),
            ..Default::default()
        });

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_zero_with(
        region: &Region,
        thread_count: Option<u16>,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.zero_ffi(&Options {
            region_of_interest: RegionOfInterest::Region(region.clone()),
            thread_count: thread_count.unwrap_or(0),
        });

        image_buffer.self_or_error(is_ok, function_name!())
    }

    /// Set all channels to zero.
    #[named]
    pub fn zero(&mut self) -> Result<&mut Self> {
        let is_ok = self.zero_ffi(&Options::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    /// Set all channels to as described by the [`RegionOfInterest`] to
    /// zero.
    #[named]
    pub fn zero_with(&mut self, options: &Options) -> Result<&mut Self> {
        let is_ok = self.zero_ffi(options);

        self.mut_self_or_error(is_ok, function_name!())
    }
}

pub enum NoiseType {
    Gaussian,
    White,
    Uniform,
    Blue,
    Salt,
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
    fn noise_ffi(
        &mut self,
        noise_type: NoiseType,
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
    pub fn noise(
        &mut self,
        noise_type: NoiseType,
        a: f32,
        b: f32,
    ) -> Result<&mut Self> {
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

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub enum TextAlignX {
    #[default]
    Left = oiio_TextAlignX::oiio_TextAlignX_Left.0 as _,
    Center = oiio_TextAlignX::oiio_TextAlignX_Center.0 as _,
    Right = oiio_TextAlignX::oiio_TextAlignX_Right.0 as _,
}

impl From<TextAlignX> for oiio_TextAlignX {
    fn from(text_align_x: TextAlignX) -> Self {
        Self(match text_align_x {
            TextAlignX::Left => oiio_TextAlignX::oiio_TextAlignX_Left.0 as _,
            TextAlignX::Center => {
                oiio_TextAlignX::oiio_TextAlignX_Center.0 as _
            }
            TextAlignX::Right => oiio_TextAlignX::oiio_TextAlignX_Right.0 as _,
        })
    }
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub enum TextAlignY {
    #[default]
    Baseline = oiio_TextAlignY::oiio_TextAlignY_Baseline.0 as _,
    Top = oiio_TextAlignY::oiio_TextAlignY_Top.0 as _,
    Bottom = oiio_TextAlignY::oiio_TextAlignY_Bottom.0 as _,
    Center = oiio_TextAlignY::oiio_TextAlignY_Center.0 as _,
}

impl From<TextAlignY> for oiio_TextAlignY {
    fn from(text_align_y: TextAlignY) -> Self {
        Self(match text_align_y {
            TextAlignY::Baseline => {
                oiio_TextAlignY::oiio_TextAlignY_Baseline.0 as _
            }
            TextAlignY::Top => oiio_TextAlignY::oiio_TextAlignY_Top.0 as _,
            TextAlignY::Bottom => {
                oiio_TextAlignY::oiio_TextAlignY_Bottom.0 as _
            }
            TextAlignY::Center => {
                oiio_TextAlignY::oiio_TextAlignY_Center.0 as _
            }
        })
    }
}

#[derive(Clone, PartialEq)]
pub struct RenderTextOptions<'a> {
    /// The nominal height of the font (in pixels).
    pub font_size: u16,
    /// The name of the font. If the name is not a full pathname to a font
    /// file, it will search for a matching font, defaulting to some reasonable
    /// system font if not supplied at all).
    ///
    /// Note that any named fonts (if not a full pathname) will search for the
    /// fonts in the following places:
    ///
    /// 1. Any directories named in the global `font_searchpath` attribute or
    ///    the `$OPENIMAGEIO_FONTS` environment variable.
    ///
    /// 2. Any font-related subdirectories (`fonts`, `Fonts`, `share/fonts`, or
    ///    `Library/Fonts`) underneath the directories in environment variables
    ///    `$HOME`, `$SystemRoot`, `$OpenImageIO_ROOT`.
    ///
    /// 3. A number of common system font areas, including `/usr/share/fonts`,
    ///    `/Library/fonts`, and `C:/Windows/fonts`.
    ///
    /// 4. In fonts directories one level up from the place where the currently
    ///    running binary lives.
    pub font_name: Option<Ustr>,
    /// Color for drawing the text, defaulting to opaque white `[1.0, 1.0, …]`
    /// in all channels if `None`. If provided, it is expected to point to an
    /// `[f32]` slice of length at least equal to
    /// `R.specification().channel_count`, or defaults will be chosen for you.
    pub color: Option<&'a [f32]>,
    /// Text alignment in the horizontal direction.
    pub text_align_x: TextAlignX,
    /// Text alignment in the vertical direction.
    pub text_align_y: TextAlignY,
    /// If nonzero, a 'drop shadow' of this radius will be used to make the
    /// text look more clear by dilating the alpha channel of the composite
    /// (makes a black halo around the characters).
    pub shadow_size: u16,
    /// See the [Region of Interest](#region-of-interest) section on
    /// [`ImageBuffer`].
    pub region_of_interest: RegionOfInterest,
    /// See the [Multithreading](#multithreading) section on [`ImageBuffer`].
    pub thread_count: u16,
}

impl Default for RenderTextOptions<'_> {
    fn default() -> Self {
        Self {
            font_size: 16,
            font_name: None,
            color: None,
            text_align_x: TextAlignX::default(),
            text_align_y: TextAlignY::default(),
            shadow_size: 0,
            region_of_interest: RegionOfInterest::default(),
            thread_count: 0,
        }
    }
}

impl ImageBuffer {
    #[inline(always)]
    fn render_text_ffi(
        &mut self,
        x: i32,
        y: i32,
        text: &str,
        options: &RenderTextOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_render_text(
                self.ptr,
                x,
                y,
                StringView::from(text).ptr,
                options.font_size as _,
                options
                    .font_name
                    .as_ref()
                    .map(|s| StringView::from(s).ptr)
                    .unwrap_or(ptr::null_mut()),
                options
                    .color
                    .as_ref()
                    .map(|c| CspanF32::new(c).ptr)
                    .unwrap_or(CspanF32::new(&[1.0]).ptr) as _,
                options.text_align_x.into(),
                options.text_align_y.into(),
                options.shadow_size as _,
                options.region_of_interest.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }
}

/// # Render Text
///
/// Render a text string (encoded as UTF-8).
///
/// # Parameters
///
/// * `x`, `y` -– The position to place the text.
///
/// * `text` –- The text to draw. Linefeed (`\n`) characters are respected as
///   indications that the text spans multiple rows.
///
/// * `options` -- See [`RenderTextOptions`].
impl ImageBuffer {
    /// Render text into an image.
    ///
    /// Text will be rendered into the existing image by essentially doing an
    /// 'over' of the character into the existing pixel data.
    #[named]
    pub fn render_text(
        &mut self,
        x: i32,
        y: i32,
        text: &str,
    ) -> Result<&mut Self> {
        let is_ok =
            self.render_text_ffi(x, y, text, &RenderTextOptions::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    /// Render text into an image buffer with given [`RenderTextOptions`].
    ///
    /// Text will be rendered into the existing image by essentially doing an
    /// 'over' of the character into the existing pixel data.
    #[named]
    pub fn render_text_with(
        &mut self,
        x: i32,
        y: i32,
        text: &str,
        options: &RenderTextOptions,
    ) -> Result<&mut Self> {
        let is_ok = self.render_text_ffi(x, y, text, options);

        self.mut_self_or_error(is_ok, function_name!())
    }

    /// Create an image buffer from rendering text.
    ///
    /// The resulting image will be initialized to be a black background exactly
    /// large enough to contain the rasterized text.
    #[named]
    pub fn from_render_text(x: i32, y: i32, text: &str) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.render_text_ffi(
            x,
            y,
            text,
            &RenderTextOptions::default(),
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    /// Create an image buffer from rendering text with given
    /// [`RenderTextOptions`].
    ///
    /// The resulting image will be initialized to be a black background exactly
    /// large enough to contain the rasterized text.
    #[named]
    pub fn from_render_text_with(
        x: i32,
        y: i32,
        text: &str,
        options: &RenderTextOptions,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.render_text_ffi(x, y, text, options);

        image_buffer.self_or_error(is_ok, function_name!())
    }
}
