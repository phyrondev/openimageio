use crate::*;
use anyhow::Result;
use core::{
    mem::{transmute, MaybeUninit},
    ops::{Div, Mul, Range},
};
use num_traits::AsPrimitive;

pub(crate) static ALL: Bounds = Bounds {
    x: i32::MIN..0,
    y: 0..0,
    z: 0..0,
    channel: 0..0,
};

/// Describes a specific rectangular/cubic region in an image or the region of
/// the whole image.
///
/// # C++
///
/// The original name was changed to not contain abbreviations and the. The
/// original name, [`Roi`], i.e. "region of interest"" (re-capitalized to
/// conform with RFC 344), is available behind a `type` alias when the
/// `cpp_api_names` feature is enabled.
///
/// [C++ Documentation](https://openimageio.readthedocs.io/en/latest/imageioapi.html#rectangular-region-of-interest-roi).
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum Region {
    /// All of the image, or no region restriction.
    #[default]
    All,
    /// A specific region.
    Bounds(Bounds),
}

/// Convenience type alias for developers familiar with the OpenImageIO C++ API.
#[cfg(feature = "cpp_api_names")]
pub type Roi = Region;

impl Region {
    pub fn bounds(&self) -> Option<&Bounds> {
        match self {
            Region::All => None,
            Region::Bounds(r) => Some(r),
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
            Region::All => {
                // Do nothing.
            }
            Region::Bounds(a) => match b {
                Region::All => *self = b.clone(),
                Region::Bounds(b) => {
                    *self = Region::Bounds(a.union(b).clone());
                }
            },
        }

        self
    }

    pub fn intersection(&mut self, b: &Self) -> &mut Self {
        match self {
            Region::All => *self = b.clone(),
            Region::Bounds(a) => match b {
                Region::All => {
                    // Do nothing.
                }
                Region::Bounds(b) => {
                    *self = Region::Bounds(a.intersection(b).clone());
                }
            },
        }

        self
    }

    /// Scale the region uniformly using the given value.
    ///
    /// This has no effect is the `Region` is of variant `All`.
    pub fn uniform_scale(&mut self, scale: f32) -> &mut Self {
        if let Region::Bounds(r) = self {
            r.uniform_scale(scale);
        }

        self
    }

    /// Transform the region using the given 2D matrix.
    ///
    /// This has no effect is the `Region` is of variant `All`.
    pub fn transform_2d<'a>(&mut self, transform: impl Into<Matrix3Ref<'a, f32>>) -> &mut Self {
        if let Region::Bounds(r) = self {
            r.transform_2d(transform);
        }

        self
    }

    /// Transform the region using the given 3D matrix.
    ///
    /// This has no effect is the `Region` is of variant `All`.
    pub fn transform_3d<'a>(&mut self, transform: impl Into<Matrix4Ref<'a, f32>>) -> &mut Self {
        if let Region::Bounds(r) = self {
            r.transform_3d(transform);
        }

        self
    }
}

