use crate::*;
use core::{marker::PhantomData, mem::MaybeUninit};
use num_traits::{Bounded, Num, NumCast};
use anyhow::Result;

/// The type of each channel in a pixel. For example, this can be `u8`, `u16`,
/// `f32`.
pub trait Primitive:
    Copy + NumCast + Num + PartialOrd<Self> + Clone + Bounded
{
    /// The maximum value for this type of primitive within the context of
    /// color. For floats, the maximum is `1.0`, whereas the integer types
    /// inherit their usual maximum values.
    const DEFAULT_MAX_VALUE: Self;

    /// The minimum value for this type of primitive within the context of
    /// color. For floats, the minimum is `0.0`, whereas the integer types
    /// inherit their usual minimum values.
    const DEFAULT_MIN_VALUE: Self;
}

macro_rules! declare_primitive {
    ($base:ty: ($from:expr)..$to:expr) => {
        impl Primitive for $base {
            const DEFAULT_MAX_VALUE: Self = $to;
            const DEFAULT_MIN_VALUE: Self = $from;
        }
    };
}

declare_primitive!(usize: (0)..Self::MAX);
declare_primitive!(u8: (0)..Self::MAX);
declare_primitive!(u16: (0)..Self::MAX);
declare_primitive!(u32: (0)..Self::MAX);
declare_primitive!(u64: (0)..Self::MAX);

declare_primitive!(isize: (Self::MIN)..Self::MAX);
declare_primitive!(i8: (Self::MIN)..Self::MAX);
declare_primitive!(i16: (Self::MIN)..Self::MAX);
declare_primitive!(i32: (Self::MIN)..Self::MAX);
declare_primitive!(i64: (Self::MIN)..Self::MAX);
declare_primitive!(f32: (0.0)..1.0);
declare_primitive!(f64: (0.0)..1.0);

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
