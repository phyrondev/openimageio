use crate::*;
use anyhow::Result;
use std::mem::MaybeUninit;

impl<'a> ImageBuffer<'a> {
    fn zero_ffi(
        &mut self,
        roi: Option<RegionOfInterest>,
        thread_count: Option<u16>,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_zero(
                self.ptr,
                std::mem::transmute::<RegionOfInterest, oiio_ROI_t>(
                    roi.unwrap_or(self.roi()),
                ),
                thread_count.unwrap_or_default() as _,
                &mut is_ok as *mut _ as *mut _,
            );

            is_ok.assume_init()
        }
    }

    /// Set all channels to black.
    ///
    /// Errors will be logged.
    pub fn zero(&mut self) -> &mut Self {
        let is_ok = self.zero_ffi(None, None);
        self.ok_or_log_error(is_ok)
    }

    /// Set all channels as described by the [`RegionOfInterest`] to black.
    ///
    /// Errors will be logged.
    pub fn zero_with(
        &mut self,
        roi: Option<RegionOfInterest>,
        thread_count: Option<u16>,
    ) -> &mut Self {
        let is_ok = self.zero_ffi(roi, thread_count);
        self.ok_or_log_error(is_ok)
    }

    /// Try setting all channels as described by the [`RegionOfInterest`] to
    /// black.
    pub fn try_zero(
        &mut self,
        roi: Option<RegionOfInterest>,
        thread_count: Option<u16>,
    ) -> Result<&mut Self> {
        let is_ok = self.zero_ffi(roi, thread_count);
        self.ok_or_error(is_ok)
    }

    fn do_noise(
        &mut self,
        noise_type: &str,
        a: f32,
        b: f32,
        monochromatic: Option<bool>,
        seed: Option<i32>,
        roi: Option<RegionOfInterest>,
        thread_count: Option<u16>,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_noise(
                self.ptr,
                StringView::from(noise_type).as_raw_ptr_mut(),
                a,
                b,
                monochromatic.unwrap_or_default(),
                seed.unwrap_or_default(),
                std::mem::transmute::<RegionOfInterest, oiio_ROI_t>(
                    roi.unwrap_or(self.roi()),
                ),
                thread_count.unwrap_or_default() as _,
                &mut is_ok as *mut _ as *mut _,
            );

            is_ok.assume_init()
        }
    }
}
