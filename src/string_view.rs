use crate::*;
use camino::Utf8Path;
use core::slice;
use std::{
    ffi::c_char,
    fmt::{Display, Formatter},
    marker::PhantomData,
    mem::MaybeUninit,
    path::Path,
};

pub struct StringView<'a> {
    ptr: *mut oiio_StringView_t,
    // _marker needs to be invariant in 'a.
    // See "Making a struct outlive a parameter given to a method of
    // that struct": https://stackoverflow.com/questions/62374326/
    _marker: PhantomData<*mut &'a ()>,
}

impl<'a> Default for StringView<'a> {
    fn default() -> Self {
        let mut ptr = std::mem::MaybeUninit::<*mut oiio_StringView_t>::uninit();
        unsafe {
            oiio_StringView_ctor_default(&mut ptr as *mut _ as *mut _);

            Self {
                ptr: ptr.assume_init(),
                _marker: PhantomData,
            }
        }
    }
}

impl<'a> From<&'a str> for StringView<'a> {
    fn from(string: &'a str) -> Self {
        let mut ptr = std::mem::MaybeUninit::<*mut oiio_StringView_t>::uninit();
        unsafe {
            oiio_StringView_ctor(
                string.as_ptr() as *const _,
                string.len().try_into().unwrap(),
                &mut ptr as *mut _ as *mut _,
            );

            Self {
                ptr: ptr.assume_init(),
                _marker: PhantomData,
            }
        }
    }
}

impl<'a> From<&'a Path> for StringView<'a> {
    fn from(path: &'a Path) -> Self {
        let mut ptr = std::mem::MaybeUninit::<*mut oiio_StringView_t>::uninit();
        let os_str_path = path.as_os_str();
        unsafe {
            oiio_StringView_ctor(
                os_str_path.as_encoded_bytes().as_ptr() as *const _,
                os_str_path.len().try_into().unwrap(),
                &mut ptr as *mut _ as *mut _,
            );

            Self {
                ptr: ptr.assume_init(),
                _marker: PhantomData,
            }
        }
    }
}

impl<'a> From<&'a Utf8Path> for StringView<'a> {
    fn from(path: &'a Utf8Path) -> Self {
        let mut ptr = std::mem::MaybeUninit::<*mut oiio_StringView_t>::uninit();
        let str_path = path.as_str();
        unsafe {
            oiio_StringView_ctor(
                str_path.as_ptr() as *const _,
                str_path.len().try_into().unwrap(),
                &mut ptr as *mut _ as *mut _,
            );

            Self {
                ptr: ptr.assume_init(),
                _marker: PhantomData,
            }
        }
    }
}

/// This fails with an [`Error`](std::fmt::Error) both if the formattimng does
/// not succeed or if the string contains non-valid UTF-8.
impl Display for StringView<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut data = MaybeUninit::<*const c_char>::uninit();
        let mut size = MaybeUninit::<usize>::uninit();

        unsafe {
            oiio_StringView_data(self.ptr, &mut data as *mut _ as *mut _);
            oiio_StringView_size(self.ptr, &mut size as *mut _ as *mut _);

            // TODO: check with lg that a OIIO::String is guranteed to always be
            // valid UTF8.
            write!(
                f,
                "{}",
                std::string::String::from_utf8(
                    slice::from_raw_parts(
                        data.assume_init() as _,
                        size.assume_init() as _,
                    )
                    .to_vec(),
                )
                .map_err(|_| std::fmt::Error)?
            )
        }
    }
}

impl<'a> Drop for StringView<'a> {
    fn drop(&mut self) {
        unsafe { oiio_StringView_dtor(self.ptr) };
    }
}

impl<'a> StringView<'a> {
    pub(crate) fn as_raw_ptr(&self) -> *const oiio_StringView_t {
        self.ptr as _
    }

    pub(crate) fn as_raw_ptr_mut(&self) -> *mut oiio_StringView_t {
        self.ptr
    }
}
