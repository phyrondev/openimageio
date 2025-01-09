# `openimageio` – High Level Rust Wrapper for [OpenImageIO](https://github.com/AcademySoftwareFoundation/OpenImageIO)

OpenImageIO (OIIO) is a toolset for reading, writing, and manipulating image
files of any image file format relevant to VFX/animation via a format-agnostic
API with a feature set, scalability, and robustness needed for feature film
production.

The primary target audience for OIIO is VFX studios and developers of tools
such as renderers, compositors, viewers, and other image-related software you'd
find in a production pipeline.

OIIO consists of:

- Simple but powerful `ImageInput` and `ImageOutput` APIs that provide an
  abstraction for reading and writing image files of nearly any format, without
  the calling application needing to know any of the details of these file
  formats, and indeed without the calling application needing to be aware of
  which formats are available.

- A library that manages subclasses of `ImageInput` and `ImageOutput` that
  implement I/O from specific file formats, with each file format's
  implementation stored as a plug-in. Therefore, an application using OIIO's
  APIs can read and write any image file for which a plugin can be found at
  runtime.

- Plugins implementing I/O for several popular image file formats, including
  TIFF, JPEG/JFIF, JPEG XL, OpenEXR, PNG, HDR/RGBE, ICO, BMP, Targa, JPEG-2000,
  RMan Zfile, FITS, DDS, Softimage PIC, PNM, DPX, Cineon, IFF, OpenVDB, Ptex,
  Photoshop PSD, Wavefront RLA, SGI, WebP, GIF, DICOM, HEIF/HEIC/AVIF, many
  "RAW" digital camera formats, and a variety of movie formats (readable as
  individual frames). More are being developed all the time.

- An `ImageCache` class that transparently manages a cache so that it can
  access truly vast amounts of image data (tens of thousands of image files
  totaling multiple TB) very efficiently using only a tiny amount (tens of
  megabytes at most) of runtime memory.

- A `TextureSystem` class that provides filtered MIP-map texture lookups, atop
  the nice caching behavior of `ImageCache`. This is used in commercial
  renderers and has been used on many large VFX and animated films.

- `ImageBuffer` – a simple class for storing and manipulating whole images in
  memory and a collection of the most useful computations you might want to do
  involving those images, including many image processing operations.

## Example

```rust
use anyhow::Result;
use openimageio::{FromFileOptions, ImageCache, ImageBuffer, Utf8Path};

fn main() -> Result<()> {
    // Create an app-global, shared cache that will persist after this
    // instance gets dropped.
    //
    // I.e. there is only one such shared cache per process.
    let image_cache = ImageCache::shared(
        // Persist cache accross this process.
        true
    );

    // Load fg image. This is 1024×1024.
    // The `_with` variants of OIIO methods take additional parameters.
    let mut image_fg = ImageBuffer::from_file_with(
        &Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"),
        &FromFileOptions {
            image_cache,
            ..Default::default()
        },
    )?;

    // Load bg image. This is 2048×1024.
    let image_bg = ImageBuffer::from_file(
        &Utf8Path::new("assets/wooden_lounge_2k__F32_RGBA.exr"),
    )?;

    // Compose fg over bg, replacing the data window of fg
    // with the result. I.e. the result will be cropped at
    // fg's original dimensions of 1024×1024.
    //
    // Note that for this variant the order of the arguments is
    // swapped. I.e. image_b
    image_bg.over(&image_fg)?;

    // Write the result.
    image_bg.write(&Utf8Path::new("fg_over_bgexr"))?;

    // Alternatively we could combine both images and get a buffer
    // with the combined data windows.
    let combined_image = ImageBuffer::from_over(image_fg, image_bg)?;

    combined_image.write(&Utf8Path::new("from_fg_over_bg.exr"))?;

    // `image_cache` gets dropped here but the shared cache still exists
    // and can be accessed by creating a new `ImageCache::shared(true)`
    // which would give access this same instance.

    Ok(())
}
```

## Features

- `algorithms` – Add a lot of useful `ImageBuffer` algorithms (enabled by default).

- `ffi` – exposes the low-level FFI bindings to OIIO.

## Building

### Dependencies

- We currently build against/require OIIO `v3.x`.

- [`babble`](https://github.com/anderslanglands/babble).

  As of this writing `openimageio` depends on `babble` ≥ `v0.8` and the
  latest binary release is `v0.6`. As such you need to
  [build `babble` from source](https://github.com/anderslanglands/babble?tab=readme-ov-file#building-babble-from-source).

#### Debian/Ubuntu

`clang` ≥ `v17` is suggested, i.e. if neccessary:

```
sudo apt install clang-17 libclang-17-dev
```

The rest of the dependencies (note that latest Ubuntu is still on `v2.4` – you
may need to build from source):

```
sudo apt install cmake libimath-dev libopenimageio-dev
```

### Prerequisites

Make sure `BBL_PLUGIN_PATH` is set to where the Rust plugin for `babble` can be
found.
Otherwise the last step of code-generation silently fails and the Rust bindings
file is never created.

On a Linux system a typical location would be `/usr/local/plugins/libbbl-rust`.

I.e. you'd have:

```shell
export BBL_PLUGIN_PATH=/usr/local/plugins/
```

> ⚠️ After you set this you must run `cargo clean` and re-build (as the binding's
> `build.rs` only looks for changes in the input to trigger re-runs, not in the
> output)!

If you use a local build of OpenImageIO you can point CMake to it by setting
the `OIIO_DIST` to the `dist` directory of your build.

For example:

```
OIIO_DIST=$HOME/code/OpenImageIO/dist cargo test
```
