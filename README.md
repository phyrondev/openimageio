# `oiio` -- High Level Rust Wrapper for [OpenImageIO](https://github.com/AcademySoftwareFoundation/OpenImageIO)

OpenImageIO (OIIO) is a toolset for reading, writing, and manipulating image
files of any image file format relevant to VFX/animation via a format-agnostic
API with a feature set, scalability, and robustness needed for feature film
production.

The primary target audience for OIIO is VFX studios and developers of tools
such as renderers, compositors, viewers, and other image-related software you'd
find in a production pipeline.

OpenImageIO consists of:

* Simple but powerful `ImageInput` and `ImageOutput` APIs that provide an
  abstraction for reading and writing image files of nearly any format, without
  the calling application needing to know any of the details of these file
  formats, and indeed without the calling application needing to be aware of
  which formats are available.

* A library that manages subclasses of `ImageInput` and `ImageOutput` that
  implement I/O from specific file formats, with each file format's
  implementation stored as a plug-in. Therefore, an application using OIIO's
  APIs can read and write any image file for which a plugin can be found at
  runtime.

* Plugins implementing I/O for several popular image file formats, including
  TIFF, JPEG/JFIF, JPEG XL, OpenEXR, PNG, HDR/RGBE, ICO, BMP, Targa, JPEG-2000,
  RMan Zfile, FITS, DDS, Softimage PIC, PNM, DPX, Cineon, IFF, OpenVDB, Ptex,
  Photoshop PSD, Wavefront RLA, SGI, WebP, GIF, DICOM, HEIF/HEIC/AVIF, many
  "RAW" digital camera formats, and a variety of movie formats (readable as
  individual frames). More are being developed all the time.

* An `ImageCache` class that transparently manages a cache so that it can
  access truly vast amounts of image data (tens of thousands of image files
  totaling multiple TB) very efficiently using only a tiny amount (tens of
  megabytes at most) of runtime memory.

* A `TextureSystem` class that provides filtered MIP-map texture lookups, atop
  the nice caching behavior of ImageCache. This is used in commercial renderers
  and has been used on many large VFX and animated films.

* `ImageBuf` -- a simple class for storing and manipulating whole images in
  memory, including a collection of the most useful computations you might want
  to do involving those images, including many image processing operations.

## Example

```rust
use anyhow::Result;
use camino::Utf8Path;
use oiio::{ImageBuf, ImageSpec};

fn main() -> Result<()> {
    let mut image_buf_a = ImageBuf::from_file(
        &Utf8Path::new("assets/j0.3toD__F16_RGBA.exr"),
        None,
        None,
        None,
        None::<ImageSpec>,
    );

    let image_buf_b = ImageBuf::from_file(
        &Utf8Path::new("assets/wooden_lounge_2k.exr"),
        None,
        None,
        None,
        None::<ImageSpec>,
    );

    image_buf_a.over(&image_buf_b, None, None);

    image_buf_a.write("over.exr")?;

    Ok(())
}
```

## Building

### Dependencies

Install [`babble`](https://github.com/anderslanglands/babble).

As of this writing `openimageio` depends on `babble` ≥ `v0.7` and the latest binary release
is `v0.6`. As such you need to [build `babble` from source](https://github.com/anderslanglands/babble?tab=readme-ov-file#building-babble-from-source).

#### Debian/Ubuntu

`clang` ≥ `v17` is suggested, i.e. if neccessary:

```
sudo apt install clang-17 libclang-17-dev
```

The rest of the dependencies:

```
sudo apt install cmake libimath-dev libopenimageio-dev
```

### Prerequites

Make sure `BBL_PLUGIN_PATH` is set to where the Rust plugin for `babble` can be
found.
Otherwise the last step of code-generation silently fails and the Rust bindings
file is never created.

On a Linux system a typical location would be `/usr/local/plugins/libbbl-rust`.

I.e. you'd have:

```shell
export BBL_PLUGIN_PATH=/usr/local/plugins/
```

> After you set this you must run `cargo clean` and re-build (as the binding's
> `build.rs` only looks for changes in the input to trigger re-runs, not in the
> output)!
