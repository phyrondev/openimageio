use crate::*;
use camino::Utf8Path;
use core::{ffi::c_char, marker::PhantomData, mem::MaybeUninit, slice};
use std::{
    fmt::{Display, Formatter},
    path::Path,
};
use ustr::Ustr;

pub struct StringView<'a> {
    pub(crate) ptr: *mut oiio_StringView_t,
    // _marker needs to be invariant in 'a.
    // See "Making a struct outlive a parameter given to a method of
    // that struct": https://stackoverflow.com/questions/62374326/
    _marker: PhantomData<*mut &'a ()>,
}

impl StringView<'_> {
    pub fn is_empty(&self) -> bool {
        let mut is_empty = std::mem::MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_StringView_empty(self.ptr, &mut is_empty as *mut _ as _);

            is_empty.assume_init()
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        let mut len = std::mem::MaybeUninit::<usize>::uninit();

        unsafe {
            oiio_StringView_length(self.ptr, &mut len as *mut _ as _);

            len.assume_init()
        }
    }
}

impl Default for StringView<'_> {
    fn default() -> Self {
        let mut ptr = std::mem::MaybeUninit::<*mut oiio_StringView_t>::uninit();
        unsafe {
            oiio_StringView_default(&raw mut ptr as _);

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
                string.as_ptr() as _,
                string.len().try_into().unwrap(),
                &raw mut ptr as _,
            );

            Self {
                ptr: ptr.assume_init(),
                _marker: PhantomData,
            }
        }
    }
}

impl From<Ustr> for StringView<'static> {
    fn from(string: Ustr) -> Self {
        let mut ptr = std::mem::MaybeUninit::<*mut oiio_StringView_t>::uninit();
        unsafe {
            oiio_StringView_ctor(
                string.as_char_ptr(),
                string.len().try_into().unwrap(),
                &raw mut ptr as _,
            );

            Self {
                ptr: ptr.assume_init(),
                _marker: PhantomData,
            }
        }
    }
}

impl<'a> From<&'a Ustr> for StringView<'a> {
    fn from(string: &'a Ustr) -> Self {
        let mut ptr = std::mem::MaybeUninit::<*mut oiio_StringView_t>::uninit();
        unsafe {
            oiio_StringView_ctor(
                string.as_char_ptr(),
                string.len().try_into().unwrap(),
                &raw mut ptr as _,
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
                os_str_path.as_encoded_bytes().as_ptr() as _,
                os_str_path.len().try_into().unwrap(),
                &raw mut ptr as _,
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
                str_path.as_ptr() as _,
                str_path.len().try_into().unwrap(),
                &raw mut ptr as _,
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
            oiio_StringView_data(self.ptr, &mut data as *mut _ as _);
            oiio_StringView_size(self.ptr, &mut size as *mut _ as _);

            // TODO: check with lg that a OIIO::String is guranteed to always be
            // valid UTF8.
            write!(
                f,
                "{}",
                std::string::String::from_utf8(
                    slice::from_raw_parts(data.assume_init() as _, size.assume_init() as _,)
                        .to_vec(),
                )
                .map_err(|_| std::fmt::Error)?
            )
        }
    }
}

impl Drop for StringView<'_> {
    fn drop(&mut self) {
        unsafe { oiio_StringView_dtor(self.ptr) };
    }
}

impl StringView<'_> {
    pub(crate) fn as_raw_ptr(&self) -> *const oiio_StringView_t {
        self.ptr as _
    }

    pub(crate) fn _as_raw_ptr_mut(&mut self) -> *mut oiio_StringView_t {
        self.ptr
    }
}
