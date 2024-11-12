use crate::*;
use ahash::AHashMap as HashMap;
use float_derive::{FloatEq, FloatHash, FloatPartialEq};
use parking_lot::RwLock;
use std::{mem::MaybeUninit, sync::LazyLock};

// Global 2D pixel filter registry.
// FIXME: This is never freed over the lifetime of the program.
const FILTER_2D_MAP: LazyLock<RwLock<HashMap<Filter2DInfo, Filter2D>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

// This is only used for hashing the filter name & dimensions to lookup the
// FILTER_2D_MAP.
#[derive(FloatPartialEq, FloatEq, FloatHash)]
struct Filter2DInfo {
    name: PixelFilter2D,
    x_width: f32,
    y_width: f32,
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum PixelFilter2D {
    Box,
    Triangle,
    Gaussian,
    SharpGaussian,
    CatmullRom,
    BlackmanHarris,
    Sinc,
    #[default]
    Lanczos3,
    RadialLanczos3,
    NukeLanczos6,
    Mitchell,
    Bspline,
    Disk,
    Cubic,
    Keys,
    Simon,
    Rifman,
    //Binomial,
    //Laplacian,
}

impl From<PixelFilter2D> for &str {
    fn from(pf: PixelFilter2D) -> Self {
        match pf {
            PixelFilter2D::Box => "box",
            PixelFilter2D::Triangle => "triangle",
            PixelFilter2D::Gaussian => "gaussian",
            PixelFilter2D::SharpGaussian => "sharp-gaussian",
            PixelFilter2D::CatmullRom => "catmull-rom",
            PixelFilter2D::BlackmanHarris => "blackman-harris",
            PixelFilter2D::Sinc => "sinc",
            PixelFilter2D::Lanczos3 => "lanczos3",
            PixelFilter2D::RadialLanczos3 => "radial-lanczos3",
            PixelFilter2D::NukeLanczos6 => "nuke-lanczo6",
            PixelFilter2D::Mitchell => "mitchell",
            PixelFilter2D::Bspline => "b-spline",
            PixelFilter2D::Disk => "disk",
            PixelFilter2D::Cubic => "cubic",
            PixelFilter2D::Keys => "keys",
            PixelFilter2D::Simon => "simon",
            PixelFilter2D::Rifman => "rifman",
            //PixelFilter2D::Binomial => "binomial",
            //PixelFilter2D::Laplacian => "laplacian",
        }
    }
}

impl From<&str> for PixelFilter2D {
    fn from(name: &str) -> Self {
        match name {
            "box" => Self::Box,
            "triangle" => Self::Triangle,
            "gaussian" => Self::Gaussian,
            "sharp-gaussian" => Self::SharpGaussian,
            "catmull-rom" => Self::CatmullRom,
            "blackman-harris" => Self::BlackmanHarris,
            "sinc" => Self::Sinc,
            "lanczos3" => Self::Lanczos3,
            "radial-lanczos3" => Self::RadialLanczos3,
            "nuke-lanczos6" => Self::NukeLanczos6,
            "mitchell" => Self::Mitchell,
            "bspline" => Self::Bspline,
            "disk" => Self::Disk,
            "cubic" => Self::Cubic,
            "keys" => Self::Keys,
            "simon" => Self::Simon,
            "rifman" => Self::Rifman,
            //"binomial" => Self::Binomial,
            //"laplacian" => Self::Laplacian,
            _ => panic!("Unknown pixel filter: {}", name),
        }
    }
}

impl From<PixelFilter2D> for Ustr {
    fn from(pf: PixelFilter2D) -> Self {
        ustr(Into::<&str>::into(pf))
    }
}

#[derive(Clone, Copy)]
pub struct Filter2D {
    ptr: *mut oiio_Filter2D_t,
}

impl Filter2D {
    /// Create a new 2D pixel filter.
    ///
    /// The filter is cached and reused if you call new() with the same
    /// parameters.
    pub fn new(name: PixelFilter2D, x_width: f32, y_width: f32) -> Self {
        let filter = Filter2DInfo {
            name,
            x_width,
            y_width,
        };

        if let Some(filter_2d) = FILTER_2D_MAP.read().get(&filter) {
            *filter_2d
        } else {
            let filter_2d = Filter2D::new_ffi(name.into(), x_width, y_width);
            FILTER_2D_MAP.write().insert(filter, filter_2d);

            filter_2d
        }
    }

    /// Clear the global cache of 2D pixel filters.
    pub fn clear_cache() {
        FILTER_2D_MAP.write().retain(|_, filter_2d| {
            unsafe { oiio_Filter2D_destroy(filter_2d.ptr) };
            false
        });
    }
}

impl From<PixelFilter2D> for Filter2D {
    fn from(pf: PixelFilter2D) -> Self {
        match pf {
            PixelFilter2D::Box => Filter2D::new(pf, 1.0, 1.0),
            PixelFilter2D::Triangle => Filter2D::new(pf, 2.0, 2.0),
            PixelFilter2D::Gaussian => Filter2D::new(pf, 3.0, 3.0),
            PixelFilter2D::SharpGaussian => Filter2D::new(pf, 2.0, 2.0),
            PixelFilter2D::CatmullRom => Filter2D::new(pf, 4.0, 4.0),
            PixelFilter2D::BlackmanHarris => Filter2D::new(pf, 3.0, 3.0),
            PixelFilter2D::Sinc => Filter2D::new(pf, 4.0, 4.0),
            PixelFilter2D::Lanczos3 => Filter2D::new(pf, 6.0, 6.0),
            PixelFilter2D::RadialLanczos3 => Filter2D::new(pf, 6.0, 6.0),
            PixelFilter2D::NukeLanczos6 => Filter2D::new(pf, 6.0, 6.0),
            PixelFilter2D::Mitchell => Filter2D::new(pf, 4.0, 4.0),
            PixelFilter2D::Bspline => Filter2D::new(pf, 4.0, 4.0),
            PixelFilter2D::Disk => Filter2D::new(pf, 1.0, 1.0),
            PixelFilter2D::Cubic => Filter2D::new(pf, 1.0, 4.0),
            PixelFilter2D::Keys => Filter2D::new(pf, 4.0, 4.0),
            PixelFilter2D::Simon => Filter2D::new(pf, 4.0, 4.0),
            PixelFilter2D::Rifman => Filter2D::new(pf, 4.0, 4.0),
            //PixelFilter2D::Binomial => Filter2D::new(pf, 1.0, 1.0),
            //PixelFilter2D::Laplacian => Filter2D::new(pf, 3.0, 3.0),
        }
    }
}

impl Filter2D {
    fn new_ffi(name: &str, x_width: f32, y_width: f32) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_Filter2D_t>::uninit();

        unsafe {
            oiio_Filter2D_create(
                StringView::from(name).as_raw_ptr() as _,
                x_width,
                y_width,
                &mut ptr as *mut _ as _,
            );

            Self {
                ptr: ptr.assume_init(),
            }
        }
    }

    pub(crate) fn as_raw_ptr(&self) -> *const oiio_Filter2D_t {
        self.ptr
    }

    pub(crate) fn _as_raw_ptr_mut(&mut self) -> *mut oiio_Filter2D_t {
        self.ptr
    }
}
