use crate::*;
use anyhow::Result;
use core::{
    mem::{MaybeUninit, transmute},
    ops::Range,
};

pub(crate) static ALL: Region = Region {
    x: i32::MIN..0,
    y: 0..0,
    z: 0..0,
    channel: 0..0,
};

/// Describes a specific rectangular/cubic region of interest in an image or the
/// region of the whole image.
///
/// # C++
///
/// The name was changed to not contain abbreviations. The original name,
/// [`Roi`] (re-capitalized to conform with RFC 344), is available behind a
/// `type` alias when the `cpp_api_names` feature is enabled.
///
/// [C++ Documentation](https://openimageio.readthedocs.io/en/latest/imageioapi.html#rectangular-region-of-interest-roi
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum RegionOfInterest {
    /// 'All' of the image, or no region restriction.
    #[default]
    All,
    /// A specific region of interest.
    Region(Region),
}

/// Convenience type alias for developers familiar with the OpenImageIO C++ API.
#[cfg(feature = "cpp_api_names")]
pub type Roi = RegionOfInterest;

impl RegionOfInterest {
    pub fn region(&self) -> Option<&Region> {
        match self {
            RegionOfInterest::All => None,
            RegionOfInterest::Region(r) => Some(r),
        }
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

    pub fn union(&mut self, b: &Self) -> &mut Self {
        match self {
            RegionOfInterest::All => {
                // Do nothing.
            }
            RegionOfInterest::Region(a) => match b {
                RegionOfInterest::All => *self = b.clone(),
                RegionOfInterest::Region(b) => {
                    *self = RegionOfInterest::Region(a.union(b).clone());
                }
            },
        }

        self
    }

    pub fn intersection(&mut self, b: &Self) -> &mut Self {
        match self {
            RegionOfInterest::All => *self = b.clone(),
            RegionOfInterest::Region(a) => match b {
                RegionOfInterest::All => {
                    // Do nothing.
                }
                RegionOfInterest::Region(b) => {
                    *self = RegionOfInterest::Region(a.intersection(b).clone());
                }
            },
        }

        self
    }

    pub fn uniform_scale(&mut self, scale: f32) -> &mut Self {
        if let RegionOfInterest::Region(r) = self {
            r.uniform_scale(scale);
        }

        self
    }
}

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
    pub fn new(x: Range<i32>, y: Range<i32>, z: Range<i32>, channel: Option<Range<u32>>) -> Self {
        let channel = channel.unwrap_or(0..4);

        assert!(x.start <= x.end);
        assert!(y.start <= y.end);
        assert!(z.start <= z.end);
        assert!(channel.start < channel.end);

        Self { x, y, z, channel }
    }

    pub fn new_2d(x: Range<i32>, y: Range<i32>) -> Self {
        Self::new(x, y, 0..1, None)
    }

    pub fn new_3d(x: Range<i32>, y: Range<i32>, z: Range<i32>) -> Self {
        Self::new(x, y, z, None)
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

    /// Calculate the center of the region.
    pub fn center(&self) -> (f32, f32, f32) {
        (
            self.x.start as f32 + self.width() as f32 / 2.0,
            self.y.start as f32 + self.height() as f32 / 2.0,
            self.z.start as f32 + self.depth() as f32 / 2.0,
        )
    }

    /// Number of channels in the region.
    ///
    /// Beware -- this defaults to a huge number. To be meaningful you must
    /// consider:
    ///
    /// ```ignore
    /// let actual_channels = image_buf.channel_count().min(roi.channel_count());
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

    pub fn uniform_scale(&mut self, scale: f32) -> &mut Self {
        self.x = (self.x.start as f32 * scale) as _..(self.x.end as f32 * scale) as _;
        self.y = (self.y.start as f32 * scale) as _..(self.y.end as f32 * scale) as _;

        self
    }
}

/// # Predicates
impl Region {
    /// Returns `true` if the given point is contained in the region.
    ///
    /// # For C++ Developers
    ///
    /// [The (overloaded) C++ version](https://openimageio.readthedocs.io/en/latest/imageioapi.html#_CPPv4NK4OIIO3ROI8containsEiiii)
    /// of this is called `contains()`.
    #[inline]
    pub fn contains_point(&self, x: i32, y: i32, z: Option<i32>, channel: Option<u32>) -> bool {
        let z = z.unwrap_or(0);
        let channel = channel.unwrap_or(0);

        self.x.contains(&x)
            && self.y.contains(&y)
            && self.z.contains(&z)
            && self.channel.contains(&channel)
    }

