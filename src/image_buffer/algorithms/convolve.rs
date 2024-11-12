use crate::{algorithms::*, *};
use anyhow::Result;
use core::mem::MaybeUninit;

/// Optional parameters for [`ImageBuffer`]'s
/// [`replace_by_convolve_with()`](ImageBuffer::replace_by_convolve_with) and
/// [`covolve_with()`](ImageBuffer::covolve_with) methods.
pub struct ConvolveOptions {
    /// If `true`, the kernel will be normalized for the convolution, otherwise
    /// the original values will be used.
    normalize: bool,
    /// See the [Region of Interest](module@algorithms#region-of-interest)
    /// section in the [module@algorithms] module. .
    pub region: Region,
    /// See the [Multithreading](module@algorithms#multithreading) section
    /// in the [module@algorithms] module.
    pub thread_count: u16,
}

impl Default for ConvolveOptions {
    fn default() -> Self {
        Self {
            normalize: true,
            region: Region::default(),
            thread_count: 0,
        }
    }
}

/// # Convolve
///
/// Compute convolution with a kernel.
///
/// If `roi is not defined, it
/// defaults to the full size src. If normalized is true, the kernel will be
/// normalized for the convolution, otherwise the original values will be used.
impl ImageBuffer {
    #[named]
    pub fn replace_by_convolve(
        &mut self,
        source: &ImageBuffer,
        kernel: &ImageBuffer,
    ) -> Result<&mut Self> {
        let is_ok = self.convolve_ffi(source, kernel, &ConvolveOptions::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_by_convolve_with(
        &mut self,
        source: &ImageBuffer,
        kernel: &ImageBuffer,
        options: &ConvolveOptions,
    ) -> Result<&mut Self> {
        let is_ok = self.convolve_ffi(source, kernel, options);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn convolve(&mut self, kernel: &ImageBuffer) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.convolve_ffi(self, kernel, &ConvolveOptions::default());
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn convolve_with(
        &mut self,
        kernel: &ImageBuffer,
        options: &ConvolveOptions,
    ) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.convolve_ffi(self, kernel, options);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

impl ImageBuffer {
    #[inline]
    fn convolve_ffi(
        &mut self,
        source: &ImageBuffer,
        kernel: &ImageBuffer,
        options: &ConvolveOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_convolve(
                self.as_raw_ptr_mut(),
                source.as_raw_ptr(),
                kernel.as_raw_ptr(),
                options.normalize,
                options.region.clone().into(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{algorithms::*, *};

    #[test]
    fn convolve() -> Result<()> {
        let mut image_buffer =
            ImageBuffer::from_file(Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"))?;
        let kernel = ImageBuffer::from_kernel(PixelFilter2D::Disk, 15.0, 15.0)?;

        image_buffer.convolve(&kernel)?;

        image_buffer.write(Utf8Path::new("target/convolve.exr"))?;

        #[cfg(feature = "image")]
        {
            let image: image::DynamicImage = image_buffer.try_into()?;

            viuer::print(&image, &viuer::Config {
                width: Some(80),
                height: Some(40),
                ..Default::default()
            })?;
        }

        Ok(())
    }
}
