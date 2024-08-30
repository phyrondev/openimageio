use crate::*;
use core::mem::MaybeUninit;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(
    Debug, Hash, PartialEq, Eq, Clone, Copy, IntoPrimitive, TryFromPrimitive,
)]
#[repr(u32)]
pub enum IoProxyMode {
    Closed = oiio_Mode::oiio_Mode_Closed.0 as _,
    Read = oiio_Mode::oiio_Mode_Read.0 as _,
    Write = oiio_Mode::oiio_Mode_Write.0 as _,
}

impl From<IoProxyMode> for oiio_Mode {
    fn from(m: IoProxyMode) -> Self {
        unsafe { std::mem::transmute(m) }
    }
}

pub enum IoProxy {
    File(IoFile),
    MemoryWriter,
    MemoryReader,
}

pub struct IoFile {
    pub(crate) ptr: *mut oiio_IOFile_t,
}

impl IoFile {
    pub fn new(file_name: &Utf8Path, mode: IoProxyMode) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_IOFile_t>::uninit();

        unsafe {
            oiio_IOFile_ctor(
                StringView::from(file_name).ptr,
                mode.into(),
                &mut ptr as *mut _ as *mut _,
            );

            Self {
                ptr: ptr.assume_init(),
            }
        }
    }
}

impl Drop for IoFile {
    fn drop(&mut self) {
        unsafe { oiio_IOFile_dtor(self.ptr) };
    }
}