impl Region {
    pub(crate) fn _as_raw_ptr(&self) -> *const oiio_ROI_t {
        match self {
            Region::All => &ALL as *const Bounds as _,
            Region::Bounds(r) => r as *const Bounds as _,
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
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct Bounds {
    x: Range<i32>,
    y: Range<i32>,
    z: Range<i32>,
    channel: Range<u32>,
}

impl Default for Bounds {
    fn default() -> Self {
        Self::new(0..0, 0..0, 0..0, Some(0..10000))
    }
}

impl Bounds {
    pub fn new(x: Range<i32>, y: Range<i32>, z: Range<i32>, channel: Option<Range<u32>>) -> Self {
        let channel = channel.unwrap_or(0..4);

        assert!(x.start <= x.end);
        assert!(y.start <= y.end);
        assert!(z.start <= z.end);
        assert!(channel.start < channel.end);

        Self { x, y, z, channel }
    }

    pub fn new_2d(x: Range<i32>, y: Range<i32>) -> Self {
        Self::new(x, y, 0..1, Some(0..4))
    }

    pub fn new_3d(x: Range<i32>, y: Range<i32>, z: Range<i32>) -> Self {
        Self::new(x, y, z, Some(0..4))
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
impl Bounds {
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

impl<T: AsPrimitive<f32>> Mul<T> for Bounds {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: (self.x.start as f32 * rhs.as_()) as _..(self.x.end as f32 * rhs.as_()) as _,
            y: (self.y.start as f32 * rhs.as_()) as _..(self.y.end as f32 * rhs.as_()) as _,
            z: (self.z.start as f32 * rhs.as_()) as _..(self.z.end as f32 * rhs.as_()) as _,
            channel: self.channel.start..self.channel.end,
        }
    }
}

impl<T: AsPrimitive<f32>> Div<T> for Bounds {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: (self.x.start as f32 / rhs.as_()) as _..(self.x.end as f32 / rhs.as_()) as _,
            y: (self.y.start as f32 / rhs.as_()) as _..(self.y.end as f32 / rhs.as_()) as _,
            z: (self.z.start as f32 / rhs.as_()) as _..(self.z.end as f32 / rhs.as_()) as _,
            channel: self.channel.start..self.channel.end,
        }
    }
}

/// # Setters
impl Bounds {
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

    /// Transform the region by the given 3×3 matrix and return the bounds
    /// of the transformed region.
    pub fn transform_2d<'a>(&mut self, transform: impl Into<Matrix3Ref<'a, f32>>) -> &mut Self {
        use nalgebra::{Matrix3, Point2};

        let transform: Matrix3Ref<'a, _> = transform.into();
        let transform: &Matrix3<f32> = transform.into();

        let corners = [
            transform.transform_point(&Point2::<f32>::new(self.x.start as _, self.y.start as _)),
            transform.transform_point(&Point2::<f32>::new(self.x.start as _, self.y.end as _)),
            transform.transform_point(&Point2::<f32>::new(self.x.end as _, self.y.start as _)),
            transform.transform_point(&Point2::<f32>::new(self.x.end as _, self.y.end as _)),
        ];

        self.x.start =
            corners.iter().fold(
                i32::MAX,
                |acc, p| if (p.x as i32) < acc { p.x as i32 } else { acc },
            );

        self.x.end =
            corners.iter().fold(
                i32::MIN,
                |acc, p| if (p.x as i32) > acc { p.x as i32 } else { acc },
            ) + 1;

        self.y.start =
            corners.iter().fold(
                i32::MAX,
                |acc, p| if (p.y as i32) < acc { p.y as i32 } else { acc },
            );

        self.y.end =
            corners.iter().fold(
                i32::MIN,
                |acc, p| if (p.y as i32) > acc { p.y as i32 } else { acc },
            ) + 1;

        self
    }

    /// Transform the region by the given 4×4 matrix and return the bounds of
    /// the transformed region.
    pub fn transform_3d<'a>(&mut self, transform: impl Into<Matrix4Ref<'a, f32>>) -> &mut Self {
        use nalgebra::{Matrix4, Point3};

        let transform: Matrix4Ref<'a, _> = transform.into();
        let transform: &Matrix4<f32> = transform.into();

        let corners = [
            transform.transform_point(&Point3::<f32>::new(
                self.x.start as _,
                self.y.start as _,
                self.z.start as _,
            )),
            transform.transform_point(&Point3::<f32>::new(
                self.x.start as _,
                self.y.start as _,
                self.z.end as _,
            )),
            transform.transform_point(&Point3::<f32>::new(
                self.x.start as _,
                self.y.end as _,
                self.z.end as _,
            )),
            transform.transform_point(&Point3::<f32>::new(
                self.x.end as _,
                self.y.end as _,
                self.z.end as _,
            )),
            transform.transform_point(&Point3::<f32>::new(
                self.x.end as _,
                self.y.end as _,
                self.z.start as _,
            )),
            transform.transform_point(&Point3::<f32>::new(
                self.x.end as _,
                self.y.start as _,
                self.z.start as _,
            )),
        ];

        self.x.start =
            corners.iter().fold(
                i32::MAX,
                |acc, p| if (p.x as i32) < acc { p.x as i32 } else { acc },
            );

        self.x.end =
            corners.iter().fold(
                i32::MIN,
                |acc, p| if (p.x as i32) > acc { p.x as i32 } else { acc },
            ) + 1;

        self.y.start =
            corners.iter().fold(
                i32::MAX,
                |acc, p| if (p.y as i32) < acc { p.y as i32 } else { acc },
            );

        self.y.end =
            corners.iter().fold(
                i32::MIN,
                |acc, p| if (p.y as i32) > acc { p.y as i32 } else { acc },
            ) + 1;

        self.z.start =
            corners.iter().fold(
                i32::MAX,
                |acc, p| if (p.z as i32) < acc { p.z as i32 } else { acc },
            );

        self.z.end =
            corners.iter().fold(
                i32::MIN,
                |acc, p| if (p.z as i32) > acc { p.z as i32 } else { acc },
            ) + 1;

        self
    }
}

