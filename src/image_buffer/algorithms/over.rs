use crate::{algorithms::*, *};

/// # Over
///
/// These implement [Porter-Duff compositing](https://en.wikipedia.org/wiki/Alpha_compositing).
impl ImageBuffer {
    #[named]
    pub fn from_over(a: &ImageBuffer, b: &ImageBuffer) -> Result<Self> {
        let image_buffer = ImageBuffer::from_over_ffi(a, b, Options::default());

        image_buffer.self_or_error(true, function_name!())
    }

    #[named]
    pub fn from_over_with(
        a: &ImageBuffer,
        b: &ImageBuffer,
        options: Options,
    ) -> Result<Self> {
        let image_buffer = ImageBuffer::from_over_ffi(a, b, options);

        image_buffer.self_or_error(true, function_name!())
    }

    #[named]
    pub fn over(&mut self, other: &ImageBuffer) -> Result<&mut Self> {
        let is_ok = self.over_ffi(other, Options::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn over_with(
        &mut self,
        other: &ImageBuffer,
        options: Options,
    ) -> Result<&mut Self> {
        let is_ok = self.over_ffi(other, options);

        self.mut_self_or_error(is_ok, function_name!())
    }
}

impl ImageBuffer {
    fn over_ffi(&mut self, other: &ImageBuffer, options: Options) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_over(
                self.ptr,
                self.ptr,
                other.ptr,
                options.region_of_interest.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }

    fn from_over_ffi(
        a: &ImageBuffer,
        b: &ImageBuffer,
        options: Options,
    ) -> ImageBuffer {
        let mut ptr = MaybeUninit::<*mut oiio_ImageBuf_t>::uninit();

        unsafe {
            oiio_ImageBufAlgo_from_over(
                a.ptr,
                b.ptr,
                options.region_of_interest.clone().into(),
                options.thread_count as _,
                &mut ptr as *mut _ as _,
            );

            Self {
                ptr: ptr.assume_init(),
                image_cache: None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn over() -> Result<()> {
        let mut image_buf_a = ImageBuffer::new();
        let mut image_buf_b = ImageBuffer::new();
        let image_buf_c = ImageBuffer::new();

        //println!("Over test");
        image_buf_a.over(image_buf_b.over(&image_buf_c)?)?;

        //println!("Over test done");
        Ok(())
    }
}
