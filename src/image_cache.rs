use crate::*;
use core::mem::MaybeUninit;
use once_cell::sync::OnceCell;
use std::sync::Arc;

// The OIIO C++ API wants the user to decide on resource deallocation if this
// also deletes the shared cache. We turn this on its head by moving the
// decision to initialization time.
//
// I.e. when `ImageCache::shared()` is called, it's `persist` parameter can be
// used to decide that the memory allocated by the cache will persist until the
// app exits.
fn set_or_get_persist(persist: bool) -> bool {
    // This only tracks the singular event when the app asks for the first time
    // to persist the cache. This sets the `OnceCell` and from this we can
    // conclude what to feed `oiio_ImageCache_destroy()`'s `teardown` flag
    // (see `ImageCache::drop()`).
    static PERSIST: OnceCell<()> = OnceCell::new();

    if persist {
        PERSIST.get_or_init(|| ());
        true
    } else {
        // Have we set the cache to persist?
        PERSIST.get().is_some()
    }
}

/// Enables an application to read pixels from a large number of image files
/// while using a small amount of memory and other resources.
///
/// If your application will read pixels from many large image files, use
/// `ImageCache` to manage all the resources.
///
/// It is possible to access thousands of image files totalling hundreds of
/// GBs of pixels, efficiently and using a memory footprint on the order of
/// 50MB.
///
/// Alternatively the same can be done directly using [`ImageInput`].
/// `ImageCache` offers the following advantages over `ImageInput`:
///
/// * Simpler interface -- the only supported operations are:
///
///   * Asking for an [`ImageSpec`] of a subimage.
///
///   * Retrieving a block of pixels.
///
///   * Locking/reading/releasing individual tiles.
///
/// * You refer to images by filename only.
///
///   * No need to keep track of individual file handles or `ImageInput`
///     instances.
///
///   * No need to explicitly open or close files.
///
/// * Thread-safety; if multiple threads are accessing the same file, the cache
///   will handle all the locking and resource sharing.
///
/// * Regardless of how many files are accessed, `ImageCache` will maintain a
///   bounded number of simultaneously-open files. It will automatically close
///   files that have not been accessed recently.
///
/// The cache will use only a small amount of memory regardless of the total
/// number of pixels in all the image files being accessed are.
/// An least-recently-used scheme is used to evic tiles from the cache that
/// haven't been accessed recently.

#[derive(Clone, Debug)]
pub struct ImageCache {
    pub(crate) ptr: Arc<*mut oiio_ImageCache_t>,
}

unsafe impl Send for ImageCache {}
unsafe impl Sync for ImageCache {}

impl PartialEq for ImageCache {
    fn eq(&self, other: &Self) -> bool {
        *self.ptr == *other.ptr
    }
}

/*
use core::{mem, ptr};

pub trait CleanUp {
    fn clean_up(&mut self);
}

pub struct CleanOnDrop<T: CleanUp>(T);

impl<T: CleanUp> Drop for CleanOnDrop<T> {
    fn drop(&mut self) {
        self.0.clean_up();
    }
}

impl<T: CleanUp> CleanOnDrop<T> {
    /// Extracts the inner value, without triggering any clean_up
    pub fn into_inner(mut self) -> T {
        unsafe {
            let inner = ptr::read(&mut self.0);
            mem::forget(self);
            inner
        }
    }
}*/

impl Eq for ImageCache {}

impl Default for ImageCache {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageCache {
    /// Create a unique `ImageCache`.
    pub fn new() -> Self {
        let mut ptr = MaybeUninit::<*mut oiio_ImageCache_t>::uninit();

        Self {
            ptr: Arc::new(unsafe {
                oiio_ImageCache_create(false, &mut ptr as *mut _ as _);
                ptr.assume_init()
            }),
        }
    }

    /// Create a shared `ImageCache` so that multiple parts of an application
    /// that request an `ImageCache` will all end up with the same one.
    ///
    /// # Arguments
    ///
    /// * `persist` -- if set to `true`, the cache will live as long as the
    ///   application. I.e. dropping this instance will not free the resources
    ///   used by the shared cache.
    pub fn shared(persist: bool) -> Self {
        set_or_get_persist(persist);

        let mut ptr = std::mem::MaybeUninit::<*mut oiio_ImageCache_t>::uninit();

        Self {
            ptr: Arc::new(unsafe {
                oiio_ImageCache_create(true, &mut ptr as *mut _ as _);
                ptr.assume_init()
            }),
        }
    }
}

impl ImageCache {
    pub(crate) fn _as_raw_ptr(&self) -> *const oiio_ImageCache_t {
        *self.ptr
    }

    pub(crate) fn as_raw_ptr_mut(&self) -> *mut oiio_ImageCache_t {
        *self.ptr
    }
}

impl Drop for ImageCache {
    /// # Safety
    ///
    /// It is safe to drop a [shared `ImageCache`](ImageCache::shared), as
    /// the implementation will only release its resources when the last shared
    /// instance goes out of scope.
    fn drop(&mut self) {
        if 1 == Arc::<*mut oiio_ImageCache_t>::strong_count(&self.ptr) {
            // FIXME? Can we have a situation where the cache is dropped
            // while another thread copies some object that holds a reference
            // to the cache?
            // If that were the case try_unwrap() would return
            // Result::Err and we wouldn't be able to run the destructor.
            unsafe {
                oiio_ImageCache_destroy(
                    *self.ptr, //.deref_mut(),
                    !set_or_get_persist(false),
                );
            }
        }
    }
}
