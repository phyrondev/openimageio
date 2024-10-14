use crate::*;
use core::mem::MaybeUninit;
use num_enum::IntoPrimitive;
use std::sync::Arc;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, IntoPrimitive)]
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

/// Proxy for I/O.
///
/// This provides a simplified interface for file I/O that can have custom
/// overrides.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IoProxy {
    File(IoFile),
    MemoryWriter,
    MemoryReader,
}

/// [`IoProxy`] variant for reading or writing (but not both).
///
/// This wraps a C `stdio` `FILE`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IoFile {
    ptr: Arc<*mut oiio_IOFile_t>,
}

impl IoFile {
    pub fn new(file_name: &Utf8Path, mode: IoProxyMode) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_IOFile_t>::uninit();

        unsafe {
            oiio_IOFile_ctor(
                StringView::from(file_name).as_raw_ptr() as _,
                mode.into(),
                &mut ptr as *mut _ as *mut _,
            );

            Self {
                ptr: Arc::new(ptr.assume_init()),
            }
        }
    }
}

impl Drop for IoFile {
    fn drop(&mut self) {
        if 1 == Arc::<*mut oiio_IOFile_t>::strong_count(&self.ptr) {
            unsafe { oiio_IOFile_dtor(*self.ptr) };
        }
    }
}
