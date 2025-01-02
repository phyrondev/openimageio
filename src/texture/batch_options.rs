use crate::*;

/// Holds many options controlling single-point texture lookups. Because each
/// texture lookup call takes a reference to a `TextureOptions`.
#[derive(Debug, PartialEq)]
pub struct TextureBatchOptions<'a> {
    // We use `u16` for some only-ever-positive `i32` values in the FFI struct that can reasonably
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
    pub s_blur: [f32; 16],
    /// Blur amount in `t` direction.
    pub t_blur: [f32; 16],
    /// Blur amount in the `r` direction
    pub r_blur: [f32; 16],
    /// Multiplier for derivative in `s` direction.
    pub s_width: [f32; 16],
    /// Multiplier for derivative in `t` direction.
    pub t_width: [f32; 16],
    /// Multiplier for derivatives in `r` direction.
    pub r_width: [f32; 16],
    /// Fill value for missing channels.
    pub fill: f32,
    /// Color for missing texture.
    pub missing_color: Option<&'a [f32]>,
    /// Stratified sample value.
    pub random: [f32; 16],
}

impl From<&TextureBatchOptions<'_>> for oiio_TextureOptBatch_v1_t {
    fn from(t: &TextureBatchOptions<'_>) -> Self {
        let mut dst = MaybeUninit::<oiio_TextureOptBatch_v1_t>::uninit();

        unsafe {
            oiio_TextureSystem_make_texture_batch_options(
                t.first_channel as _,
                t.sub_image as _,
                t.sub_image_name.as_char_ptr(),
                t.s_wrap.into(),
                t.t_wrap.into(),
                t.mip_mode.into(),
                t.interpolation_mode.into(),
                t.anisotropic.try_into().unwrap(),
                t.conservative_filter,
                &raw const t.s_blur as _,
                &raw const t.t_blur as _,
                &raw const t.s_width as _,
                &raw const t.t_width as _,
                t.fill,
                t.missing_color
                    .as_ref()
                    .map(|c| c as *const _ as _)
                    .unwrap_or(ptr::null()) as _,
                &raw const t.random as _,
                t.r_wrap.into(),
                &raw const t.r_blur as _,
                &raw const t.r_width as _,
                &mut dst as *mut _ as _,
            );
            dst.assume_init()
        }
    }
}

impl Default for TextureBatchOptions<'_> {
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
            s_blur: [0.0f32; 16],
            t_blur: [0.0f32; 16],
            r_blur: [0.0f32; 16],
            s_width: [1.0f32; 16],
            t_width: [1.0f32; 16],
            r_width: [1.0f32; 16],
            fill: 0.0,
            missing_color: None,
            random: [-1.0; 16],
        }
    }
}

/// Used to for interop until we have binary compatibility between
/// `TextureBatchOptions` and `oiio_TextureOptBatch_t`.
/// This hinges on `Ustring`.
pub(crate) struct TextureBatchOpt {
    ptr: *mut oiio_TextureOptBatch_v1_t,
}

impl Default for TextureBatchOpt {
    fn default() -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_TextureOptBatch_v1_t>::uninit();

        Self {
            ptr: unsafe {
                oiio_TextureOptBatch_v1_default(&raw mut ptr as _);
                ptr.assume_init()
            },
        }
    }
}

impl TextureBatchOpt {
    pub fn as_raw_ptr(&self) -> *const oiio_TextureOptBatch_v1_t {
        self.ptr as _
    }

    pub fn _as_raw_ptr_mut(&mut self) -> *mut oiio_TextureOptBatch_v1_t {
        self.ptr
    }
}
