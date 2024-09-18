use crate::*;
use core::{mem::MaybeUninit, ptr};

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub enum TextAlignX {
    #[default]
    Left = oiio_TextAlignX::oiio_TextAlignX_Left.0 as _,
    Center = oiio_TextAlignX::oiio_TextAlignX_Center.0 as _,
    Right = oiio_TextAlignX::oiio_TextAlignX_Right.0 as _,
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub enum TextAlignY {
    #[default]
    Baseline = oiio_TextAlignY::oiio_TextAlignY_Baseline.0 as _,
    Top = oiio_TextAlignY::oiio_TextAlignY_Top.0 as _,
    Bottom = oiio_TextAlignY::oiio_TextAlignY_Bottom.0 as _,
    Center = oiio_TextAlignY::oiio_TextAlignY_Center.0 as _,
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
    /// Create an `ImageBuffer` from rendering text.
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

    /// Create an `ImageBuffer` from rendering text with given
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

    /// Render text.
    ///
    /// Text will be rendered into the existing image by essentially doing an
    /// 'over' of the characters onto the existing pixel data.
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

    /// Render text with given [`RenderTextOptions`].
    ///
    /// Text will be rendered into the existing image by essentially doing an
    /// 'over' of the characters onto the existing pixel data.
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
