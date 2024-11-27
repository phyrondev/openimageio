use log::error;
use std::{env, fs, path::PathBuf};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    // Do not generate anything during a `doc` build or the user told us so.
    if cfg!(doc) || env::var("OIIO_DO_NOT_GENERATE_BINDINGS").is_ok() {
        return Ok(());
    }

    println!("cargo:rerun-if-changed=bbl-oiio/*");

    #[cfg(target_os = "linux")]
    println!("cargo::rustc-link-arg=-lstdc++");

    if let Ok(cmake_install_prefix) = std::env::var("CMAKE_INSTALL_PREFIX") {
        println!("cargo:rustc-link-search={}/lib", cmake_install_prefix);
    }

    // Build OIIO itself.

    /*
    let oiio_dist = cmake::Config::new("libopenimageio")
        .cflag("-B target/cmake-build")
        .cflag("-S OpenImageIO")
        .cflag("-D OpenImageIO_BUILD_MISSING_DEPS=all")
        .cflag("-D STOP_ON_WARNING=0")
        .build();

    let oiio_dist = cmake::Config::new("libopenimageio")
        .cflag("--build target/cmake-build")
        .cflag("--target install")
        .cflag(format!("-j{}"))
        .build();

    println!("{}", oiio_dist.display());*/

    // We only re-generate the bindings if the environment variable
    // OIIO_REGENERATE is set.
    //println!("cargo::rustc-check-cfg=cfg(generated_bindings)");

    if std::env::var("BBL_PLUGIN_PATH").is_err() {
        error!("BBL_PLUGIN_PATH is no set");
    }

    let mut bindings = bbl_build::Config::new("oiio", "bbl-oiio");

    bindings.define("BBL_LANGUAGES", "rust");

    if let Ok(oiio_dist) = env::var("OIIO_DIST") {
        bindings.define("OIIO_DIST", oiio_dist);
    }

    let bindings_path = bindings.build()?.join("build/oiio.rs");

    let destination = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("bindings");

    let _ = fs::create_dir_all(&destination);

    // Update the bindings file.
    fs::copy(bindings_path, destination.join("oiio.rs"))?;

    Ok(())
}
