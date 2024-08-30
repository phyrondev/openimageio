#![feature(str_from_raw_parts)]
#![feature(c_size_t)]
#![doc(
    html_logo_url = "https://artwork.aswf.io/projects/openimageio/icon/color/openimageio-icon-color.svg"
)]
//! Abbreviated as *OIIO* -- a library for reading, writing, and processing
//! images in a wide variety of file formats, using a format-agnostic API. OIIO
//! emphasizes formats and functionality used in professional, large-scale
//! visual effects and feature film animation, and it is used ubiquitously by
//! large VFX studios, as well as incorporated into many commercial products.
//!
//! This crate exposes most of the C++ API in an oxidized wrapper. See
//! [below](#differences-to-the-c-api) for differences between the C++ library
//! and their motivations.
//!
//! # Introduction
//!
//! Welcome to OpenImageIO!
//!
//! > I kinda like "Oy-e-oh" with a bit of a groaning Yiddish accent, as in
//! > "OIIO, did you really write yet another file I/O library?"
//! >
//! > ---Dan Wexler
//!
//! ## Overview
//!
//! OpenImageIO provides simple but powerful [`ImageInput`] and [`ImageOutput`]
//! APIs that abstract the reading and writing of 2D image file formats. They
//! don't support every possible way of encoding images in memory, but for a
//! reasonable and common set of desired functionality, they provide an
//! exceptionally easy way for an application using the APIs support a wide --
//! and extensible -- selection of image formats without knowing the details of
//! any of these formats.
//!
//! Concrete instances of these APIs, each of which implements the ability to
//! read and/or write a different image file format, are stored as plugins
//! (i.e., dynamic libraries, DLL's, or DSO's) that are loaded at runtime. The
//! OpenImageIO distribution contains such plugins for several popular formats.
//! Any user may create conforming plugins that implement reading and writing
//! capabilities for other image formats, and any application that uses
//! OpenImageIO would be able to use those plugins.
//!
//! The library also implements the helper type [`ImageBuffer`], which is a
//! handy way to store and manipulate images in memory. `ImageBuffer` itself
//! uses `ImageInput` and `ImageOutput` for its file I/O, and therefore is
//! also agnostic as to image file formats. A variety of functions in the
//! [`algorithms`] module are available to perform common image processing
//! operations on ImageBuf's.
//!
//! [`ImageCache`] transparently manages a cache so that it can access truly
//! vast amounts of image data (thousands of image files totaling hundreds of GB
//! to several TBs) very efficiently using only a tiny amount (tens of megabytes
//! to a few GB at most) of runtime memory. Additionally, [`TextureSystem`]
//! provides filtered MIP-map texture lookups, atop the nice caching behavior of
//! `ImageCache`.
//!
//! ## Simplifying Assumptions
//!
//! OpenImageIO is not the only image library in the world. Certainly there are
//! many fine libraries that implement a single image format (including the
//! excellent `libtiff`, `libjpeg`, and `OpenEXR` that OpenImageIO itself relies
//! on). Many libraries attempt to present a uniform API for reading and writing
//! multiple image file formats. Most of these support a fixed set of image
//! formats, though a few of these also attempt to provide an extensible set by
//! using the plugin approach.
//!
//! But in our experience, these libraries are all flawed in one or more ways:
//! (1) They either support only a few formats, or many formats but with the
//! majority of them somehow incomplete or incorrect. (2) Their APIs are not
//! sufficiently expressive as to handle all the image features we need (such as
//! tiled images, which is critical for our texture library). (3) Their APIs are
//! too complete, trying to handle every possible permutation of image format
//! features, and as a result are horribly complicated.
//!
//! The third sin is the most severe, and is almost always the main problem at
//! the end of the day. Even among the many open source image libraries that
//! rely on extensible plugins, we have not found one that is both sufficiently
//! flexible and has APIs anywhere near as simple to understand and use as those
//! of OpenImageIO.
//!
//! Good design is usually a matter of deciding what not to do, and OpenImageIO
//! is no exception. We achieve power and elegance only by making simplifying
//! assumptions. Among them:
//!
//! * OpenImageIO only deals with ordinary 2D images, and to a limited extent 3D
//!   volumes, and image files that contain multiple (but finite) independent
//!   images within them. OpenImageIO's support of “movie” files is limited to
//!   viewing them as a sequence of separate frames within the file, but not as
//!   movies per se (for example, no support for dealing with audio or
//!   synchronization).
//!
//! * Pixel data are presented as 8- 16- or 32-bit integers (signed or
//!   unsigned), 16- 32- or 64-bit float. *Nothing else*. No bit images, or
//!   pixel value boundaries that aren't byte boundaries. Files with will appear
//!   to the client application as 8-bit unsigned grayscale images.
//!
//! * Only fully elaborated, non-compressed data are accepted and returned by
//!   the API. Compression or special encodings are handled entirely within an
//!   OpenImageIO plugin.
//!
//! * Color space is by default converted to grayscale or RGB. Non-spectral
//!   color models, such as XYZ, CMYK, or YUV, are converted to RGB upon reading
//!   (though there is a way to override this and ask for raw pixel values).
//!
//! * All color channels can be treated (by apps or readers/writers) as having
//!   the same data format (though there is a way to deal with per-channel
//!   formats for apps and readers/writers that truly need it).
//!
//! * All image channels in a subimage are sampled at the same resolution. For
//!   file formats that allow some channels to be subsampled, they will be
//!   automatically up-sampled to the highest resolution channel in the
//!   subimage.
//!
//! * Color information is always in the order R, G, B, and the alpha channel,
//!   if any, always follows RGB, and Z channel (if any) always follows alpha.
//!   So if a file actually stores ABGR, the plugin is expected to rearrange it
//!   as RGBA.
//!
//! It's important to remember that these restrictions apply to data passed
//! through the APIs, not to the files themselves. It's perfectly fine to have
//! an OpenImageIO plugin that supports YUV data, or 4 bits per channel, or any
//! other exotic feature. You could even write a movie-reading ImageInput
//! (despite OpenImageIO's claims of not supporting movies) and make it look to
//! the client like it's just a series of images within the file. It's just that
//! all the nonconforming details are handled entirely within the OpenImageIO
//! plugin and are not exposed through the main OpenImageIO APIs.
//!
//! ## Differences to the C++ API
//!
//! ### Naming
//!
//! * Type- and function names were changed to adhere to the official [Rust API Naming](https://rust-lang.github.io/api-guidelines/naming.html)
//!   guidelines and [RFC 344](https://github.com/rust-lang/rfcs/blob/master/text/0344-conventions-galore.md).
//!
//!   That said --, for all types were names were changed the crate ships with
//!   `type` aliases that mirror the original C++ names as much as possible
//!   within the constraints referenced in the previous paragraph.
//!
//!   * Abbreviations were removed to make naming more stringent across the API.
//!     For example `ImageBuf` (C++) became `ImageBuffer` (Rust) (an `ImageBuf`
//!     `type` alias is available).
//!
//!     The C++ API uses abbreviations sometimes and other times not. It is not
//!     obvious why, especially to a non-native speaker.
//!
//!     For example, the word `Output`, like `Buffer`, is six characters long.
//!     Yet the resp. class in the C++ API is called `ImageOutput`, not
//!     `ImageOut` (as the abbreviation in `ImageBuf` would have suggested),
//!     etc.
//!
//!   * Acronyms were spelled out to make code easier to read for developers
//!     coming from a non-VFX background. For example `ROI` (C++) became
//!     `RegionOfInterest` (Rust). However, there is a type alias, `Roi` for
//!     those familiar with the acronym (note the capitalization change to
//!     adhere to
//!     [aforementioned guidelines](https://rust-lang.github.io/api-guidelines/naming.html)).
//!
//!   * The ubiquitous `n`/`num`-prefixes (C++) meaning 'number (of)' were
//!     replaced by a `_count` suffix (Rust). For example `nthreads` (C++)
//!     becomes `thread_count` (Rust).
//!
//!   * Contractions were broken up with underscores. I.e. `filterwidth` (C++)
//!     becomes `filter_width` (Rust).
//!
//! * Rust does not have named parameters and quite a few OIIO C++ methods have
//!   variants that take a lot of those (with defaults, if omitted).
//!
//!   Wrapping each of these in an `Option` doesn't make for very readable code.
//!   This is because of the absence of parameter names at call sites and as
//!   they still have to specified, even if omitted (one `None` per each).
//!
//!   The [builder pattern](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)
//!   is mostly used for opaque/side-effects-free `struct` initalization in the
//!   wild and quite verbose. But it is seldom used for optional function
//!   parameters. It also requires a lot of boilerplate code.
//!
//!   Instead the
//!   [init-struct pattern](https://xaeroxe.github.io/init-struct-pattern/)
//!   was chosen.
//!
//!   It requires almost no boilerplate, parameter names are clearly spelled
//!   out on initialization/assignment and the compiler will optimize most/all
//!   of it away (zero cost abstractions).
//!
//!   For example, the [`ImageBuffer::rotate()`] method has a useful C++ variant
//!   taking *five* extra parameters.
//!
//!   On the Rust side we expose a simple version,
//!   [`rotate()`](ImageBuffer::rotate) but also an equivalent,
//!   [`rotate_with()`](ImageBuffer::rotate_with), that takes a single
//!   [`RotateOptions`](operators::RotateOptions) parameter with the
//!   aforementioned five parameters.
//!
//!   Specifying each of these can be (partially) omitted by using
//!   `Default::default()` `struct`-initialization shorthand:
//!
//!   ```
//!   # use openimageio::{ImageBuffer, operators::{
//!   #     PixelFilter::BlackmanHarris, RotateOptions
//!   # }, Utf8Path};
//!   # use std::f32::consts::TAU;
//!   let mut image_buf = ImageBuffer::from_file(Utf8Path::new(
//!       "assets/wooden_lounge_2k__F32_RGBA.exr",
//!   ));
//!
//!   image_buf.rotate_with(
//!       42.0 * TAU / 360.0,
//!       &RotateOptions {
//!           // Use a Blackmann-Harris filter to avoid halos easily introduced
//!           // when operating on HDRs using the default filter, Lanczos3.
//!           pixel_filter: BlackmanHarris,
//!           ..Default::default()
//!       },
//!   );
//!   ```
//!
//! ## Opaque Types
pub use camino::{Utf8Path, Utf8PathBuf};

mod ffi;
#[cfg(feature = "ffi")]
pub use ffi::*;
#[cfg(not(feature = "ffi"))]
pub(crate) use ffi::*;

mod file_system;
pub use file_system::*;

mod image_buffer;
pub use image_buffer::*;

mod image_cache;
pub use image_cache::*;

mod image_specification;
pub use image_specification::*;

mod misc;
pub(crate) use misc::*;

mod region_of_interest;
pub use region_of_interest::*;

mod string;
pub(crate) use string::*;

mod string_view;
pub(crate) use string_view::*;

mod texture;
pub use texture::*;

mod type_description;
pub use type_description::*;

mod ustring;
pub use ustring::*;
