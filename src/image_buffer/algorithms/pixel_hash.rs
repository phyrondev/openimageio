use crate::algorithms::*;
use core::mem::MaybeUninit;
use std::hash::{Hash, Hasher};

/// Compute the [SHA-1](https://en.wikipedia.org/wiki/SHA-1) byte hash for all
/// the pixels in the specified region of the image.
impl ImageBuffer {
    pub fn pixel_hash_sha1(&self) -> String {
        self.pixel_hash_sha1_ffi(&PixelHashOptions::default())
    }

    pub fn pixel_hash_sha1_with(&self, options: &PixelHashOptions) -> String {
        self.pixel_hash_sha1_ffi(options)
    }
}

impl Hash for ImageBuffer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(
            self.pixel_hash_sha1_with(&PixelHashOptions {
                // Block size 8 should ensure a 1080p image being hashed on a 128 core machine
                // still maxes out all cores (1080/128 ≈ 8)
                block_size: 8,
                ..Default::default()
            })
            .as_bytes(),
        )
        // TODO: Add all of the `ImageBuffer`'s metadata to the hash too; once
        // we implement accessors for it.
    }
}

/// Options for [`ImageBuffer`]'s
/// [`pixel_hash_sha1_with()`](ImageBuffer::pixel_hash_sha1_with) method.
#[derive(Clone, Default)]
pub struct PixelHashOptions {
    /// See the [Region of Interest](module@algorithms#region-of-interest)
    /// section in the [module@algorithms] module. .
    pub region: Region,
    /// Additional text that will be incorporated into the hash.
    pub extra_info: Option<String>,
    /// If `block_size` > `0`, the function will compute separate SHA-1 hashes
    /// of each `block_size` batch of scanlines, then return a hash of the
    /// individual hashes.
    ///
    /// This is just as strong a hash, but will *not* match a single hash of the
    /// entire image (`block_size` = `0`). But by breaking up the hash into
    /// independent blocks, we can parallelize across multiple threads,
    /// given by `thread_count` (if `thread_count` is 0, it will use the global
    /// OIIO `thread_count`).
    pub block_size: u16,
    /// See the [Multithreading](module@algorithms#multithreading) section
    /// in the [module@algorithms] module.
    pub thread_count: u16,
}

// Internal pixel hash FFI call.
impl ImageBuffer {
    fn pixel_hash_sha1_ffi(&self, options: &PixelHashOptions) -> String {
        unsafe {
            let mut hash = MaybeUninit::<OiioString>::uninit();
            oiio_ImageBufAlgo_computePixelHashSHA1(
                self.ptr,
                options
                    .extra_info
                    .as_ref()
                    .map(|s| StringView::from(s.as_str()))
                    .unwrap_or_default()
                    .as_raw_ptr() as _,
                options.region.clone().into(),
                options.block_size as _,
                options.thread_count as _,
                &mut hash as *mut _ as _,
            );

            format!("{}", hash.assume_init())
        }
    }
}
