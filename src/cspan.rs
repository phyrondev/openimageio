use crate::*;
use core::{marker::PhantomData, mem::MaybeUninit};

macro_rules! cspan {
    ($oiio_type:ty, $ctor_name:ident, $dtor_name:ident, $rust_name:ident, $rust_type:ty) => {
        pub struct $rust_name<'a> {
            ptr: *const $oiio_type,
            marker: PhantomData<*const &'a ()>,
        }

        impl<'a> $rust_name<'a> {
            pub fn new(data: &'a [$rust_type]) -> Self {
                let mut ptr = MaybeUninit::<*const $oiio_type>::uninit();

                unsafe {
                    $ctor_name(data.as_ptr() as _, data.len() as _, &raw mut ptr as _);

                    Self {
                        ptr: ptr.assume_init(),
                        marker: PhantomData,
                    }
                }
            }

            #[inline(always)]
            pub fn as_raw_ptr(&self) -> *const $oiio_type {
                self.ptr
            }
        }

        impl Drop for $rust_name<'_> {
            fn drop(&mut self) {
                unsafe {
                    $dtor_name(self.ptr as _);
                }
            }
        }
    };
}

cspan!(
    oiio_CspanF32_t,
    oiio_CspanF32_ctor,
    oiio_CspanF32_dtor,
    CspanF32,
    f32
);

cspan!(
    oiio_CspanF64_t,
    oiio_CspanF64_ctor,
    oiio_CspanF64_dtor,
    CspanF64,
    f64
);

cspan!(
    oiio_CspanU8_t,
    oiio_CspanU8_ctor,
    oiio_CspanU8_dtor,
    CspanI8,
    i8
);

cspan!(
    oiio_CspanU8_t,
    oiio_CspanU8_ctor,
    oiio_CspanU8_dtor,
    CspanU8,
    u8
);

cspan!(
    oiio_CspanU16_t,
    oiio_CspanU16_ctor,
    oiio_CspanU16_dtor,
    CspanI16,
    i16
);

cspan!(
    oiio_CspanU16_t,
    oiio_CspanU16_ctor,
    oiio_CspanU16_dtor,
    CspanU16,
    u16
);

cspan!(
    oiio_CspanI32_t,
    oiio_CspanI32_ctor,
    oiio_CspanI32_dtor,
    CspanI32,
    i32
);

cspan!(
    oiio_CspanU32_t,
    oiio_CspanU32_ctor,
    oiio_CspanU32_dtor,
    CspanU32,
    u32
);

/*
pub struct CspanF32<'a> {
    ptr: *const oiio_CspanF32_t,
    marker: PhantomData<*const &'a ()>,
}

impl<'a> CspanF32<'a> {
    pub fn new(data: &'a [f32]) -> Self {
        let mut ptr = MaybeUninit::<*const oiio_CspanF32_t>::uninit();

        unsafe {
            oiio_CspanF32_ctor(data.as_ptr() as _, data.len() as _, &raw mut ptr as _);

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

impl CspanF32<'_> {
    #[inline(always)]
    pub fn as_raw_ptr(&self) -> *const oiio_CspanF32_t {
        self.ptr
    }
}*/
