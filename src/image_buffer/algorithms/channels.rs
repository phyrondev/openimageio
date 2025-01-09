use crate::*;
use smallvec::SmallVec;

/// Optional parameters for [`ImageBuffer`]'s
/// [`replace_by_convolve_with()`](ImageBuffer::replace_by_convolve_with) and
/// [`convolve_with()`](ImageBuffer::convolve_with) methods.
#[derive(Clone, Default)]
pub struct ChannelsOptions {
    /// If `true` the channel names will be taken from the corresponding
    /// channels of the source image.
    ///
    /// Be careful with this, shuffling both channel ordering and their names
    /// could result in no semantic change at all, if you catch the drift.
    ///
    /// If `false` (the default), the resulting dst image will have default
    /// channel names in the usual order (`R`, `G`, etc.).
    pub shuffle_names: bool,
    /// See the [Multithreading](module@algorithms#multithreading) section
    /// in the [module@algorithms] module.
    pub thread_count: u16,
}

pub enum Channel<'a> {
    Constant(f32, Option<&'a str>),
    Index(u32, Option<&'a str>),
}

/// # Channels
///
/// Generic channel shuffling.
///
/// return (or store in dst) a copy of src, but with
/// channels in the order channelorder[0..channel_count-1] (or set to a constant
/// value, designated by channelorder[i] = -1 and having the fill value in
/// channelvalues[i]. In-place operation is allowed (i.e., dst and src the same
/// image, but an extra copy will occur).
impl ImageBuffer {
    #[named]
    pub fn replace_by_channels(
        &mut self,
        source: &ImageBuffer,
        order: &[Channel],
    ) -> Result<&mut Self> {
        let is_ok = self.channels_ffi(source, order, &ChannelsOptions::default());

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn replace_by_channels_with(
        &mut self,
        source: &ImageBuffer,
        order: &[Channel],
        options: &ChannelsOptions,
    ) -> Result<&mut Self> {
        let is_ok = self.channels_ffi(source, order, options);

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn channels(&mut self, order: &[Channel]) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.channels_ffi(self, order, &ChannelsOptions::default());
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }

    #[named]
    pub fn channels_with(
        &mut self,
        order: &[Channel],
        options: &ChannelsOptions,
    ) -> Result<&mut Self> {
        let mut image_buffer = ImageBuffer::new();
        let is_ok = image_buffer.channels_ffi(self, order, options);
        *self = image_buffer;

        self.mut_self_or_error(is_ok, function_name!())
    }
}

impl ImageBuffer {
    #[inline]
    fn channels_ffi(
        &mut self,
        source: &ImageBuffer,
        order: &[Channel],
        options: &ChannelsOptions,
    ) -> bool {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        let mut channel_index = SmallVec::<[i32; 8]>::new();
        let mut channel_value = SmallVec::<[f32; 8]>::new();
        let mut channel_name = SmallVec::<[OiioString; 8]>::new();
        let mut channel_name_ptr = SmallVec::<[*const oiio_String_t; 8]>::new();

        for channel in order {
            match channel {
                Channel::Index(idx, name) => {
                    channel_index.push(*idx as i32);
                    channel_value.push(0.0);
                    channel_name.push(OiioString::new(name.unwrap_or("")));
                    channel_name_ptr.push(channel_name.last().unwrap().as_raw_ptr());
                }
                Channel::Constant(value, name) => {
                    channel_index.push(-1);
                    channel_value.push(*value);
                    channel_name.push(OiioString::new(name.unwrap_or("")));
                    channel_name_ptr.push(channel_name.last().unwrap().as_raw_ptr());
                }
            }
        }

        unsafe {
            oiio_ImageBufAlgo_channels(
                self.as_raw_ptr_mut(),
                source.as_raw_ptr(),
                channel_index.len() as _,
                CspanI32::new(&channel_index).as_raw_ptr() as _,
                CspanF32::new(&channel_value).as_raw_ptr() as _,
                //CspanRawOiioString::new(&channel_name_ptr).as_raw_ptr() as _,
                CspanRawOiioString::default().as_raw_ptr() as _,
                options.shuffle_names,
                options.thread_count as _,
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
    fn channels() -> Result<()> {
        let mut image_buffer =
            ImageBuffer::from_file(Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"))?;

        image_buffer.channels(&[
            Channel::Index(0, Some("RadNotRed")),
            Channel::Index(2, None),
            Channel::Index(1, None),
            Channel::Constant(0.5, Some("AlphaMale")),
        ])?;

        image_buffer.write(Utf8Path::new("target/channels.exr"))?;

        Ok(())
    }
}
