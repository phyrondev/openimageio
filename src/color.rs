use crate::*;
use anyhow::{Result, anyhow};
use core::mem::MaybeUninit;
use parking_lot::{ArcRwLockReadGuard, ArcRwLockWriteGuard, RawRwLock, RwLock};
use std::sync::Arc;

/// Represents the set of all color transformations that are allowed.
///
/// If [OpenColorIO](https://opencolorio.org/) is enabled at build time, a
/// [configuration](https://opencolorio.readthedocs.io/en/latest/configurations/_index.html)
/// can be loaded at runtime, allowing the user to have complete control of all
/// color transformation math.
///
/// If OpenColorIO is not enabled at build time, a generic color configuration
/// is provided for minimal color support.
///
/// > ⚠ `ColorConfig`(s) construction is potentially heavy-weight.
/// > As such construction/dropping should be kept to a minimum. Because of this
/// > the type supports cheap cloning. Clones refer to the same underlying
/// > `ColorConfig` instance.
#[derive(Clone)]
pub struct ColorConfig {
    pub(crate) ptr: Arc<RwLock<*mut oiio_ColorConfig_t>>,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// # Constructors
impl ColorConfig {
    /// Construct a OpenColorIO configuration by reading the env variable
    /// `$OCIO`.
    pub fn new() -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_ColorConfig_t>::uninit();

        unsafe {
            oiio_ColorConfig_ctor(StringView::default().ptr, &mut ptr as *mut _ as _);

            Self {
                ptr: Arc::new(RwLock::new(ptr.assume_init())),
            }
        }
    }

    /// Construct a OpenColorIO configuration using the given file.
    pub fn from_file(path: &Utf8Path) -> Result<Self> {
        let mut ptr = MaybeUninit::<*mut oiio_ColorConfig_t>::uninit();

        let color_config = unsafe {
            oiio_ColorConfig_ctor(StringView::from(path).ptr, &mut ptr as *mut _ as _);

            Self {
                ptr: Arc::new(RwLock::new(ptr.assume_init())),
            }
        };

        if color_config.is_ok() {
            Ok(color_config)
        } else {
            Err(anyhow!(color_config.error(true).unwrap_or(
                "ColorConfig::from_file(): unknown error".into()
            )))
        }
    }
}

/// # Getters
impl ColorConfig {
    pub fn is_ok(&self) -> bool {
        let mut is_error = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ColorConfig_has_error(*self.ptr.read_arc(), &mut is_error as *mut _ as _);

            !is_error.assume_init()
        }
    }

    pub fn error(&self, clear: bool) -> Option<String> {
        let mut error = MaybeUninit::<*mut oiio_String_t>::uninit();

        if unsafe {
            0 != oiio_ColorConfig_geterror(*self.ptr.read_arc(), clear, &mut error as *mut _ as _)
        } {
            // Something went wrong.
            None
        } else {
            let error = OiioString::from(unsafe { error.assume_init() });

            if error.is_empty() {
                None
            } else {
                Some(error.to_string())
            }
        }
    }

    //gen_fn_is_ok!(oiio_ColorConfig_has_error);
    //gen_fn_error!(oiio_ColorConfig_geterror);
}

impl ColorConfig {
    pub(crate) fn read_arc(&self) -> ArcRwLockReadGuard<RawRwLock, *mut oiio_ColorConfig_t> {
        self.ptr.read_arc()
    }

    #[allow(dead_code)]
    pub(crate) fn write_arc(&self) -> ArcRwLockWriteGuard<RawRwLock, *mut oiio_ColorConfig_t> {
        self.ptr.write_arc()
    }
}

impl Drop for ColorConfig {
    fn drop(&mut self) {
        let ptr_locked = self.ptr.write_arc();

        if 1 == Arc::<RwLock<*mut oiio_ColorConfig_t>>::strong_count(&self.ptr) {
            // FIXME? Can we have a situation where the cache is dropped
            // while another thread copies some object that holds a reference
            // to the cache?.
            unsafe {
                oiio_ColorConfig_dtor(*ptr_locked);
            }
        }
    }
}
