use crate::*;

/// Holds many options controlling single-point texture lookups. Because each
/// texture lookup call takes a reference to a `TextureOptions`.
#[derive(Debug, PartialEq)]
pub struct TextureOptions<'a> {
    // We use u16 for some only-ever-positive `i32` values in the FFI struct that can reasonably
    // never even be close to `u16::MAX` in practice to avoid any chance of overflow (and using
    // `try_into()` at the FFI boundary).
    /// First channel of the lookup.
    pub first_channel: u16,
    /// Sub-image or [`Ptex`](https://ptex.us/) face ID.
    pub sub_image: u16,
    /// Sub-image name.
    pub sub_image_name: Ustr,
    /// Wrap mode in the `s` direction.
    pub s_wrap: Wrap,
    /// Wrap mode in the `t` direction.
    pub t_wrap: Wrap,
    /// Wrap mode in the `r` direction for 3D volume texture lookups only.
    pub r_wrap: Wrap,
    /// Mip mode.
    pub mip_mode: MipMode,
    /// Interpolation mode.
    pub interpolation_mode: InterpolationMode,
    /// Maximum anisotropic ratio.
    pub anisotropic: u32,
    /// If `true` then the lookup will rather over-blur than alias.
    pub conservative_filter: bool,
    /// Blur amount in `s` direction.
    pub s_blur: f32,
    /// Blur amount in `t` direction.
    pub t_blur: f32,
    /// Blur amount in the `r` direction
    pub r_blur: f32,
    /// Multiplier for derivative in `s` direction.
    pub s_width: f32,
    /// Multiplier for derivative in `t` direction.
    pub t_width: f32,
    /// Multiplier for derivatives in `r` direction.
    pub r_width: f32,
    /// Fill value for missing channels.
    pub fill: f32,
    /// Color for missing texture.
    pub missing_color: Option<&'a [f32]>,
    /// Stratified sample value.
    pub random: f32,
}

impl From<&TextureOptions<'_>> for oiio_TextureOpt_v2_t {
    fn from(t: &TextureOptions<'_>) -> Self {
        let mut dst = MaybeUninit::<oiio_TextureOpt_v2_t>::uninit();

        unsafe {
            oiio_TextureSystem_make_texture_options(
                t.first_channel as _,
                t.sub_image as _,
                t.sub_image_name.as_char_ptr(),
                t.s_wrap.into(),
                t.t_wrap.into(),
                t.mip_mode.into(),
                t.interpolation_mode.into(),
                t.anisotropic.try_into().unwrap(),
                t.conservative_filter,
                t.s_blur,
                t.t_blur,
                t.s_width,
                t.t_width,
                t.fill,
                t.missing_color
                    .as_ref()
                    .map(|c| c as *const _ as *const _)
                    .unwrap_or(ptr::null()) as _,
                t.random,
                t.r_wrap.into(),
                t.r_blur,
                t.r_width,
                &mut dst as *mut _ as _,
            );
            dst.assume_init()
        }
    }
}

impl Default for TextureOptions<'_> {
    fn default() -> Self {
        Self {
            first_channel: 0,
            sub_image: 0,
            sub_image_name: Ustr::default(),
            s_wrap: Wrap::default(),
            t_wrap: Wrap::default(),
            r_wrap: Wrap::default(),
            mip_mode: MipMode::default(),
            interpolation_mode: InterpolationMode::default(),
            anisotropic: 32,
            conservative_filter: true,
            s_blur: 0.0,
            t_blur: 0.0,
            r_blur: 0.0,
            s_width: 1.0,
            t_width: 1.0,
            r_width: 1.0,
            fill: 0.0,
            missing_color: None,
            random: -1.0,
        }
    }
}

/// Used to for interop until we have binary compatibility between
/// `TextureOptions` and `oiio_TextureOpt_t`.
/// This hinges on `Ustring`.
pub(crate) struct TextureOpt {
    ptr: *mut oiio_TextureOpt_v2_t,
}

impl Default for TextureOpt {
    fn default() -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_TextureOpt_v2_t>::uninit();

        Self {
            ptr: unsafe {
                oiio_TextureOpt_v2_default(&raw mut ptr as _);
                ptr.assume_init()
            },
        }
    }
}

impl TextureOpt {
    pub fn as_raw_ptr(&self) -> *const oiio_TextureOpt_v2_t {
        self.ptr as _
    }

    pub fn _as_raw_ptr_mut(&mut self) -> *mut oiio_TextureOpt_v2_t {
        self.ptr
    }
}
