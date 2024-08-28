use crate::*;
use std::mem::MaybeUninit;

pub enum IoProxyMode {
    Closed,
    Read,
    Write,
}

pub enum IoProxy {
    File(IoFile),
    MemoryWriter,
    MemoryReader,
}

pub struct IoFile {
    ptr: *mut oiio_IOFile_t,
}

impl IoFile {
    pub fn new(file_name: &Utf8Path, mode: IoProxyMode) -> Self {
        let ptr = MaybeUninit::<*mut oiio_IOProxy_t>::uninit();

        unsafe {
            /*
            oiio_IoFile_ctor(
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
