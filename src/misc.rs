use crate::*;
#[cfg(test)]
use anyhow::Result;
use core::{marker::PhantomData, mem::MaybeUninit};

pub(crate) struct CspanF32<'a> {
    pub(crate) ptr: *const oiio_CspanF32_t,
    marker: PhantomData<*const &'a ()>,
}

impl<'a> CspanF32<'a> {
    pub fn new(data: &'a [f32]) -> Self {
        let mut ptr = MaybeUninit::<*const oiio_CspanF32_t>::uninit();

        unsafe {
            oiio_CspanF32_ctor(
                data.as_ptr() as _,
                data.len() as _,
                &mut ptr as *mut _ as _,
            );

            Self {
                ptr: ptr.assume_init(),
                marker: PhantomData,
            }
        }
    }
}

impl Drop for CspanF32<'_> {
    fn drop(&mut self) {
        unsafe {
            oiio_CspanF32_dtor(self.ptr as _);
        }
    }
}

#[cfg(test)]
pub fn compare_images(image_buf: &ImageBuffer, name: &str) -> Result<()> {
    use camino::Utf8Path;

    let other =
        ImageBuffer::from_file(&Utf8Path::new("test_results").join(name))?;

    if image_buf.compare(&other, 1.0 / 255.0, 0.0).is_error {
        Err(anyhow::anyhow!("Image comparison for {name} failed."))
    } else {
        Ok(())
    }
}

macro_rules! gen_fn_error {
    ($error_fn:expr) => {
        /// Return the text of all pending error messages.
        ///
        /// If `clear` is `true`, any pending error message will be cleared.
        ///
        /// If no error message is pending, this will return `None`.
        pub fn error(&self, clear: bool) -> Option<String> {
            let mut error = MaybeUninit::<*mut oiio_String_t>::uninit();

            if unsafe {
                0 != $error_fn(self.ptr, clear, &mut error as *mut _ as _)
            } {
                // Something went wrong.
                None
            } else {
                let error = OiioString::from(unsafe { error.assume_init() });

                if error.is_empty() {
                    None
                } else {
                    Some(error.to_string())
                }
            }
        }
    };
}

pub(crate) use gen_fn_error;

macro_rules! gen_fn_is_ok {
    ($has_error_fn:expr) => {
        /// Returns `false` if there was an error.
        ///
        /// The latter implies that an error message is ready to retrieve via
        /// `error()`.
        pub fn is_ok(&self) -> bool {
            let mut is_error = MaybeUninit::<bool>::uninit();

            unsafe {
                $has_error_fn(self.ptr, &mut is_error as *mut _ as _);

                !is_error.assume_init()
            }
        }
    };
}

pub(crate) use gen_fn_is_ok;