/// # Predicates
impl Bounds {
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
    /// [C++ version](https://openimageio.readthedocs.io/en/latest/imageioapi.html#_CPPv4NK4OIIO3ROI8containsERK3ROI)
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
impl Bounds {
    /// Returns `true` if the region is defined.
    ///
    /// This is equivalent to [`!is_empty()`](Region::is_empty).
    pub fn defined(&self) -> bool {
        !self.is_empty()
    }
}

/// # Operations
impl Bounds {
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

impl From<Bounds> for Region {
    fn from(src: Bounds) -> Region {
        Region::Bounds(src)
    }
}

mod internal {
    use super::*;

    impl From<oiio_ROI_t> for Bounds {
        fn from(src: oiio_ROI_t) -> Bounds {
            unsafe { transmute(src) }
        }
    }

    impl From<Bounds> for oiio_ROI_t {
        fn from(src: Bounds) -> Self {
            unsafe { transmute(src) }
        }
    }

    impl From<&Bounds> for *const oiio_ROI_t {
        fn from(src: &Bounds) -> Self {
            src as *const _ as _
        }
    }

    impl TryFrom<*const oiio_ROI_t> for Region {
        type Error = ();

        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        fn try_from(src: *const oiio_ROI_t) -> Result<Self, ()> {
            match unsafe { src.as_ref() } {
                None => Err(()),
                Some(roi) => Ok(if i32::MIN == roi.xbegin {
                    Region::All
                } else {
                    Region::Bounds({
                        let mut dst = MaybeUninit::<Bounds>::uninit();

                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                src as *const Bounds,
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

    impl From<oiio_ROI_t> for Region {
        fn from(src: oiio_ROI_t) -> Region {
            if i32::MIN == src.xbegin {
                Region::All
            } else {
                Region::Bounds(src.into())
            }
        }
    }

    impl From<Region> for oiio_ROI_t {
        fn from(src: Region) -> oiio_ROI_t {
            match src {
                Region::All => unsafe { transmute::<region::Bounds, oiio_ROI_t>(ALL.clone()) },
                Region::Bounds(r) => unsafe { transmute::<region::Bounds, oiio_ROI_t>(r) },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region() {
        let region = Region::Bounds(Bounds::new_3d(3..42, 5..16, -33..9));

        assert_eq!(
            oiio_ROI_t {
                xbegin: 3,
                xend: 42,
                ybegin: 5,
                yend: 16,
                zbegin: -33,
                zend: 9,
                chbegin: 0,
                chend: 4,
            },
            oiio_ROI_t::from(region)
        );
    }
}
