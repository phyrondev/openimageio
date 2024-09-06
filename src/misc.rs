use crate::*;
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
use anyhow::Result;

#[cfg(test)]
pub(crate) fn compare_images(
    image_buf: &ImageBuffer,
    name: &str,
) -> Result<()> {
    use camino::Utf8Path;

    let other =
        ImageBuffer::from_file(&Utf8Path::new("test_results").join(name))?;

    if image_buf.compare(&other, 1.0 / 255.0, 0.0).is_error {
        Err(anyhow::anyhow!("Image comparison for {name} failed."))
    } else {
        Ok(())
    }
}
