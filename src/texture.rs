use crate::*;
use core::{
    marker::PhantomData,
    mem::{MaybeUninit, transmute},
    ptr,
};
use ustr::Ustr;

/// Describes what happens when texture coordinates hit a value outside the
/// usual *\[0, 1\]* range where a texture is defined.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
#[repr(C)]
pub enum Wrap {
    /// Use the default found in the file.
    #[default]
    Default = oiio_Wrap::oiio_Wrap_WrapDefault.0 as _,
    /// Black outside [0..1].
    Black = oiio_Wrap::oiio_Wrap_WrapBlack.0 as _,
    /// Clamp to [0..1]
    Clamp = oiio_Wrap::oiio_Wrap_WrapClamp.0 as _,
    /// Periodic mod 1.
    Periodic = oiio_Wrap::oiio_Wrap_WrapPeriodic.0 as _,
    /// Mirror the image.
    Mirror = oiio_Wrap::oiio_Wrap_WrapMirror.0 as _,
    /// Periodic, but only for powers of 2.
    PeriodicPow2 = oiio_Wrap::oiio_Wrap_WrapPeriodicPow2.0 as _,
    /// Periodic with shared border (environment map).
    PeriodicSharedBorder = oiio_Wrap::oiio_Wrap_WrapPeriodicSharedBorder.0 as _,
}

impl From<Wrap> for oiio_Wrap {
    fn from(m: Wrap) -> Self {
        unsafe { transmute(m) }
    }
}

/// Determines if/how we use mipmaps.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
#[repr(C)]
pub enum MipMode {
    /// Default high-quality lookup.
    #[default]
    Default = oiio_MipMode::oiio_MipMode_MipModeDefault.0 as _,
    /// Just use highest-res image, no mipmapping,
    NoMip = oiio_MipMode::oiio_MipMode_MipModeNoMIP.0 as _,
    /// Use just one mipmap level.
    OneLevel = oiio_MipMode::oiio_MipMode_MipModeOneLevel.0 as _,
    /// Use two mipmap levels (trilinear).
    Trilinear = oiio_MipMode::oiio_MipMode_MipModeTrilinear.0 as _,
    /// Use two mipmap levels with anisotropic filtering.
    Anisotropic = oiio_MipMode::oiio_MipMode_MipModeAniso.0 as _,
}

impl From<MipMode> for oiio_MipMode {
    fn from(m: MipMode) -> Self {
        unsafe { transmute(m) }
    }
}

/// Determines how we sample within a mipmap level.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
#[repr(C)]
pub enum InterpolationMode {
    /// Closest texel.
    Closest = oiio_InterpMode::oiio_InterpMode_InterpClosest.0 as _,
    /// Bilinear lookup within a mip level.
    Bilinear = oiio_InterpMode::oiio_InterpMode_InterpBilinear.0 as _,
    /// Cubic lookup within a mip level.
    Bicubic = oiio_InterpMode::oiio_InterpMode_InterpBicubic.0 as _,
    /// Bicubic when magnifying, else bilinear.
    #[default]
    SmartBicubic = oiio_InterpMode::oiio_InterpMode_InterpSmartBicubic.0 as _,
}

impl From<InterpolationMode> for oiio_InterpMode {
    fn from(m: InterpolationMode) -> Self {
        unsafe { transmute(m) }
    }
}

/// Holds many options controlling single-point texture lookups. Because each
/// texture lookup call takes a reference to a `TextureOptions`.
#[derive(Debug, PartialEq)]
pub struct TextureOptions<'a> {
    /// First channel of the lookup.
    pub first_channel: u32,
    /// Sub-image or `Ptex` face ID.
    pub sub_image: u32,
    /// Sub-image name.
    pub sub_image_name: Ustr,
    /// Wrap mode in the `s` direction.
    pub s_wrap: Wrap,
    /// Wrap mode in the `t` direction.
    pub t_wrap: Wrap,
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
    /// Multiplier for derivative in `s` direction.
    pub s_width: f32,
    /// Multiplier for derivative in `t` direction.
    pub t_width: f32,
    /// Fill value for missing channels.
    pub fill: f32,
    /// Color for missing texture.
    pub missing_color: Option<&'a [f32]>,
    /// Time (for time-dependent texture lookups).
    pub time: f32,
    /// Stratified sample value.
    pub random: f32,
    /// Number of samples for shadows.
    pub samples: u32,
    /// Wrap mode in the `r` direction for 3D volume texture lookups only.
    pub r_wrap: Wrap,
    /// Blur amount in the `r` direction
    pub r_blur: f32,
    /// Multiplier for derivatives in `r` direction.
    pub r_width: f32,
}

