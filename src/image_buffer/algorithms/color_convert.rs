use crate::*;
use core::{mem::MaybeUninit, ptr};

pub struct ColorConvertOptions<'a> {
    /// If `true` (the default), unpremultiply the image (divide the RGB
    /// channels by alpha if it exists and is nonzero) before color
    /// conversion, then repremult after the after the color conversion.
    ///
    /// Passing `false` skips this step, which may be desirable if
    /// you know that the image is "unassociated alpha" (a.k.a., "not
    /// pre-multiplied colors").
    pub unpremultiply: bool,
    /// Define an optional context via a key-value tuple (for example, a
    /// shot-specific transform).
    pub context: Option<(&'a str, &'a str)>,
    pub config: Option<ColorConfig>,
    pub region: Region,
    pub thread_count: u16,
}

impl Default for ColorConvertOptions<'_> {
    fn default() -> Self {
        Self {
            unpremultiply: true,
            context: None,
            config: None,
            region: Region::default(),
            thread_count: 0,
        }
    }
}

/// # Color Conversion
impl ImageBuffer {
    #[named]
    pub fn replace_by_color_convert(
        &mut self,
        source: &ImageBuffer,
        from_space: Option<Ustr>,
        to_space: Ustr,
    ) -> Result<&mut Self> {
        let is_ok = self.color_convert_ffi(
            source,
            from_space,
            to_space,
            &ColorConvertOptions::default(),
        );

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_by_color_convert_with(
        &mut self,
        source: &ImageBuffer,
        from_space: Option<Ustr>,
        to_space: Ustr,
        options: &ColorConvertOptions,
    ) -> Result<&mut Self> {
        let is_ok = self.color_convert_ffi(source, from_space, to_space, options);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn color_convert(&mut self, from_space: Option<Ustr>, to_space: Ustr) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.color_convert_ffi(
            self,
            from_space,
            to_space,
            &ColorConvertOptions::default(),
        );
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn color_convert_with(
        &mut self,
        from_space: Option<Ustr>,
        to_space: Ustr,
        options: &ColorConvertOptions,
    ) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.color_convert_ffi(self, from_space, to_space, options);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

// Actual implementations.
impl ImageBuffer {
    #[inline]
    fn color_convert_ffi(
        &mut self,
        source: &ImageBuffer,
        from_space: Option<Ustr>,
        to_space: Ustr,
        options: &ColorConvertOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_colorconvert(
                self.as_raw_ptr_mut(),
                source.as_raw_ptr(),
                from_space
                    .map_or(StringView::default(), StringView::from)
                    .as_raw_ptr() as _,
                StringView::from(to_space).as_raw_ptr() as _,
                options.unpremultiply,
                options
                    .context
                    .map_or(StringView::default(), |c| StringView::from(c.0))
                    .as_raw_ptr() as _,
                options
                    .context
                    .map_or(StringView::default(), |c| StringView::from(c.1))
                    .as_raw_ptr() as _,
                options
                    .config
                    .as_ref()
                    .map_or(ptr::null_mut(), |s| *s.read_arc()),
                options.region.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }
}
