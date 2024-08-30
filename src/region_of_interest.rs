use crate::*;
use core::{
    i32,
    mem::{transmute, MaybeUninit},
    ops::Range,
};

static ALL: Region = Region {
    x: i32::MIN..0,
    y: 0..0,
    z: 0..0,
    channel: 0..0,
};

/// Describes a region of interest in an image.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum RegionOfInterest {
    /// 'All' of the image, or no region restriction.
    #[default]
    All,
    /// A specific region of interest.
    Region(Region),
}

pub type Roi = RegionOfInterest;

impl RegionOfInterest {
    pub(crate) fn as_raw_ptr(&self) -> *const oiio_ROI_t {
        match self {
            RegionOfInterest::All => &ALL as *const Region as _,
            RegionOfInterest::Region(r) => r as *const Region as _,
        }
    }
}

/// Describes a rectangular/cubic region in an image.
///
/// The region is *[`x.start`, `x.end`) × [`y.start`, `y.end`) × [`z.start`,
/// `z.end`)*, with the `*.end` designators signifying one past the last pixel
/// in each dimension.
///
/// * [Getters](#getters)
/// * [Setters](#setters)
/// * [Predicates](#predicates)
/// * [Operations](#operations)
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct Region {
    x: Range<i32>,
    y: Range<i32>,
    z: Range<i32>,
    channel: Range<u32>,
}

impl Region {
    pub fn new(
        x: Range<i32>,
        y: Range<i32>,
        z: Option<Range<i32>>,
        channel: Option<Range<u32>>,
    ) -> Self {
        let z = z.unwrap_or(0..1);
        let channel = channel.unwrap_or(0..10000);

        debug_assert!(x.start <= x.end);
        debug_assert!(y.start <= y.end);
        debug_assert!(z.start <= z.end);
        debug_assert!(channel.start < channel.end);

        Self { x, y, z, channel }
    }

    pub fn new_2d(x: Range<i32>, y: Range<i32>) -> Self {
        Self::new(x, y, None, None)
    }

    pub fn new_3d(x: Range<i32>, y: Range<i32>, z: Range<i32>) -> Self {
        Self::new(x, y, Some(z), None)
    }

    pub fn all() -> Self {
        Self::default()
    }

    pub fn from_union(a: &Self, b: &Self) -> Self {
        let mut result = a.clone();
        result.union(b);
        result
    }

    pub fn from_intersection(a: &Self, b: &Self) -> Self {
        let mut result = a.clone();
        result.intersection(b);
        result
    }
}

/// # Getters
impl Region {
    /// The width of the region.
    pub fn width(&self) -> u32 {
        debug_assert!(self.x.start < self.x.end);

        (self.x.end - self.x.start) as _
    }

    /// The height of the region.
    pub fn height(&self) -> u32 {
        debug_assert!(self.x.start < self.x.end);

        (self.y.end - self.y.start) as _
    }

    /// The depth of the region.
    pub fn depth(&self) -> u32 {
        debug_assert!(self.z.start < self.z.end);

        (self.z.end - self.z.start) as _
    }

    /// Number of channels in the region.
    ///
    /// Beware -- this defaults to a huge number. To be meaningful you must
    /// consider:
    ///
    /// ```
    /// let actual_channels = imagebuf.channels_count().min(roi.channels_count());
    /// ```
    ///
    /// # C++
    ///
    /// The C++ method is called `nchannels()`.
    pub fn channel_count(&self) -> u32 {
        self.channel.end - self.channel.start
    }

    /// Total number of pixels in the region.
    ///
    /// # C++
    ///
    /// The C++ method is called `npixels()`.
    pub fn pixel_count(&self) -> usize {
        self.width() as usize * self.height() as usize * self.depth() as usize
    }

    pub fn x(&self) -> &Range<i32> {
        &self.x
    }

    pub fn y(&self) -> &Range<i32> {
        &self.y
    }

    pub fn z(&self) -> &Range<i32> {
        &self.z
    }

    pub fn channel(&self) -> &Range<u32> {
        &self.channel
    }

    pub fn x_start(&self) -> i32 {
        self.x.start
    }

    pub fn x_end(&self) -> i32 {
        self.x.end
    }

    pub fn y_start(&self) -> i32 {
        self.y.start
    }

    pub fn y_end(&self) -> i32 {
        self.y.end
    }

    pub fn z_start(&self) -> i32 {
        self.z.start
    }

    pub fn z_end(&self) -> i32 {
        self.z.end
    }

