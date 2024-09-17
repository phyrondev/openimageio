use crate::*;
use core::{mem::MaybeUninit, ptr};

pub struct ColorConvertOptions {
    pub unpremultiply: bool,
    pub context_key: Option<Ustr>,
    pub context_value: Option<Ustr>,
    pub config: Option<ColorConfig>,
    pub region_of_interest: RegionOfInterest,
    pub thread_count: u16,
}

impl Default for ColorConvertOptions {
    fn default() -> Self {
        Self {
            unpremultiply: true,
            context_key: None,
            context_value: None,
            config: None,
            region_of_interest: RegionOfInterest::default(),
            thread_count: 0,
        }
    }
}

/// # Color Conversion
impl ImageBuffer {
    #[named]
    pub fn color_convert(
        &mut self,
        from_space: Option<Ustr>,
        to_space: Ustr,
    ) -> Result<&mut Self> {
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
        let is_ok =
            image_buffer.color_convert_ffi(self, from_space, to_space, options);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_color_convert(
        source: &ImageBuffer,
        from_space: Option<Ustr>,
        to_space: Ustr,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.color_convert_ffi(
            source,
            from_space,
            to_space,
            &ColorConvertOptions::default(),
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_color_convert_with(
        source: &ImageBuffer,
        from_space: Option<Ustr>,
        to_space: Ustr,
        options: &ColorConvertOptions,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer
            .color_convert_ffi(source, from_space, to_space, options);

        image_buffer.self_or_error(is_ok, function_name!())
    }
}

// Actual implementations.
impl ImageBuffer {
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
                    .context_key
                    .map_or(StringView::default(), StringView::from)
                    .as_raw_ptr() as _,
                options
                    .context_value
                    .map_or(StringView::default(), StringView::from)
                    .as_raw_ptr() as _,
                options
                    .config
                    .as_ref()
                    .map_or(ptr::null_mut(), |s| *s.read_arc()),
                options.region_of_interest.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }
}
