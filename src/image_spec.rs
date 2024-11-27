use crate::*;
use core::mem::MaybeUninit;
use ustr::Ustr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ChannelFormat {
    Uniform(BaseType, usize),
    PerChannel(Vec<BaseType>),
}

impl Default for ChannelFormat {
    fn default() -> Self {
        Self::Uniform(BaseType::default(), 4)
    }
}

impl ChannelFormat {
    pub fn channel_count(&self) -> usize {
        match self {
            Self::Uniform(_, len) => *len,
            Self::PerChannel(channels) => channels.len(),
        }
    }
}

/// Describes the data format of an image -- dimensions, layout, number and
/// meanings of image channels.
///
/// It contains:
///
/// * The image resolution (number of pixels) and origin. This specifies what is
///   often called the “pixel data window.”
///
/// * The full size and offset of an abstract 'full' or 'display' window.
///   Differing full and data windows can indicate that the pixels are a crop
///   region or a larger image, or contain overscan pixels.
///
/// * Whether the image is organized into tiles, and if so, the tile size.
///
/// * The native data format of the pixel values (e.g., float, 8-bit integer,
///   etc.).
///
/// * The number of color channels in the image (e.g., 3 for RGB images), names
///   of the channels, and whether any particular channels represent alpha and
///   depth.
///
/// * A user-extensible (and format-extensible) list of any other
///   arbitrarily-named and -typed data that may help describe the image or its
///   disk representation.
///
/// An `ImageSpecification` can be used to describe nearly any image, and an
/// extensible list of arbitrary attributes that can hold metadata that may be
/// user-defined or specific to individual file formats.
///
/// The `width`, `height` & `depth` are the size of the data of this image,
/// i.e., the number of pixels in each dimension.  A `depth` greater than `1`
/// indicates a 3D 'volumetric' image.
///
/// The `x`, `y` & `z` fields indicate the *origin* of the pixel data of the
/// image. These default to `0`, but setting them differently may indicate that
/// this image is offset from the usual origin.
///
/// Therefore the pixel data are defined over pixel coordinates
///    `[x .. x + width - 1]` horizontally,
///    `[y .. y + height - 1]` vertically,
///    and `[z .. z + depth - 1]` in depth.
///
/// The analogous `full_width`, `full_height`, `full_depth` and `full_x`,
/// `full_y` & `full_z` fields define a *full* or *display* image window over
/// the region `[full_x .. full_x + full_width - 1]` horizontally, `[full_y ..
/// full_y + full_height - 1]` vertically, and `[full_z .. full_z + full_depth -
/// 1]` in depth.
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
///
/// # C++
///
/// The name was changed to not contain abbreviations. The original name,
/// [`ImageSpec`], is available behind a `type` alias.
///
/// [C++ Documentation](https://openimageio.readthedocs.io/en/latest/imageioapi.html#image-specification-imagespec)
#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct ImageSpec {
    /// Origin (upper left corner) of pixel data.
    pub x: i32,
    /// Origin (upper left corner) of pixel data.
    pub y: i32,
    /// Origin (upper left corner) of pixel data.
    pub z: i32,
    /// Width of the pixel data window.
    pub width: u32,
    /// Height of the pixel data window.
    pub height: u32,
    /// Depth of pixel data, >1 indicates a "volume".
    pub depth: u32,
    /// Origin of the full (display) window.
    pub display_window_x: i32,
    /// Origin of the full (display) window.
    pub display_window_y: i32,
    /// Origin of the full (display) window.
    pub display_window_z: i32,
    /// Width of the full (display) window.
    pub display_window_width: u32,
    /// Height of the full (display) window.
    pub display_window_height: u32,
    /// Depth of the full (display) window.
    pub display_window_depth: u32,
    /// Tile width (0 for a non-tiled image).
    pub tile_width: u32,
    /// Tile height (0 for a non-tiled image).
    pub tile_height: u32,
    /// Tile depth (0 for a non-tiled image, 1 for a non-volume image).
    pub tile_depth: u32,
    /// Data format of the channels.
    ///
    /// Describes the native format of the pixel data values themselves, as a
    /// [`BaseType`]. Typical values would be [`BaseType::U8`] for 8-bit
    /// unsigned values, [`BaseType::F32`] for 32-bit floating-point
    /// values, etc.
    ///
    /// If all channels of the image have the same data format, that will be
    /// described by `ChannelFormat::Uniform`.
    ///
    /// If there are different data formats for each channel, they will be
    /// described in the `ChannelFormat::PerChannel` field's `Vec`, and the
    /// `format` will indicate a single default data format for applications
    /// that don't wish to support per-channel formats (usually this will be
    /// the format of the channel that has the most precision).
    pub channel_format: ChannelFormat,
    /// The names of each channel, in order. Typically this will be `"R"`,
    /// `"G"`, `"B"`, `"A"` (alpha), `"Z"` (depth), or other arbitrary names.
    pub channel_name: Vec<Ustr>,
    /// The index of the channel that represents *alpha* (pixel coverage and/or
    /// transparency).
    ///
    /// It defaults to `None` if no alpha channel is present, or if it is not
    /// known which channel represents alpha.
    pub alpha_channel_index: Option<u32>,
    /// The index of the channel that represents *z* or *depth* (from the
    /// camera).
    ///
    /// It defaults to `None` if no depth channel is present, or if it is not
    /// know which channel represents depth.
    pub z_channel_index: Option<u32>,

    /// If the image contains deep data.
    ///
    /// If `true`, this indicates that the image describes contains 'deep' data
    /// consisting of multiple samples per pixel. If  `false`, it's an
    /// ordinary image with one data value (per channel) per pixel.
    pub deep: bool,
    /*
    /// A list of arbitrarily-named and arbitrarily-typed additional attributes
    /// of the image, for any metadata not described by the hard-coded
    /// fields described above.  This list may be manipulated with the
    /// `attribute()` and `find_attribute()` methods.
    extra_attribs: ParamValueList,*/
}