    pub fn channel_start(&self) -> u32 {
        self.channel.start
    }

    pub fn channel_end(&self) -> u32 {
        self.channel.end
    }
}

/// # Setters
impl Region {
    pub fn set_x(&mut self, x: Range<i32>) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: Range<i32>) {
        self.y = y;
    }

    pub fn set_z(&mut self, z: Range<i32>) {
        self.z = z;
    }

    pub fn set_channel(&mut self, channel: Range<u32>) {
        self.channel = channel;
    }

    pub fn set_x_start(&mut self, x: i32) {
        self.x.start = x;
    }

    pub fn set_x_end(&mut self, x: i32) {
        self.x.end = x;
    }

    pub fn set_y_start(&mut self, y: i32) {
        self.y.start = y;
    }

    pub fn set_y_end(&mut self, y: i32) {
        self.y.end = y;
    }

    pub fn set_z_start(&mut self, z: i32) {
        self.z.start = z;
    }

    pub fn set_z_end(&mut self, z: i32) {
        self.z.end = z;
    }

    pub fn set_channel_start(&mut self, channel: u32) {
        self.channel.start = channel;
    }

    pub fn set_channel_end(&mut self, channel: u32) {
        self.channel.end = channel;
    }
}

/// # Predicates
impl Region {
    #[inline]
    pub fn contains_point(
        &self,
        x: i32,
        y: i32,
        z: Option<i32>,
        channel: Option<u32>,
    ) -> bool {
        let z = z.unwrap_or(0);
        let channel = channel.unwrap_or(0);

        x >= self.x.start
            && x < self.x.end
            && y >= self.y.start
            && y < self.y.end
            && z >= self.z.start
            && z < self.z.end
            && channel >= self.channel.start
            && channel < self.channel.end
    }

    #[inline]
    pub fn contains(&self, other: &Self) -> bool {
        other.x.start >= self.x.start
            && other.x.end <= self.x.end
            && other.y.start >= self.y.start
            && other.y.end <= self.y.end
            && other.z.start >= self.z.start
            && other.z.end <= self.z.end
            && other.channel.start >= self.channel.start
            && other.channel.end <= self.channel.end
    }

    pub fn is_empty(&self) -> bool {
        0 != self.pixel_count()
    }
}

/// # Operations
impl Region {
    /// Union of two regions, the smallest region containing both.
    pub fn union(&mut self, b: &Self) {
        if b.is_empty() {
            if self.is_empty() {
                self.x = self.x.start.min(b.x.start)..self.x.end.max(b.x.end);
                self.y = self.y.start.min(b.y.start)..self.y.end.max(b.y.end);
                self.z = self.z.start.min(b.z.start)..self.z.end.max(b.z.end);
                self.channel = self.channel.start.min(b.channel.start)
                    ..self.channel.end.max(b.channel.end);
            }
        } else {
            *self = b.clone();
        }
    }

    /// Intersection of two regions.
    pub fn intersection(&mut self, b: &Self) {
        if b.is_empty() {
            if self.is_empty() {
                self.x = self.x.start.max(b.x.start)..self.x.end.min(b.x.end);
                self.y = self.y.start.max(b.y.start)..self.y.end.min(b.y.end);
                self.z = self.z.start.max(b.z.start)..self.z.end.min(b.z.end);
                self.channel = self.channel.start.max(b.channel.start)
                    ..self.channel.end.min(b.channel.end);
            }
        } else {
            *self = b.clone();
        }
    }
}

impl From<oiio_ROI_t> for Region {
    fn from(src: oiio_ROI_t) -> Region {
        unsafe { transmute(src) }
    }
}

mod internal {
    use super::*;

    impl From<*const oiio_ROI_t> for RegionOfInterest {
        fn from(src: *const oiio_ROI_t) -> RegionOfInterest {
            if i32::MIN == unsafe { src.as_ref().unwrap().xbegin } {
                RegionOfInterest::All
            } else {
                RegionOfInterest::Region({
                    let mut dst = MaybeUninit::<Region>::uninit();

                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            src as *const Region,
                            dst.as_mut_ptr(),
                            1,
                        );

                        dst.assume_init()
                    }
                })
            }
        }
    }

    impl From<RegionOfInterest> for oiio_ROI_t {
        fn from(src: RegionOfInterest) -> oiio_ROI_t {
            match src {
                RegionOfInterest::All => unsafe { transmute(ALL.clone()) },
                RegionOfInterest::Region(r) => unsafe { transmute(r) },
            }
        }
    }
}
