use crate::*;
use anyhow::anyhow;
use std::mem::MaybeUninit;

pub trait AsTypeDesc {
    fn as_type_desc(&self) -> TypeDesc;
}

impl AsTypeDesc for &str {
    fn as_type_desc(&self) -> TypeDesc {
        TypeDesc::STRING
    }
}

impl AsTypeDesc for bool {
    fn as_type_desc(&self) -> TypeDesc {
        TypeDesc::U8
    }
}

impl AsTypeDesc for i32 {
    fn as_type_desc(&self) -> TypeDesc {
        TypeDesc::I32
    }
}

/// Sets global attributes (i.e., properties or options) of OIIO.
///
/// The name designates the name of the attribute, type describes the type of
/// data, and val is a pointer to memory containing the new value for the
/// attribute.
///
/// If the name is known, valid attribute that matches the type specified, the
/// attribute will be set to the new value and attribute() will return true. If
/// name is not recognized, or if the types do not match (e.g., type is
/// `f32` but the named attribute is a `String`), the attribute will not be
/// modified, and `set_attributes()` will return an error.
///
/// ```
/// # use openimageio::set_attributes;
/// set_attributes("threads=4,log_times=1");
/// ```
///
/// Note that if an option takes a string value that must itself contain a
/// comma, it is permissible to enclose the value in either single (`'`) or
/// double (`"`) quotes.
///
/// The following are the recognized attributes:
///
/// - `threads` (`i32`): How many threads to use for operations that can be sped
///   up by being multithreaded.
///
///   Default: `0`
///
///   Examples: simultaneous format conversions of multiple scanlines read
///   together, or many [`algorithms`](image_buffer::algorithms) operations.)
///   The default of `0` means to use the full available hardware concurrency
///   detected.
///
///   Situations where the main application logic is essentially single threaded
///   (i.e., one top-level call into OIIO at a time) should leave this at the
///   default value, or some reasonable number of cores, thus allowing lots of
///   threads to fill the cores when OIIO has big tasks to complete. But
///   situations where you have many threads at the application level, each of
///   which is expected to be making separate OIIO calls simultaneously, should
///   set this to `1`, thus having each calling thread do its own work inside of
///   OIIO rather than spawning new threads with a high overall 'fan out.'
///
/// - `exr_threads` (`i32`): The internal OpenEXR thread pool size.
///
///   Default: `0`
///
///   The default of `0` means is to use as many threads as the amount of
///   hardware concurrency detected. Note that this is separate from the OIIO
///   `threads` attribute above.
///
/// - `font_searchpath` (`String`) Colon-/semicolon-separated list of
///   directories to search if fonts are needed.
///
///   Such as for [`ImageBuffer::render_text()`].
///
/// - `use_tbb` (`i32`): If nonzero and TBB was found and support configured
///   when OIIO was compiled, parallel processing within OIIO (including inside
///   the `parallel.h` utilities) will try to use TBB by default where possible.
///
///   Default: ?
///
///   If zero, they will try to use OIIO’s native thread pool even if TBB is
///   available.
///
/// - `plugin_searchpath` (`String`)
///
///   Colon-/semicolon-separated list of directories to search for
///   dynamically-loaded format plugins.
///
/// - `try_all_readers` (`i32`): Wether to try all availabe readers if the
///   guessed reader fails.
///
///   Default: `0`
///
///   When nonzero, a call to `ImageInput::create()` or `ImageInput::open()`
///   that does not succeed in opening the file with the format reader implied
///   by the file extension will try all available format readers to see if one
///   of them can open the file.
///
///   If this is zero, the only reader that will be tried is the one implied by
///   the file extension.
///
/// - `read_chunk` (`i32`)
///
///   When performing a `read_image()`, this is the number of scanlines it will
///   attempt to read at a time (some formats are more efficient when reading
///   and decoding multiple scanlines). The default is 256. The special value of
///   `0` indicates that it should try to read the whole image if possible.
///
/// - `missingcolor` ([f32], &str): Color used for missing data.
///
///   Default: not set
///
///   This attribute may either be a slice of `f32` values, or a string
///   containing a comma-separated list of the values. Setting this option
///   globally is equivalent to always passing an `ImageInput`
///   open-with-configuration hint `oiio:missingcolor` with the value.
///
///   When set, it gives some `ImageInput` readers the option of ignoring any
///   missing tiles or scanlines in the file, and instead of treating the read
///   failure of an individual tile as a full error, will interpret is as an
///   intentionally missing tile and proceed by simply filling in the missing
///   pixels with the color specified. If the first element is negative, it will
///   use the absolute value, but draw alternating diagonal stripes of the
///   color. For example:
///
///   ```ignore
///   let missing = [ -1.0f32, 0.0, 0.0, 0.0 ]; // striped red
///   set_attribute("missingcolor", &missing);
///   ```
///
///   Note that only some file formats support files with missing tiles or
///   scanlines, and this is only taken as a hint. Please see the file format
///   plugins for details on which formats accept a `missingcolor` configuration
///   hint.
///
/// - `debug` (`i32`): When nonzero, various debug messages may be printed.
///
///   Default: `0` (for `release` builds), `1` (for `debug` builds)
///
///   Values > `1` are for OIIO developers to print even more debugging
///   information. This attribute but also may be overridden by the
///   `OPENIMAGEIO_DEBUG` environment variable.
///
/// - `tiff:half` (`i32`): When nonzero, allows TIFF to write half pixel data.
///
///   Default: `0`
///
///   > Most apps may not read these correctly, but OIIO will.
///   > That's why the default is not to support it.
///
/// - `dds:bc5normal` (`i32`): When nonzero, treats BC5/ATI2 format files as
///   normal maps (loads as 3 channels, computes blue from red and green).
///
///   Default: `0`.
///
/// - `openexr:core` (`i32): When nonzero, use the new :OpenEXR core C library""
///   when available, for OpenEXR >= 3.1.
///
///   Default: `0`
///
///   This is experimental.
///
/// - `jpeg:com_attributes` (`i32`)
///
///   Default: `1`
///
///   When nonzero, try to parse JPEG comment blocks as key-value attributes,
///   and only set ImageDescription if the parsing fails. Otherwise, always set
///   `ImageDescription` to the first comment block.
///
/// - `limits:channels` (`u16`): When nonzero, the maximum number of color
///   channels in an image.
///
///   Default: `1024`
///
///   Image files whose headers indicate they have more channels might be
///   assumed to be corrupted or malicious files. In situations when more
///   channels are expected to be encountered, the application should raise this
///   limit.
///
/// - `limits:imagesize_MB` (`u32`):  When nonzero, the maximum expected size in
///   MB of the uncompressed pixel data of a single 2D image.
///
///   Default: `32768`
///
///   Images whose headers indicate that they are larger than this might be
///   assumed to be corrupted or malicious files.
///
///   The default is 32768 (32 GB of uncompressed pixel data --; equivalent to
///   64k×64k×channel×half), or the total amount of total physical memory
///   available to the running process, whichever is smaller. In situations when
///   images larger than this are expected to be encountered, you should raise
///   this limit.
///
///   Setting the limit to `0` means having *no limit*.
///
/// - `log_times` (`i32`):
///
///   Default: `0`
///
///   When nonzero, the `image_buffer::algorithms` functions are instrumented to
///   record the number of times they were called and the total amount of time
///   spent executing them. It can be overridden by environment variable
///   `OPENIMAGEIO_LOG_TIMES`.
///
///   The totals will be recorded and can be retrieved as a string by using
///   `attribute("timing_report", ...)`. Additionally, if the value is `2` or
///   more, the timing report will be printed to `stdout` upon application exit
///   (not advised in contexts where it isn’t ok to print to the terminal via
///   `stdout`, such as GUI apps or libraries).
///
///   When enabled, there is a slight runtime performance cost due to checking
///   the time at the start and end of each of those function calls, and the
///   locking and recording of the data structure that holds the log
///   information. When the log_times attribute is disabled, there is no
///   additional performance cost.
///
/// - `oiio:print_uncaught_errors (1)
///
///   If nonzero, upon program exit, any error messages that would have been
///   retrieved by a call to OIIO::geterror(), but never were, will be printed
///   to stdout. While this may seem chaotic, we are presuming that any
///   well-written library or application will proactively check error codes and
///   retrieve errors, so this will never print anything upon exit. But for less
///   sophisticated applications (or users), this is very useful for forcing
///   display of error messages so that users can see relevant errors even if
///   they never check them explicitly, thus self-diagnose their troubles before
///   asking the project dev deam for help. Advanced users who for some reason
///   desire to neither retrieve errors themselves nor have them printed in this
///   manner can disable the behavior by setting this attribute to `0`.
///
/// - `imagebuf:print_uncaught_errors` (1):
///
///   If nonzero, an ImageBuf upon destruction will print any error messages
///   that were never retrieved by its geterror() method. While this may seem
///   chaotic, we are presuming that any well-written library or application
///   will proactively check error codes and retrieve errors, so will never
///   print anything upon destruction. But for less sophisticated applications
///   (or users), this is very useful for forcing display of error messages so
///   that users can see relevant errors even if they never check them
///   explicitly, thus self-diagnose their troubles before asking the project
///   dev deam for help. Advanced users who for some reason desire to neither
///   retrieve errors themselves nor have them printed in this manner can
///   disable the behavior by setting this attribute to `0`.
///
/// - `imagebuf:use_imagecache` (`i32`): If nonzero, an `ImageBuffer` that
///   references a file but is *not* given an `ImageCache` will read the image
///   through the default `ImageCache`.
///
///   Default: `0`
pub fn set_attributes(token_values: &str) -> Result<()> {
    let ok = {
        let mut is_ok = MaybeUninit::<bool>::uninit();

        unsafe {
            oiio_set_attribute(
                StringView::from("options").as_raw_ptr() as _,
                TypeDesc::STRING.into(),
                OiioString::new(token_values).as_raw_ptr() as _,
                &raw mut is_ok as _,
            );

            is_ok.assume_init()
        }
    };

    if ok {
        Ok(())
    } else {
        Err(anyhow!(
            "Failed to set attribute 'options' to '{}'",
            token_values
        ))
    }
}

/*pub fn set_attribute(name: &str, value: &impl AsTypeDesc) -> Result<()> {
    if set_attribute_ffi(name, value) {
        Ok(())
    } else {
        Err(anyhow!("Failed to set attribute '{}'", name))
    }
}

#[inline]
fn set_attribute_ffi(name: &str, value: &impl AsTypeDesc) -> bool {
    let mut is_ok = MaybeUninit::<bool>::uninit();

    unsafe {
        oiio_set_attribute(
            StringView::from(name).as_raw_ptr() as _,
            oiio_TypeDesc_t::from(value.as_type_desc()),
            &raw const value as *const _ as _,
            &raw mut is_ok as _,
        );

        is_ok.assume_init()
    }
}*/
