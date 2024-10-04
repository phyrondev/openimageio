# `openimageio-sys`

[Babble](https://github.com/anderslanglands/babble) auto-generated Rust bindings
to the [OpenImageIO](https://openimageio.org/) C++ library.

## Building

The crate ships with pre-geneated bindings. To regenerate the bindings, run:

```sh
OIIO_REGENERATE=1 cargo build
```

This will re-generate the `oiio.rs` bindings source in the `bindings/` directory.

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0).