// Convenience type alias for developers familiar with the OpenImageIO C++ API.
//pub type ImageSpec = ImageSpecification;

impl ImageSpec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with(channel_type: BaseType) -> Self {
        Self {
            channel_format: ChannelFormat::Uniform(channel_type, 4),
            ..Default::default()
        }
    }

    pub fn new_with_dimensions(
        width: u32,
        height: u32,
        channel_count: u32,
        channel_type: BaseType,
    ) -> Self {
        Self {
            width,
            height,
            channel_format: ChannelFormat::Uniform(channel_type, channel_count as _),
            ..Default::default()
        }
    }
}

pub(crate) struct ImageSpecInternal {
    ptr: *mut oiio_ImageSpec_t,
}

/*impl From<ImageSpec> for oiio_ImageSpec_t {
    /// This is mainly to pass an `oiio_ImageSpec_t` on the stack to the FFI.
    ///
    /// The C++ side of the FFI will be responsible for freeing the memory.
    fn from(image_spec: ImageSpec) -> Self {
        let ptr = image_spec.ptr;
        std::mem::forget(image_spec);

        *ptr
    }
}*/

impl From<&ImageSpec> for ImageSpecInternal {
    fn from(i: &ImageSpec) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_ImageSpec_t>::uninit();

        unsafe {
            oiio_ImageSpec_new((&TypeDesc::default()).into(), &mut ptr as *mut _ as *mut _);

            let ptr = ptr.assume_init();

            oiio_ImageSpec_set_x(ptr, i.x);
            oiio_ImageSpec_set_y(ptr, i.y);
            oiio_ImageSpec_set_z(ptr, i.z);
            oiio_ImageSpec_set_width(ptr, i.width as _);
            oiio_ImageSpec_set_height(ptr, i.height as _);
            oiio_ImageSpec_set_depth(ptr, i.depth as _);
            oiio_ImageSpec_set_full_x(ptr, i.display_window_x);
            oiio_ImageSpec_set_full_y(ptr, i.display_window_y);
            oiio_ImageSpec_set_full_z(ptr, i.display_window_z);
            oiio_ImageSpec_set_full_width(ptr, i.display_window_width as _);
            oiio_ImageSpec_set_full_height(ptr, i.display_window_height as _);
            oiio_ImageSpec_set_full_depth(ptr, i.display_window_depth as _);
            oiio_ImageSpec_set_tile_width(ptr, i.tile_width as _);
            oiio_ImageSpec_set_tile_height(ptr, i.tile_height as _);
            oiio_ImageSpec_set_tile_depth(ptr, i.tile_depth as _);

            match &i.channel_format {
                ChannelFormat::Uniform(base_type, len) => {
                    oiio_ImageSpec_set_format(
                        ptr,
                        oiio_TypeDesc_t {
                            basetype: (*base_type).into(),
                            ..Default::default()
                        },
                    );
                    oiio_ImageSpec_set_nchannels(ptr, *len as _);
                }
                ChannelFormat::PerChannel(formats) => {
                    oiio_ImageSpec_clear_and_reserve_channelformats(ptr, formats.len());
                    for format in formats.iter() {
                        oiio_ImageSpec_push_channelformat(
                            ptr,
                            oiio_TypeDesc_t {
                                basetype: (*format).into(),
                                ..Default::default()
                            },
                        );
                    }
                }
            }

            oiio_ImageSpec_clear_and_reserve_channelnames(ptr, i.channel_name.len());
            for name in i.channel_name.iter() {
                oiio_ImageSpec_push_channelname(ptr, OiioString::new(name).as_raw_ptr());
            }

            oiio_ImageSpec_set_alpha_channel(
                ptr,
                i.alpha_channel_index.map(|i| i as _).unwrap_or(-1),
            );
            oiio_ImageSpec_set_z_channel(ptr, i.z_channel_index.map(|i| i as _).unwrap_or(-1));
            oiio_ImageSpec_set_deep(ptr, i.deep);

            // TODO: set `extra_attribs`

            Self { ptr }
        }
    }
}

