use crate::*;
use core::{
    ffi::{c_char, c_ulong},
    mem::MaybeUninit,
    slice,
};
use std::fmt::{Display, Formatter};

// Wraps a C++ String.
pub(crate) struct OiioString {
    ptr: *mut oiio_String_t,
}

impl OiioString {
    pub fn new(s: &str) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_String_t>::uninit();

        unsafe {
            oiio_String_ctor(
                s.as_ptr() as _,
                s.len().try_into().unwrap(),
                &raw mut ptr as _,
            );
            Self {
                ptr: ptr.assume_init(),
            }
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        let mut is_empty = std::mem::MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_String_empty(self.ptr, &mut is_empty as *mut _ as _);

            is_empty.assume_init()
        }
    }

    #[allow(dead_code)]
    pub(crate) fn len(&self) -> usize {
        let mut len = std::mem::MaybeUninit::<usize>::uninit();

        unsafe {
            oiio_String_length(self.ptr, &mut len as *mut _ as _);

            len.assume_init()
        }
    }
}

impl Drop for OiioString {
    fn drop(&mut self) {
        unsafe { oiio_String_dtor(self.ptr) };
    }
}

impl From<*mut oiio_String_t> for OiioString {
    fn from(ptr: *mut oiio_String_t) -> Self {
        Self { ptr }
    }
}

/// This fails with an [`Error`](std::fmt::Error) both if the formattimng does
/// not succeed or if the string contains non-valid UTF-8.
impl Display for OiioString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ptr = MaybeUninit::<*const c_char>::uninit();
        let mut size = MaybeUninit::<c_ulong>::uninit();

        unsafe {
            oiio_String_data(self.ptr, &raw mut ptr as _);
            oiio_String_size(self.ptr, &raw mut size as _);

            // TODO: check with lg that a OIIO::String is guranteed to always be
            // valid UTF8.
            write!(
                f,
                "{}",
                std::string::String::from_utf8(
                    slice::from_raw_parts(ptr.assume_init() as _, size.assume_init() as _,)
                        .to_vec(),
                )
                .map_err(|_| std::fmt::Error)?
            )
        }
    }
}

impl OiioString {
    pub fn as_raw_ptr(&self) -> *const oiio_String_t {
        self.ptr
    }

    pub fn _as_raw_ptr_mut(&mut self) -> *mut oiio_String_t {
        self.ptr
    }
}
