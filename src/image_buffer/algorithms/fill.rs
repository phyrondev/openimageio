use crate::{algorithms::*, *};
use core::mem::MaybeUninit;

/// # Fill
///
/// Fill an image region with given channel values,
///
///  Note that the value slices start with channel 0, even if the
/// `RegionOfInterest` indicates that a later channel is the first to be
/// changed.
///
/// Three varieties of `fill...()` exist:
///
/// * [Uniform fill](#uniform-fill)
///
/// * [Vertical gradient fill](#vertical-gradient-fill)
///
/// * [Four corner gradient fill](#four-corner-gradient-fill)
///
/// ## Uniform Fill
///
/// A single set of channel values that will apply to the whole
/// `RegionOfInterest`.
impl ImageBuffer {
    #[named]
    pub fn from_fill(values: &[f32], region: &Region) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.fill_ffi(
            values,
            &Options {
                region_of_interest: RegionOfInterest::Region(region.clone()),
                ..Default::default()
            },
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_fill_with(
        values: &[f32],
        region: &Region,
        thread_count: Option<u16>,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.fill_ffi(
            values,
            &Options {
                region_of_interest: RegionOfInterest::Region(region.clone()),
                thread_count: thread_count.unwrap_or(0),
            },
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn fill(&mut self, values: &[f32]) -> Result<&mut Self> {
        let is_ok = self.fill_ffi(values, &Options::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn fill_with(
        &mut self,
        values: &[f32],
        options: &Options,
    ) -> Result<&mut Self> {
        let is_ok = self.fill_ffi(values, options);

        self.mut_self_or_error(is_ok, function_name!())
    }
}

/// ## Vertical Gradient Fill
///
/// Two sets of valuesthat will create a linearly interpolated gradient from top
/// to bottom of the `RegionOfInterest`.
impl ImageBuffer {
    #[named]
    pub fn from_fill_vertical(
        start: &[f32],
        end: &[f32],
        region: &Region,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.fill_vertical_ffi(
            start,
            end,
            &Options {
                region_of_interest: RegionOfInterest::Region(region.clone()),
                ..Default::default()
            },
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_fill_vertical_with(
        start: &[f32],
        end: &[f32],
        region: &Region,
        thread_count: Option<u16>,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.fill_vertical_ffi(
            start,
            end,
            &Options {
                region_of_interest: RegionOfInterest::Region(region.clone()),
                thread_count: thread_count.unwrap_or(0),
            },
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn fill_vertical(
        &mut self,
        start: &[f32],
        end: &[f32],
    ) -> Result<&mut Self> {
        let is_ok = self.fill_vertical_ffi(start, end, &Options::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn fill_vertical_with(
        &mut self,
        start: &[f32],
        end: &[f32],
        options: &Options,
    ) -> Result<&mut Self> {
        let is_ok = self.fill_vertical_ffi(start, end, options);

        self.mut_self_or_error(is_ok, function_name!())
    }
}

/// ## Four Corner Gradient Fill
///
/// Four sets of values that will be bilinearly interpolated across all four
/// corners of the `RegionOfInterest`.
impl ImageBuffer {
    #[named]
    pub fn from_fill_corners(
        top_left: &[f32],
        top_right: &[f32],
        bottom_left: &[f32],
        bottom_right: &[f32],
        region: &Region,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.fill_corners_ffi(
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            &Options {
                region_of_interest: RegionOfInterest::Region(region.clone()),
                ..Default::default()
            },
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn from_fill_corners_with(
        top_left: &[f32],
        top_right: &[f32],
        bottom_left: &[f32],
        bottom_right: &[f32],
        region: &Region,
        thread_count: Option<u16>,
    ) -> Result<Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.fill_corners_ffi(
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            &Options {
                region_of_interest: RegionOfInterest::Region(region.clone()),
                thread_count: thread_count.unwrap_or(0),
            },
        );

        image_buffer.self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn fill_corners(
        &mut self,
        top_left: &[f32],
        top_right: &[f32],
        bottom_left: &[f32],
        bottom_right: &[f32],
    ) -> Result<&mut Self> {
        let is_ok = self.fill_corners_ffi(
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            &Options::default(),
        );

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn fill_corners_with(
        &mut self,
        top_left: &[f32],
        top_right: &[f32],
        bottom_left: &[f32],
        bottom_right: &[f32],
        options: &Options,
    ) -> Result<&mut Self> {
        let is_ok = self.fill_corners_ffi(
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            options,
        );

        self.mut_self_or_error(is_ok, function_name!())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn fill() -> Result<()> {
        let pink = [1.0, 0.7, 0.7];
        let red = [1.0, 0.0, 0.0];
        let blue = [0.0, 0.1, 0.8];
        let yellow = [0.5, 0.5, 0.0];

        // Create a new 640x480 RGB image, with a top-to-bottom gradient
        // from red to pink

        let mut image_buf = ImageBuffer::from_fill_vertical(
            &pink,
            &red,
            &Region::new(0..640, 0..480, Some(0..1), Some(0..3)),
        )?;

        // Draw a filled red rectangle overtop existing image A.
        image_buf.fill_with(
            &red,
            &Options {
                region_of_interest: RegionOfInterest::Region(Region::new_2d(
                    50..100,
                    75..175,
                )),
                ..Default::default()
            },
        )?;

        // Draw a filled red rectangle overtop existing image A.
        image_buf.fill_corners(&red, &blue, &yellow, &pink)?;

        compare_images(&image_buf, "test_fill.png")?;

        Ok(())
    }
}

impl ImageBuffer {
    #[inline(always)]
    fn fill_ffi(&mut self, values: &[f32], options: &Options) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_fill(
                self.ptr,
                CspanF32::new(values).ptr as _,
                *options.region_of_interest.as_raw_ptr(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }

    #[inline(always)]
    fn fill_vertical_ffi(
        &mut self,
        start: &[f32],
        end: &[f32],
        options: &Options,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_fill_vertical(
                self.ptr,
                CspanF32::new(start).ptr as _,
                CspanF32::new(end).ptr as _,
                *options.region_of_interest.as_raw_ptr(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }

    #[inline(always)]
    fn fill_corners_ffi(
        &mut self,
        top_left: &[f32],
        top_right: &[f32],
        bottom_left: &[f32],
        bottom_right: &[f32],
        options: &Options,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_fill_corners(
                self.ptr,
                CspanF32::new(top_left).ptr as _,
                CspanF32::new(top_right).ptr as _,
                CspanF32::new(bottom_left).ptr as _,
                CspanF32::new(bottom_right).ptr as _,
                *options.region_of_interest.as_raw_ptr(),
                options.thread_count as _,
                &mut is_ok as *mut _ as _,
            );

            is_ok.assume_init()
        }
    }
}