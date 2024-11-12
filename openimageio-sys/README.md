# `openimageio-sys`

[Babble](https://github.com/anderslanglands/babble) auto-generated Rust bindings
to the [OpenImageIO](https://openimageio.org/) (OIIO) C++ library.

## Building

The crate ships with pre-geneated bindings in `bindings/oiio.rs` that are updated for _any non-`doc`_ build unless the `OIIO_DO_NOT_GENERATE_BINDINGS` environment variable is set (see [below](#building-without-openimageio)).

> _The bindings are overwritten in this case._
>
> I.e. `git` may show you `bindings/oiio.rs` as changed.

CMake will try to find OIIO using the `find_package` mechanism. This can be
overridden by using a custom OIIO location, see next section.

### Using a Custom OpenImageIO Distribution

If you have a custom OpenImageIO distribution, you can use the `OIIO_DIST` environment variable to point to the root of it:

```sh
OIIO_DIST=/path/to/oiio_dist cargo build
```

### Building Without OpenImageIO

If you do not have an OIIO distribution installed, you can disable the babble part of the build by setting the OIIO_DO_NOT_GENERATE_CPP_API environment variable:

```sh
OIIO_DO_NOT_GENERATE_BINDINGS=1 cargo build
```

This can be useful if you just want to make sure whatever changes you did in your OIIO-dependent code do build.

> _The build will **fail** at the linking stage if:_
>
> - OIIO is not found.
> - The OIIO version installed on your system is not compatible with the
>   version of OIIO the bindings shipped by this crate were generated against
>   (v2.5.x).

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0).
