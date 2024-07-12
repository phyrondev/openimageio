//use crate::*;
use std::{
    ffi::{c_int, c_uint},
    //mem::MaybeUninit,
    ops::Range,
    //ptr::copy_nonoverlapping,
};

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct RegionOfInterest {
    pub x: Range<c_int>,
    pub y: Range<c_int>,
    pub z: Range<c_int>,
    pub channel: Range<c_uint>,
}

pub type Roi = RegionOfInterest;

/*
impl From<*const oiio_Roi_t> for RegionOfInterest {
    fn from(src: *const oiio_Roi_t) -> RegionOfInterest {
        let mut dst = MaybeUninit::<RegionOfInterest>::uninit();

        unsafe {
            copy_nonoverlapping(
                src as *const RegionOfInterest,
                dst.as_mut_ptr(),
                1,
            );

            dst.assume_init()
        }
    }
}*/

impl RegionOfInterest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn all() -> Self {
        Self::default()
    }
}
