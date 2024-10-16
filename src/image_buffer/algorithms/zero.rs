use crate::{algorithms::*, *};
use core::mem::MaybeUninit;

impl ImageBuffer {
    #[named]
    pub fn from_zero(bounds: &Bounds) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.zero_ffi(&Options {
            region: Region::Bounds(bounds.clone()),
            ..Default::default()
        });

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_zero_with(bounds: &Bounds, thread_count: Option<u16>) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.zero_ffi(&Options {
            region: Region::Bounds(bounds.clone()),
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

    /// Set all channels as described by the [`Region`] to
    /// zero.
    #[named]
    pub fn zero_with(&mut self, options: &Options) -> Result<&mut Self> {
        let is_ok = self.zero_ffi(options);

        self.mut_self_or_error(is_ok, function_name!())
    }
}

impl ImageBuffer {
    #[inline]
    fn zero_ffi(&mut self, options: &Options) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_zero(
                self.ptr,
                options.region.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }
}
