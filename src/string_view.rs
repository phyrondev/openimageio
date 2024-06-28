use crate::*;
use std::{marker::PhantomData, path::Path};

pub struct StringView<'a> {
    ptr: *mut oiio_StringView_t,
    // _marker needs to be invariant in 'a.
    // See "Making a struct outlive a parameter given to a method of
    // that struct": https://stackoverflow.com/questions/62374326/
    _marker: PhantomData<*mut &'a ()>,
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
