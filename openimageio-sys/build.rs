use bbl_build::Config;
use log::error;
use std::{env, fs, path::PathBuf};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=bbl-oiio/*");

    let regenerate_bindings = env::var("OIIO_REGENERATE").is_ok();

    // We only re-generate the bindings if the environment variable
    // OIIO_REGENERATE is set.
    //println!("cargo::rustc-check-cfg=cfg(builtin_bindings)");
    if !regenerate_bindings {
        //println!("cargo:rustc-cfg=builtin_bindings");
        return Ok(());
    }

    if std::env::var("BBL_PLUGIN_PATH").is_err() {
        error!("BBL_PLUGIN_PATH is no set");
    }

    #[cfg(target_os = "linux")]
    println!("cargo::rustc-link-arg=-lstdc++");

    if let Ok(cmake_install_prefix) = std::env::var("CMAKE_INSTALL_PREFIX") {
        println!("cargo:rustc-link-search={}/lib", cmake_install_prefix);
    }

    let bindings_path = Config::new("oiio", "bbl-oiio")
        .define("BBL_LANGUAGES", "rust")
        .build()?
        .join("build/oiio.rs");

    if regenerate_bindings {
        let destination = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("bindings");

        let _ = fs::create_dir_all(&destination);

        fs::copy(bindings_path, destination.join("oiio.rs"))?;
    }

    Ok(())
}