impl ImageSpecInternal {
    pub fn new() -> Self {
        Self::new_with(&TypeDesc::default())
    }

    pub fn new_with(type_desc: &TypeDesc) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_ImageSpec_t>::uninit();

        unsafe {
            oiio_ImageSpec_new(type_desc.into(), &raw mut ptr as _);

            Self {
                ptr: ptr.assume_init(),
            }
        }
    }

    pub fn set_color_space(&mut self, color_space: &str) {
        unsafe {
            oiio_ImageSpec_set_colorspace(
                self.ptr,
                StringView::from(color_space).as_raw_ptr() as _,
            );
        }
    }

    pub fn _new_with_dimensions(
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
                &raw mut ptr as _,
            );

            Self {
                ptr: ptr.assume_init(),
            }
        }
    }

    /*
    pub fn new_with_bounds(bounds: Bounds, type_desc: &TypeDesc) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_ImageSpec_t>::uninit();

        unsafe {
            oiio_ImageSpec_with_dimensions(
                x_res as _,
                y_res as _,
                channel_count as _,
                type_desc.into(),
                &raw mut ptr as _,
            );

            Self {
                ptr: ptr.assume_init(),
            }
        }
    }*/
}

impl ImageSpecInternal {
    pub fn as_raw_ptr(&self) -> *const oiio_ImageSpec_t {
        self.ptr
    }

    pub fn _as_raw_ptr_mut(&self) -> *mut oiio_ImageSpec_t {
        self.ptr
    }
}

impl Default for ImageSpecInternal {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ImageSpecInternal {
    fn drop(&mut self) {
        unsafe { oiio_ImageSpec_dtor(self.ptr) };
    }
}
