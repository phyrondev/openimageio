use crate::*;
use bitflags::bitflags;
use core::{
    marker::PhantomData,
    mem::{transmute, MaybeUninit},
    ops::{Add, Div, Mul},
    ptr,
};
use num_traits::{
    float::Float,
    identities::{One, Zero},
};

mod options;
pub use options::*;
mod batch_options;
pub use batch_options::*;

pub const BATCH_SIZE: usize = 16;

bitflags! {
    /// An integer large enough to hold at least
    /// [`BATCH_SIZE`] bits.
    ///
    /// The least significant bit corresponds to the first (i.e., `[0]`) position
    /// of all batch arrays. For each position `i` in the batch, the bit
    /// identified by `(1 << i)` controls whether that position will be computed.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TextureBatchMask: u64 {
        const FULL = 0xffffffffffffffff;
    }
}

struct Dual2<T: Float + Add<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T> + Zero + One> {
    pub value: T,
    pub dx: T,
    pub dy: T,
}

impl<T: Zero + Float> Dual2<T> {
    fn _new(value: T) -> Self {
        Self {
            value,
            dy: T::zero(),
            dx: T::zero(),
        }
    }
}

impl<T: Float> Add<Dual2<T>> for Dual2<T> {
    type Output = Dual2<T>;

    fn add(self, rhs: Dual2<T>) -> Self::Output {
        Self {
            value: self.value + rhs.value,
            dx: self.dx + rhs.dx,
            dy: self.dy + rhs.dy,
        }
    }
}

impl<T: Float> Add<T> for Dual2<T> {
    type Output = Dual2<T>;

    fn add(self, rhs: T) -> Self::Output {
        Self {
            value: self.value + rhs,
            dx: self.dx,
            dy: self.dy,
        }
    }
}

impl<T: Float> Mul<T> for Dual2<T> {
    type Output = Dual2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            value: self.value * rhs,
            dx: self.dx * rhs,
            dy: self.dy * rhs,
        }
    }
}
impl<T: Float> Mul<Dual2<T>> for Dual2<T> {
    type Output = Dual2<T>;

    fn mul(self, rhs: Dual2<T>) -> Self::Output {
        Self {
            value: self.value * rhs.value,
            dx: self.value * rhs.dx + self.dx * rhs.value,
            dy: self.value * rhs.dy + self.dy * rhs.value,
        }
    }
}

impl<T: Float> Div<T> for Dual2<T> {
    type Output = Dual2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            value: self.value / rhs,
            dx: self.dx / rhs,
            dy: self.dy / rhs,
        }
    }
}

impl<T: Float + One> Div<Dual2<T>> for Dual2<T> {
    type Output = Dual2<T>;

    fn div(self, rhs: Dual2<T>) -> Self::Output {
        let rhs_val_inv = T::one() / rhs.value;
        let aval_rhs_val = self.value * rhs_val_inv;

        Self {
            value: aval_rhs_val,
            dx: rhs_val_inv * (self.dx - aval_rhs_val * rhs.dx),
            dy: rhs_val_inv * (self.dy - aval_rhs_val * rhs.dy),
        }
    }
}

/// Describes what happens when texture coordinates hit a value outside the
/// usual *\[0, 1\]* range where a texture is defined.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
#[repr(u8)]
pub enum Wrap {
    /// Use the default found in the file.
    #[default]
    Default = oiio_Wrap::oiio_Wrap_Default.0 as _,
    /// Black outside [0..1].
    Black = oiio_Wrap::oiio_Wrap_Black.0 as _,
    /// Clamp to [0..1]
    Clamp = oiio_Wrap::oiio_Wrap_Clamp.0 as _,
    /// Periodic mod 1.
    Periodic = oiio_Wrap::oiio_Wrap_Periodic.0 as _,
    /// Mirror the image.
    Mirror = oiio_Wrap::oiio_Wrap_Mirror.0 as _,
    /// Periodic, but only for powers of 2.
    PeriodicPow2 = oiio_Wrap::oiio_Wrap_PeriodicPow2.0 as _,
    /// Periodic with shared border (environment map).
    PeriodicSharedBorder = oiio_Wrap::oiio_Wrap_PeriodicSharedBorder.0 as _,
}

impl From<Wrap> for oiio_Wrap {
    fn from(m: Wrap) -> Self {
        unsafe { transmute(m) }
    }
}

