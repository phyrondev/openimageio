use crate::{algorithms::*, *};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Kernel {
    Gaussian(f32, f32),
    SharpGaussian(f32, f32),
    Box(f32, f32),
    Triangle(f32, f32),
    BlackmanHarris(f32, f32),
    Mitchell(f32, f32),
    Bspline(f32, f32),
    CatmullRom,
    Lanczos3,
    Disk(f32, f32),
    Binomial(f32, f32),
    Laplacian(f32, f32),
}

impl From<Kernel> for &str {
    fn from(kernel: Kernel) -> Self {
        match kernel {
            Kernel::Gaussian(_, _) => "gaussian",
            Kernel::SharpGaussian(_, _) => "sharp-gaussian",
            Kernel::Box(_, _) => "box",
            Kernel::Triangle(_, _) => "triangle",
            Kernel::BlackmanHarris(_, _) => "blackman-harris",
            Kernel::Mitchell(_, _) => "mitchell",
            Kernel::Bspline(_, _) => "b-spline",
            Kernel::CatmullRom => "catmull-rom",
            Kernel::Lanczos3 => "lanczos3",
            Kernel::Disk(_, _) => "disk",
            Kernel::Binomial(_, _) => "binomial",
            Kernel::Laplacian(_, _) => "laplacian",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FromKernelOptions {
    depth: f32,
    normalize: bool,
}

impl Default for FromKernelOptions {
    fn default() -> Self {
        Self {
            depth: 1.0,
            normalize: false,
        }
    }
}

/// These implement [Porter-Duff compositing](https://en.wikipedia.org/wiki/Alpha_compositing).
impl ImageBuffer {
    #[named]
    pub fn from_kernel(kernel: PixelFilter2D, width: f32, height: f32) -> Result<Self> {
        Self::from_kernel_ffi(kernel, width, height, &FromKernelOptions::default())
            .self_or_error(true, function_name!())
    }

    #[named]
    pub fn from_kernel_with(
        kernel: PixelFilter2D,
        width: f32,
        height: f32,
        options: &FromKernelOptions,
    ) -> Result<Self> {
        Self::from_kernel_ffi(kernel, width, height, options).self_or_error(true, function_name!())
    }
}

impl ImageBuffer {
    #[inline]
    fn from_kernel_ffi(
        kernel: PixelFilter2D,
        width: f32,
        height: f32,
        options: &FromKernelOptions,
    ) -> Self {
        let mut kernel_buffer = MaybeUninit::<*mut oiio_ImageBuf_t>::uninit();

        let kernel_str: &str = kernel.into();

        unsafe {
            oiio_ImageBufAlgo_from_kernel(
                StringView::from(kernel_str).as_raw_ptr() as _,
                width,
                height,
                options.depth,
                options.normalize,
                &raw mut kernel_buffer as _,
            );

            Self::from_raw_ptr(kernel_buffer.assume_init())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{algorithms::*, *};

    #[test]
    fn from_kernel() -> Result<()> {
        // Load fg image. This is 1024×1024
        let blur_kernel = ImageBuffer::from_kernel_with(
            PixelFilter2D::Gaussian,
            4.0,
            4.0,
            &FromKernelOptions {
                normalize: true,
                ..Default::default()
            },
        )?;

        blur_kernel.write(Utf8Path::new("target/from_kernel.exr"))
    }
}
