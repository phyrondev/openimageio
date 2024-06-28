use crate::*;
use std::{
    ffi::{c_char, c_ulong},
    mem::MaybeUninit,
    slice,
};

pub(crate) struct OiioString(*mut oiio_String_t);

impl Drop for OiioString {
    fn drop(&mut self) {
        unsafe { oiio_String_dtor(self.0) };
    }
}

impl From<*mut oiio_String_t> for OiioString {
    fn from(s: *mut oiio_String_t) -> OiioString {
        Self(s)
    }
}

impl OiioString {
    pub fn new(s: &str) -> OiioString {
        let mut ptr = MaybeUninit::<*mut oiio_String_t>::uninit();

        unsafe {
            oiio_String_ctor(
                s.as_ptr() as _,
                s.len().try_into().unwrap(),
                &mut ptr as *mut _ as *mut _,
            );
            Self(ptr.assume_init())
        }
    }

    pub fn to_string(self) -> String {
        unsafe {
            let mut ptr = MaybeUninit::<*const c_char>::uninit();
            oiio_String_data(self.0, &mut ptr as *mut _ as *mut _);

            let mut size = MaybeUninit::<c_ulong>::uninit();
            oiio_String_size(self.0, &mut size as *mut _ as *mut _);

            String::from_utf8_unchecked(
                slice::from_raw_parts(
                    ptr.assume_init() as _,
                    size.assume_init() as _,
                )
                .to_vec(),
            )
        }
    }

    pub fn as_ptr(&self) -> *const oiio_String_t {
        self.0
    }

    pub fn as_mut_ptr(&mut self) -> *mut oiio_String_t {
        self.0
    }
}
