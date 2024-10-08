# `openimageio-sys`

[Babble](https://github.com/anderslanglands/babble) auto-generated Rust bindings
to the [OpenImageIO](https://openimageio.org/) (OIIO) C++ library.

## Building

The crate ships with pre-geneated bindings in `bindings/oiio.rs` that are used
as-is for `doc` builds or if the `OIIO_DO_NOT_GENERATE_CPP_API` environment
variable is set (see [below](#building-without-openimageio)).

These are overwritten when the crate builds normally. I.e. `git` may show you
`bindings/oiio.rs` as changed.

A 'normal' build will generate both the bindings as well as the C FFI wrappers
for the OIIO C++ headers that `babble` will also compile.

CMake will try to find OIIO using the `find_package` mechanism. This can be
overridden by using a custom prefix, see next section.

### Using a Custom OpenImageIO Distribution

If you have a custom OpenImageIO distribution, you can use the
`CMAKE_INSTALL_PREFIX` environment variable to point to the root of the
this:

```sh
CMAKE_INSTALL_PREFIX=/path/to/oiio cargo build
```

### Building Without OpenImageIO

If you do not have an OIIO distribution installed, you can disable the `babble`
part of the build by setting the `OIIO_DO_NOT_GENERATE_CPP_API` environment
variable:

```sh
OIIO_DO_NOT_GENERATE_CPP_API=1 cargo build
```

This can be useful if you just want to make sure whatever changes you did in
your code depending on OIIO do build.

_But it does mean the build will **fail** at the linking stage._

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0).
