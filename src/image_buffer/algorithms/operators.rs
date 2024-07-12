use crate::*;
use anyhow::Result;
use std::mem::MaybeUninit;

#[non_exhaustive]
pub enum PixelFilter {
    Gaussian(f32),
    SharpGaussian(f32),
    Box(f32),
    Triangle(f32),
    Mitchell(f32),
    BlackmanHarris(f32),
    Bspline(f32),
    CatmullRom,
    Lanczos3,
    Cubic(f32),
    Keys(f32),
    Simon(f32),
    Rifman(f32),
    Disk(f32),
    Binomial(f32),
    Laplacian(f32),
}

impl From<PixelFilter> for &str {
    fn from(pf: PixelFilter) -> Self {
        match pf {
            PixelFilter::Gaussian(_) => "gaussian",
            PixelFilter::SharpGaussian(_) => "sharp-gaussian",
            PixelFilter::Box(_) => "box",
            PixelFilter::Triangle(_) => "triangle",
            PixelFilter::Mitchell(_) => "mitchell",
            PixelFilter::BlackmanHarris(_) => "blackman-harris",
            PixelFilter::Bspline(_) => "b-spline",
            PixelFilter::CatmullRom => "catmull-rom",
            PixelFilter::Lanczos3 => "lanczos3",
            PixelFilter::Cubic(_) => "cubic",
            PixelFilter::Keys(_) => "keys",
            PixelFilter::Simon(_) => "simon",
            PixelFilter::Rifman(_) => "rifman",
            PixelFilter::Disk(_) => "disk",
            PixelFilter::Binomial(_) => "binomial",
            PixelFilter::Laplacian(_) => "laplacian",
            //_ => "unknown",
        }
    }
}

// Actual implementations.
impl<'a> ImageBuffer<'a> {
    fn over_ffi(&mut self, other: &ImageBuffer, options: Options) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_over(
                self.ptr,
                self.ptr,
                other.ptr,
                std::mem::transmute::<Roi, oiio_ROI_t>(
                    roi.unwrap_or(self.roi()),
                ),
                options.thread_count.unwrap_or_default() as _,
                &mut is_ok as *mut _ as *mut _,
            );

            is_ok.assume_init()
        }
    }

    /*fn rotate_ffi(
        &mut self,
        other: &ImageBuffer,
        pixel_filter: Option<PixelFilter>,
        roi: Option<Roi>,
        thread_count: Option<u16>,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_rotate(
                self.ptr,
                other.ptr,
                std::mem::transmute::<Roi, oiio_ROI_t>(
                    roi.unwrap_or(self.roi()),
                ),
                thread_count.unwrap_or_default() as _,
                &mut is_ok as *mut _ as *mut _,
            );

            is_ok.assume_init()
        }
    }*/
}

struct Options {
    region_of_interest: Option<Roi>,
    thread_count: Option<u16>,
}

/// # Compositing Operators
///
/// These implement [Porter-Duff compositing](https://en.wikipedia.org/wiki/Alpha_compositing).
impl<'a> ImageBuffer<'a> {
    pub fn over(&mut self, other: &ImageBuffer) -> &mut Self {
        let is_ok = self.over_ffi(other, None, None);
        self.ok_or_log_error(is_ok);
        self
    }

    pub fn over_with(
        &mut self,
        other: &ImageBuffer,
        options: Option<Options>,
    ) -> &mut Self {
        let is_ok = self.over_ffi(other, region_of_interest, thread_count);
        self.ok_or_log_error(is_ok)
    }

    pub fn try_over(&mut self, other: &ImageBuffer) -> Result<&mut Self> {
        let is_ok = self.over_ffi(other, None, None);
        self.ok_or_error(is_ok)
    }

    pub fn try_over_with(
        &mut self,
        other: &ImageBuffer,
        options: Option<Options>,
    ) -> Result<&mut Self> {
        let is_ok = self.over_ffi(other, region_of_interest, thread_count);
        self.ok_or_error(is_ok)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn over() {
        let mut image_buf_a = ImageBuffer::new();
        let mut image_buf_b = ImageBuffer::new();
        let image_buf_c = ImageBuffer::new();

        image_buf_a.over(image_buf_b.over(&image_buf_c));
    }
}
