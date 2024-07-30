use std::ops::Range;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct RegionOfInterest {
    pub x: Range<i32>,
    pub y: Range<i32>,
    pub z: Range<i32>,
    pub channel: Range<u32>,
}

impl Default for RegionOfInterest {
    fn default() -> Self {
        Self {
            x: i32::MIN..0,
            y: 0..0,
            z: 0..0,
            channel: 0..0,
        }
    }
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
