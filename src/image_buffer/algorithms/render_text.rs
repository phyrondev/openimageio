use crate::*;

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
        let is_ok = image_buffer.render_text_ffi(x, y, text, &RenderTextOptions::default());

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
    pub fn render_text(&mut self, x: i32, y: i32, text: &str) -> Result<&mut Self> {
        let is_ok = self.render_text_ffi(x, y, text, &RenderTextOptions::default());

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

/// Text alignment in the horizontal direction.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
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
            TextAlignX::Center => oiio_TextAlignX::oiio_TextAlignX_Center.0 as _,
            TextAlignX::Right => oiio_TextAlignX::oiio_TextAlignX_Right.0 as _,
        })
    }
}

/// Text alignment in the vertical direction.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
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
            TextAlignY::Baseline => oiio_TextAlignY::oiio_TextAlignY_Baseline.0 as _,
            TextAlignY::Top => oiio_TextAlignY::oiio_TextAlignY_Top.0 as _,
            TextAlignY::Bottom => oiio_TextAlignY::oiio_TextAlignY_Bottom.0 as _,
            TextAlignY::Center => oiio_TextAlignY::oiio_TextAlignY_Center.0 as _,
        })
    }
}
/// Optional parameters for [`ImageBuffer`]'s
/// [`from_render_text_with()`](ImageBuffer::from_render_text_with),
/// [`render_text_with()`](ImageBuffer::render_text_with) methods.
#[derive(Clone, Debug, PartialEq)]
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
    pub font_name: Option<&'a str>,
    /// Color for drawing the text, defaulting to opaque white `[1.0, 1.0, …]`
    /// in all channels if `None`. If provided, it is expected to point to an
    /// `[f32]` slice of length at least equal to
    /// `R.specification().channel_count`, or defaults will be chosen for you.
    pub color: &'a [f32],
    /// Text alignment in the horizontal direction.
    pub text_align_x: TextAlignX,
    /// Text alignment in the vertical direction.
    pub text_align_y: TextAlignY,
    /// If nonzero, a 'outline' of this radius will be used to make the
    /// text look more clear by dilating the alpha channel of the composite
    /// (makes a black halo around the characters).
    pub outline: u16,
    /// See the [Region](#region-of-interest) section on [`ImageBuffer`].
    pub region: Region,
    /// See the [Multithreading](#multithreading) section on [`ImageBuffer`].
    pub thread_count: u16,
}

impl Default for RenderTextOptions<'_> {
    fn default() -> Self {
        Self {
            font_size: 16,
            font_name: None,
            color: &[1.0],
            text_align_x: TextAlignX::default(),
            text_align_y: TextAlignY::default(),
            outline: 0,
            region: Region::default(),
            thread_count: 0,
        }
    }
}

impl ImageBuffer {
    #[inline]
    fn render_text_ffi(&mut self, x: i32, y: i32, text: &str, options: &RenderTextOptions) -> bool {
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
                    .map_or(StringView::default(), StringView::from)
                    .as_raw_ptr() as _,
                CspanF32::new(options.color).as_raw_ptr() as _,
                options.text_align_x.into(),
                options.text_align_y.into(),
                options.outline as _,
                options.region.clone().into(),
                options.thread_count as _,
                &raw mut is_ok as _,
            );

            is_ok.assume_init()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{algorithms::*, *};

    #[test]
    fn render_text() -> Result<()> {
        let mut image_buffer =
            ImageBuffer::from_file(Utf8Path::new("assets/wooden_lounge_2k__F32_RGBA.exr"))?;

        image_buffer.render_text_with(
            512,
            256,
            "Kringers Fossed!",
            &RenderTextOptions {
                font_size: 128,
                font_name: Some("assets/ProtestGuerrilla-Regular.ttf"),
                text_align_x: TextAlignX::Center,
                text_align_y: TextAlignY::Center,
                color: &[1.0, 0.0, 0.0, 0.25],
                ..Default::default()
            },
        )?;

        image_buffer.write(Utf8Path::new("target/render_text.exr"))?;

        Ok(())
    }
}
