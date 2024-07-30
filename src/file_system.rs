use crate::*;
use std::mem::MaybeUninit;

pub enum Mode {
    Closed,
    Read,
    Write,
}

pub struct IoProxy {
    ptr: *mut oiio_IOProxy_t,
}

impl IoProxy {
    pub fn new(file_name: &Utf8Path, mode: Mode) -> Self {
        let ptr = MaybeUninit::<*mut oiio_IOProxy_t>::uninit();

        unsafe {
            /*oiio_IoProxy_ctor(
                StringView::from(file_name).as_raw_ptr(),
                mode as _,
                &mut ptr as *mut _ as *mut _,
            );

            Self {
                ptr: ptr.assume_init(),
            }*/
            Self {
                ptr: std::ptr::null_mut(),
            }
        }
    }
}
