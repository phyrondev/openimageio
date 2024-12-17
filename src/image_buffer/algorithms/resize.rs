use crate::{algorithms::*, *};
use anyhow::Result;
use core::mem::MaybeUninit;

/// # Resize
///
/// Set the image over the `Region`, to be a resized version of the
/// corresponding portion of `source` (mapping such that the 'full' image window
/// of each correspond to each other, regardless of resolution).
///
/// Also see the [Region of Interest](#region-of-interest) section on
/// [`ImageBuffer`].
///
/// *By choosing [`Nearest`](PixelFilter2D::Nearest) and
/// [`NearestBilinear`](PixelFilter2D::NearestBilinear) for the
/// [`filter`](ResizeOptions::filter) a **much faster** but lower quality
/// algorithm for the resizing will be used.*
impl ImageBuffer {
    #[named]
    pub fn replace_by_resize(&mut self, source: &ImageBuffer) -> Result<&mut Self> {
        let is_ok = self.resize_or_resample_ffi(source, &ResizeOptions::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_by_resize_with(
        &mut self,
        source: &ImageBuffer,
        resize_options: &ResizeOptions,
    ) -> Result<&mut Self> {
        let is_ok = self.resize_or_resample_ffi(source, resize_options);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn resize(&mut self, new_size: &Bounds) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.resize_or_resample_ffi(
            self,
            &ResizeOptions {
                region: new_size.clone().into(),
                ..Default::default()
            },
        );
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    /// In this variant the `new_size` is set via the [`region`] member of the
    /// `ResizeOptions`.
    #[named]
    pub fn resize_with(&mut self, resize_options: &ResizeOptions) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.resize_or_resample_ffi(self, resize_options);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

/// Optional parameters for [`ImageBuffer`]'s
/// [`replace_by_resize_with()`](ImageBuffer::replace_by_resize_with),
/// [`resize_with()`](ImageBuffer::resize_with) methods.
#[derive(Clone, Default)]
pub struct ResizeOptions {
    /// Either the region to resize from source to the destination or the new
    /// size, if there is no source.
    pub region: Region,
    /// The pixel filter is used to weight the pixels falling underneath
    /// it for each final pixel; the filter’s size is expressed in pixel units
    /// of the the destination image.
    ///
    /// If this is `None` the `resize()` variant will choose a reasonable
    /// high-quality default ([`BlackmanHarris`](PixelFilter2D::BlackmanHarris)
    /// when upsizing, [`Lanczos3`](PixelFilter2D::Lanczos3) when
    /// downsizing).
    pub filter: Option<Filter2D>,
    /// See the [Multithreading](#multithreading) section on [`ImageBuffer`].
    pub thread_count: u16,
}

impl ImageBuffer {
    fn resize_or_resample_ffi(
        &mut self,
        source: &ImageBuffer,
        resize_options: &ResizeOptions,
    ) -> bool {
        if let Some(filter_2d) = resize_options.filter {
            match filter_2d.filter {
                PixelFilter2D::Nearest => self.resample_ffi(source, resize_options, false),
                PixelFilter2D::NearestBilinear => self.resample_ffi(source, resize_options, true),
                _ => self.resize_ffi(source, resize_options),
            }
        } else {
            self.resize_ffi(source, resize_options)
        }
    }

    #[inline]
    fn resize_ffi(&mut self, source: &ImageBuffer, resize_options: &ResizeOptions) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        let options = if let Some(filter) = resize_options.filter {
            let filter_ptr = ParamValue::new_ffi(
                "filterptr",
                // TODO: is this safe? What are the lifetimes expectation for this pointer?
                // We assume it only has to outlive the `oiio_ImageBufAlgo_resize()` call.
                filter.as_raw_ptr(),
                TypeDesc::PTR,
                1,
                &ParamValueOptions::default(),
            );

            let mut options = ParamValueList::new();
            options.add_or_replace(filter_ptr);

            options
        } else {
            ParamValueList::default()
        };

        let options = ParamValueSlice::from(&options);

        unsafe {
            oiio_ImageBufAlgo_resize(
                self.as_raw_ptr_mut(),
                source.as_raw_ptr(),
                options.as_raw_ptr() as _,
                resize_options.region.clone().into(),
                resize_options.thread_count as _,
                &raw mut is_ok as _,
            );

            is_ok.assume_init()
        }
    }

    #[inline]
    fn resample_ffi(
        &mut self,
        source: &ImageBuffer,
        resize_options: &ResizeOptions,
        interpolate: bool,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_ImageBufAlgo_resample(
                self.as_raw_ptr_mut(),
                source.as_raw_ptr(),
                interpolate as _,
                resize_options.region.clone().into(),
                resize_options.thread_count as _,
                &raw mut is_ok as _,
            );

            is_ok.assume_init()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{algorithms::*, *};

    #[test]
    fn resize() -> Result<()> {
        let mut image_buffer =
            ImageBuffer::from_file(Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"))?;

        image_buffer.resize_with(&ResizeOptions {
            region: Bounds::new_2d(0..80, 0..80).into(),
            filter: Some(PixelFilter2D::NearestBilinear.into()),
            ..Default::default()
        })?;

        #[cfg(feature = "image")]
        {
            image_buffer.color_convert(None, "sRGB")?;
            let image: image::DynamicImage = image_buffer.try_into()?;

            viuer::print(
                &image,
                &viuer::Config {
                    width: Some(80),
                    height: Some(40),
                    ..Default::default()
                },
            )?;
        }

        #[cfg(not(feature = "image"))]
        image_buffer.write(Utf8Path::new("target/resize.exr"))?;

        Ok(())
    }
}