    /// Returns `true` if the given `Region` is contained in this region.
    ///
    /// # For C++ Developers
    ///
    /// [The (overloaded) C++ version](https://openimageio.readthedocs.io/en/latest/imageioapi.html#_CPPv4NK4OIIO3ROI8containsERK3ROI)
    /// of this is called `contains()`.
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

    /// Returns `true` if the region is empty.
    ///
    /// For C++ Developers
    ///
    /// [The C++ version](https://openimageio.readthedocs.io/en/latest/imageioapi.html#_CPPv4NK4OIIO3ROI7definedEv)
    /// of this is called [`defined()`](Region::defined).
    pub fn is_empty(&self) -> bool {
        0 == self.width() || 0 == self.height() || 0 == self.depth()
    }
}

#[cfg(feature = "cpp_api_names")]
impl Region {
    /// Returns `true` if the region is defined.
    ///
    /// This is equivalent to [`!is_empty()`](Region::is_empty).
    pub fn defined(&self) -> bool {
        !self.is_empty()
    }
}

/// # Operations
impl Region {
    /// Union of two regions.
    ///
    /// The smallest region containing both.
    pub fn union(&mut self, b: &Self) -> &mut Self {
        if !b.is_empty() {
            if !self.is_empty() {
                self.x = self.x.start.min(b.x.start)..self.x.end.max(b.x.end);
                self.y = self.y.start.min(b.y.start)..self.y.end.max(b.y.end);
                self.z = self.z.start.min(b.z.start)..self.z.end.max(b.z.end);
                self.channel =
                    self.channel.start.min(b.channel.start)..self.channel.end.max(b.channel.end);
            }
        } else {
            *self = b.clone();
        }

        self
    }

    /// Intersection of two regions.
    pub fn intersection(&mut self, b: &Self) -> &mut Self {
        if !b.is_empty() {
            if !self.is_empty() {
                self.x = self.x.start.max(b.x.start)..self.x.end.min(b.x.end);
                self.y = self.y.start.max(b.y.start)..self.y.end.min(b.y.end);
                self.z = self.z.start.max(b.z.start)..self.z.end.min(b.z.end);
                self.channel =
                    self.channel.start.max(b.channel.start)..self.channel.end.min(b.channel.end);
            }
        } else {
            *self = b.clone();
        }

        self
    }
}

impl From<Region> for RegionOfInterest {
    fn from(src: Region) -> RegionOfInterest {
        RegionOfInterest::Region(src)
    }
}

mod internal {
    use super::*;

    impl From<oiio_ROI_t> for Region {
        fn from(src: oiio_ROI_t) -> Region {
            unsafe { transmute(src) }
        }
    }

    impl From<Region> for oiio_ROI_t {
        fn from(src: Region) -> Self {
            unsafe { transmute(src) }
        }
    }

    impl From<&Region> for *const oiio_ROI_t {
        fn from(src: &Region) -> Self {
            src as *const _ as _
        }
    }

    impl TryFrom<*const oiio_ROI_t> for RegionOfInterest {
        type Error = ();

        fn try_from(src: *const oiio_ROI_t) -> Result<Self, ()> {
            match unsafe { src.as_ref() } {
                None => Err(()),
                Some(roi) => Ok(if i32::MIN == roi.xbegin {
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
                }),
            }
        }
    }

    impl From<oiio_ROI_t> for RegionOfInterest {
        fn from(src: oiio_ROI_t) -> RegionOfInterest {
            if i32::MIN == src.xbegin {
                RegionOfInterest::All
            } else {
                RegionOfInterest::Region(src.into())
            }
        }
    }

    impl From<RegionOfInterest> for oiio_ROI_t {
        fn from(src: RegionOfInterest) -> oiio_ROI_t {
            match src {
                RegionOfInterest::All => unsafe {
                    transmute::<region_of_interest::Region, oiio_ROI_t>(ALL.clone())
                },
                RegionOfInterest::Region(r) => unsafe {
                    transmute::<region_of_interest::Region, oiio_ROI_t>(r)
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_of_interest() {
        let region = RegionOfInterest::Region(Region::new_3d(3..42, 5..16, -33..9));

        assert_eq!(
            oiio_ROI_t {
                xbegin: 3,
                xend: 42,
                ybegin: 5,
                yend: 16,
                zbegin: -33,
                zend: 9,
                chbegin: 0,
                chend: 10000,
            },
            oiio_ROI_t::from(region)
        );
    }
}
