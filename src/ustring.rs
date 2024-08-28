use crate::*;

pub struct Ustring {
    ptr: *mut oiio_UString_t,
}

impl Ustring {
    pub fn new(s: &str) -> Ustring {
        let mut ptr = std::ptr::null_mut();
        unsafe {
            oiio_UString_new(&mut ptr, s.as_ptr() as *const i8);
        }
        Ustring { ptr }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let s = oiio_UString_get_string(self.ptr);
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                s as *const u8,
                self.len(),
            ))
        }
    }

    pub fn len(&self) -> usize {
        unsafe { oiio_UString_get_length(self.ptr) as usize }
    }
}

impl Drop for Ustring {
    fn drop(&mut self) {
        unsafe {
            oiio_UString_free(self.ptr);
        }
    }
}