/// Determines if/how we use mipmaps.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
#[repr(u8)]
pub enum MipMode {
    /// Default high-quality lookup.
    #[default]
    Default = oiio_MipMode::oiio_MipMode_Default.0 as _,
    /// Just use highest-res image, no mipmapping,
    NoMip = oiio_MipMode::oiio_MipMode_NoMIP.0 as _,
    /// Use just one mipmap level.
    OneLevel = oiio_MipMode::oiio_MipMode_OneLevel.0 as _,
    /// Use two mipmap levels (trilinear).
    Trilinear = oiio_MipMode::oiio_MipMode_Trilinear.0 as _,
    /// Use two mipmap levels with anisotropic filtering.
    Anisotropic = oiio_MipMode::oiio_MipMode_Aniso.0 as _,
}

impl From<MipMode> for oiio_MipMode {
    fn from(m: MipMode) -> Self {
        unsafe { transmute(m) }
    }
}

/// Determines how we sample within a mipmap level.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
#[repr(u8)]
pub enum InterpolationMode {
    /// Closest texel.
    Closest = oiio_InterpMode::oiio_InterpMode_Closest.0 as _,
    /// Bilinear lookup within a mip level.
    Bilinear = oiio_InterpMode::oiio_InterpMode_Bilinear.0 as _,
    /// Cubic lookup within a mip level.
    Bicubic = oiio_InterpMode::oiio_InterpMode_Bicubic.0 as _,
    /// Bicubic when magnifying, else bilinear.
    #[default]
    SmartBicubic = oiio_InterpMode::oiio_InterpMode_SmartBicubic.0 as _,
}

impl From<InterpolationMode> for oiio_InterpMode {
    fn from(m: InterpolationMode) -> Self {
        unsafe { transmute(m) }
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

impl TextureHandle<'_, '_> {
    pub fn texture(
        &self,
        s: f32,
        t: f32,
        delta_s_dx: f32,
        delta_t_dx: f32,
        delta_s_dy: f32,
        delta_t_dy: f32,
        channel_count: u16,
        options: Option<&TextureOptions>,
    ) -> Result<Vec<f32>> {
        let options: *const oiio_TextureOpt_v2_t = options
            .map(|o| &o.into() as *const _ as _)
            .unwrap_or(TextureOpt::default().as_raw_ptr());

        let mut result = Vec::with_capacity(channel_count as _);
        let mut is_ok = std::mem::MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_TextureSystem_texture(
                self.system.ptr,
                self.ptr,
                // Perthread
                std::ptr::null_mut() as _,
                options as *mut _,
                s,
                t,
                delta_s_dx,
                delta_t_dx,
                delta_s_dy,
                delta_t_dy,
                channel_count as _,
                result.as_mut_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &raw mut is_ok as _,
            );

            result.set_len(channel_count as _);

            if is_ok.assume_init() {
                Ok(result)
            } else {
                Err(anyhow!("Texture lookup failed"))
            }
        }
    }

    pub fn texture_batch_16(
        &self,
        mask: TextureBatchMask,
        s: &[f32; 16],
        t: &[f32; 16],
        delta_s_dx: &[f32; 16],
        delta_t_dx: &[f32; 16],
        delta_s_dy: &[f32; 16],
        delta_t_dy: &[f32; 16],
        channel_count: u16,
        options: Option<&TextureBatchOptions>,
    ) -> Result<Vec<f32>> {
        debug_assert!(s.len() <= t.len());
        debug_assert!(s.len() == t.len());

        let options: *const oiio_TextureOptBatch_v1_t = options
            .map(|o| &o.into() as *const _ as _)
            .unwrap_or(TextureBatchOpt::default().as_raw_ptr());

        let mut result = Vec::with_capacity(channel_count as _);
        let mut is_ok = std::mem::MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_TextureSystem_texture_multi(
                self.system.ptr,
                self.ptr,
                // Perthread
                std::ptr::null_mut() as _,
                options as *mut _,
                mask.bits(),
                s as *const _ as _,
                t as *const _ as _,
                delta_s_dx as *const _ as _,
                delta_t_dx as *const _ as _,
                delta_s_dy as *const _ as _,
                delta_t_dy as *const _ as _,
                channel_count as _,
                result.as_mut_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &raw mut is_ok as _,
            );

            result.set_len(channel_count as _);

            if is_ok.assume_init() {
                Ok(result)
            } else {
                Err(anyhow!("Texture lookup failed"))
            }
        }
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
    ptr: *mut oiio_TextureSystemSharedPtr_t,
    _marker: PhantomData<*mut &'a ()>,
}

impl<'a> TextureSystem<'a> {
    #[inline]
    fn do_new(shared: bool, image_cache: Option<&'a ImageCache>) -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_TextureSystemSharedPtr_t>::uninit();

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
                &raw mut ptr as _,
            );

            TextureHandle {
                ptr: ptr.assume_init(),
                system: self,
            }
        }
    }
}

impl Drop for TextureSystem<'_> {
    fn drop(&mut self) {
        unsafe { oiio_TextureSystem_destroy(self.ptr, false) };
    }
}