impl From<&TextureOptions<'_>> for oiio_TextureOpt_t {
    fn from(t: &TextureOptions<'_>) -> Self {
        let mut dst = MaybeUninit::<oiio_TextureOpt_t>::uninit();

        unsafe {
            oiio_TextureSystem_make_texture_options(
                t.first_channel.try_into().unwrap(),
                t.sub_image.try_into().unwrap(),
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
                t.time,
                t.random,
                t.samples.try_into().unwrap(),
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
            mip_mode: MipMode::default(),
            interpolation_mode: InterpolationMode::default(),
            anisotropic: 32,
            conservative_filter: true,
            s_blur: 0.0,
            t_blur: 0.0,
            s_width: 1.0,
            t_width: 1.0,
            fill: 0.0,
            missing_color: None,
            time: 0.0,
            random: -1.0,
            samples: 1,
            r_wrap: Wrap::default(),
            r_blur: 0.0,
            r_width: 1.0,
        }
    }
}

/// Used to for interop until we have binary compatibility between
/// `TextureOptions` and `oiio_TextureOpt_t`.
/// This hinges on `Ustring`.
struct TextureOpt {
    ptr: *mut oiio_TextureOpt_t,
}

impl Default for TextureOpt {
    fn default() -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_TextureOpt_t>::uninit();

        Self {
            ptr: unsafe {
                oiio_TextureOpt_default(&mut ptr as *mut _ as _);
                ptr.assume_init()
            },
        }
    }
}

impl TextureOpt {
    fn as_raw_ptr(&self) -> *const oiio_TextureOpt_t {
        self.ptr as _
    }

    fn as_raw_ptr_mut(&mut self) -> *mut oiio_TextureOpt_t {
        self.ptr
    }
}

/// An opaque handle to a texture file.
///
/// Use [`TextureSystem::texture_handle()`] to create a handle.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TextureHandle<'a, 'b> {
    ptr: *mut oiio_TextureHandle_t,
    // We peg this to the lifetime of the parent `TextureSystem` so that we
    // can ensure that a handle remains valid.
    // This is also used to call methods on the resp. `TextureSystem` through
    // this handle.
    system: &'a TextureSystem<'b>,
}

impl<'a, 'b> TextureHandle<'a, 'b> {
    pub fn texture(
        &self,
        s: f32,
        t: f32,
        ds_dx: f32,
        dt_dx: f32,
        ds_dy: f32,
        dt_dy: f32,
        channel_count: u32,
        options: Option<&TextureOptions>,
    ) -> Vec<f32> {
        let options: *const oiio_TextureOpt_t = options
            .map(|o| &o.into() as *const _ as _)
            .unwrap_or(TextureOpt::default().as_raw_ptr());

        let mut result = Vec::with_capacity(channel_count as _);

        unsafe {
            oiio_TextureSystem_texture(
                self.system.ptr,
                self.ptr,
                // Perthread
                std::ptr::null_mut() as _,
                options as *mut oiio_TextureOpt_t as _,
                s,
                t,
                ds_dx,
                dt_dx,
                ds_dy,
                dt_dy,
                channel_count.try_into().unwrap(),
                result.as_mut_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );

            result.set_len(channel_count as _);
        }
        result
    }
}

/// Manages texture files, caches of open file handles as well as tiles of
/// texels.
///
/// This allows huge amounts of textures to be accessed by an application with
/// low memory footprint
///
/// The `TextureSystem` also provides ways to perform antialiased texture-,
/// shadow map-, and environment map lookups.
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct TextureSystem<'a> {
    ptr: *mut oiio_TextureSystem_t,
    _marker: PhantomData<*mut &'a ()>,
}

impl<'a> TextureSystem<'a> {
    #[inline]
    fn do_new(shared: bool, image_cache: Option<&'a ImageCache>) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_TextureSystem_t>::uninit();

        Self {
            ptr: unsafe {
                oiio_TextureSystem_create(
                    shared,
                    image_cache
                        .map(|c| c.as_raw_ptr_mut())
                        .unwrap_or(ptr::null_mut()) as _,
                    &mut ptr as *mut _ as *mut _,
                );
                ptr.assume_init()
            },
            _marker: PhantomData,
        }
    }

    /// Creates a new `TextureSystem`.
    ///
    /// If `image_cache` is `None` a resp. [`ImageCache`] will be created inside
    /// the `TextureSystem`.
    pub fn new(image_cache: Option<&'a ImageCache>) -> Self {
        Self::do_new(false, image_cache)
    }

    /// Creates a shared `TextureSystem` so that multiple parts of an
    /// application all end up with the same one as well as the same underlying
    /// [`ImageCache`].
    pub fn shared() -> Self {
        Self::do_new(true, None)
    }
}

impl<'a> TextureSystem<'a> {
    /// Retrieve a [`TextureHandle`] for a given file name.
    pub fn texture_handle(&self, file_name: &Utf8Path) -> TextureHandle<'_, 'a> {
        let mut ptr = MaybeUninit::<*mut oiio_TextureHandle_t>::uninit();

        unsafe {
            oiio_TextureSystem_texture_handle(
                self.ptr,
                StringView::from(file_name).ptr as _,
                // Prethread
                ptr::null_mut(),
                &mut ptr as *mut _ as _,
            );

            TextureHandle {
                ptr: ptr.assume_init(),
                system: self,
            }
        }
    }
}

impl<'a> Drop for TextureSystem<'a> {
    fn drop(&mut self) {
        unsafe { oiio_TextureSystem_destroy(self.ptr, false) };
    }
}
