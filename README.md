# `oiio` -- High Level Rust Wrapper for [OpenImageIO](https://github.com/AcademySoftwareFoundation/OpenImageIO)

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
