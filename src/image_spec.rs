use crate::*;
use std::mem::MaybeUninit;

enum ChannelFormat {
    Uniform(BaseType),
    PerChannel(Vec<BaseType>),
}

/// Holds data that are required to describe nearly any image, and an extensible
/// list of arbitrary attributes that can hold metadata that may be user-defined
/// or specific to individual file formats.
///
/// This is holds data on the Rust side
///
/// Describes the data format of an image -- dimensions, layout,
/// number and meanings of image channels.
///
/// The `width`, height` & `depth` are the size of the data of this image, i.e.,
/// the number of pixels in each dimension.  A `depth` greater than 1
/// indicates a 3D 'volumetric' image.
///
/// The `x`, `y` & `z` fields indicate the *origin* of the pixel data of the image. These default to `0`, but
/// setting them differently may indicate that this image is offset from the
/// usual origin.
///
/// Therefore the pixel data are defined over pixel coordinates
///    `[x .. x + width - 1]` horizontally,
///    `[y .. y + height - 1]` vertically,
///    and `[z .. z + depth - 1]` in depth.
///
/// The analogous `full_width`, `full_height`, `full_depth` and `full_x`,
/// `full_y` & `full_z` fields define a *full* or *display* image window over
/// the region `[full_x .. full_x + full_width - 1]`` horizontally, `[full_y .. full_y + full_height - 1]` vertically, and
/// `[full_z .. full_z + full_depth  -1]` in depth.
///
/// Having the full display window different from the pixel data window can
/// be helpful in cases where you want to indicate that your image is a
/// *crop window* of a larger image (if the pixel data window is a subset of
/// the full display window), or that the pixels include *overscan* (if the
/// pixel data is a superset of the full display window), or may simply
/// indicate how different non-overlapping images piece together.
///
/// For tiled images, `tile_width`, `tile_height`, and `tile_depth` specify
/// that the image is stored in a file organized into rectangular *tiles*
/// of these dimensions. The default value of `0` for these fields indicates
/// that the image is stored in scanline order, rather than as tiles.
pub struct ImageSpecConfig {
    /// Origin (upper left corner) of pixel data.
    x: i32,
    /// Origin (upper left corner) of pixel data.
    y: i32,
    /// Origin (upper left corner) of pixel data.
    z: i32,
    /// Width of the pixel data window.
    width: u32,
    /// Height of the pixel data window.
    height: u32,
    /// Depth of pixel data, >1 indicates a "volume".
    depth: u32,
    /// Origin of the full (display) window.
    full_x: i32,
    /// Origin of the full (display) window.
    full_y: i32,
    /// Origin of the full (display) window.
    full_z: i32,
    /// Width of the full (display) window.
    full_width: u32,
    /// Height of the full (display) window.
    full_height: u32,
    /// Depth of the full (display) window.
    full_depth: u32,
    /// Tile width (0 for a non-tiled image).
    tile_width: u32,
    /// Tile height (0 for a non-tiled image).
    tile_height: u32,
    /// Tile depth (0 for a non-tiled image, 1 for a non-volume image).
    tile_depth: u32,
    /// Data format of the channels.
    ///
    /// Describes the native format of the pixel data values themselves, as a
    /// [`BaseType`]. Typical values would be [`BaseType::U8`] for 8-bit
    /// unsigned values, [`BaseType::F32`] for 32-bit floating-point
    /// values, etc.
    /// If all channels of the image have the same data format, that will be
    /// described by channel_format[0].
    ///
    /// If there are different data formats for each channel, they will be
    /// described in the `channel_formats` `Vec`, and the `format` field
    /// will indicate a single default data format for applications that
    /// don't wish to support per-channel formats (usually this will be the
    /// format of the channel that has the most precision).
    channel_format: Vec<TypeDesc>,
    /// The names of each channel, in order. Typically this will be `"R"`,
    /// `"G"`, `"B"`, `"A"` (alpha), `"Z"` (depth), or other arbitrary names.
    channel_names: Vec<String>,
    /// The index of the channel that represents *alpha* (pixel coverage and/or
    /// transparency).
    ///
    /// It defaults to `None` if no alpha channel is present, or if it is not
    /// known which channel represents alpha.
    alpha_channel_index: Option<u32>,
    /// The index of the channel that represents *z* or *depth* (from the
    /// camera).
    ///
    /// It defaults to `None` if no depth channel is present, or if it is not
    /// know which channel represents depth.
    z_channel_index: Option<u32>,

    /// If the image contains deep data.
    ///
    /// If `true`, this indicates that the image describes contains 'deep' data
    /// consisting of multiple samples per pixel. If  `false`, it's an
    /// ordinary image with one data value (per channel) per pixel.
    deep: bool,
    /*
    /// A list of arbitrarily-named and arbitrarily-typed additional attributes
    /// of the image, for any metadata not described by the hard-coded
    /// fields described above.  This list may be manipulated with the
    /// `attribute()` and `find_attribute()` methods.
    extra_attribs: ParamValueList,*/
}

struct ImageSpec {
    ptr: *mut oiio_ImageSpec_t,
}

impl From<ImageSpecConfig> for ImageSpec {
    fn from(i: ImageSpecConfig) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_ImageSpec_t>::uninit();

        unsafe {
            oiio_ImageSpec_new(
                (&TypeDesc::default()).into(),
                &mut ptr as *mut _ as *mut _,
            );

            let ptr = ptr.assume_init();

            // TODO: initalize all fields of the `oiio_ImageSpec_t`.

            Self { ptr }
        }
    }
}

impl ImageSpec {
    pub fn new() -> Self {
        Self::with_type_desc(&TypeDesc::default())
    }

    pub fn with_type_desc(type_desc: &TypeDesc) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_ImageSpec_t>::uninit();

        unsafe {
            oiio_ImageSpec_new(type_desc.into(), &mut ptr as *mut _ as *mut _);

            Self {
                ptr: ptr.assume_init(),
            }
        }
    }

    pub fn with_dimensions_and_type_desc(
        x_res: u32,
        y_res: u32,
        channel_count: u32,
        type_desc: &TypeDesc,
    ) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_ImageSpec_t>::uninit();

        unsafe {
            oiio_ImageSpec_with_dimensions(
                x_res as _,
                y_res as _,
                channel_count as _,
                type_desc.into(),
                &mut ptr as *mut _ as *mut _,
            );

            Self {
                ptr: ptr.assume_init(),
            }
        }
    }
}

impl ImageSpec {
    fn as_raw_ptr(&self) -> *const oiio_ImageSpec_t {
        self.ptr
    }

    fn as_raw_ptr_mut(&self) -> *mut oiio_ImageSpec_t {
        self.ptr
    }
}

impl Drop for ImageSpec {
    fn drop(&mut self) {
        unsafe { oiio_ImageSpec_dtor(self.as_raw_ptr_mut()) };
    }
}
