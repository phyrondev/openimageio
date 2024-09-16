use bbl_build::Config;
use log::error;

pub fn main() {
    println!("cargo:rerun-if-changed=bbl-oiio/*");

    if std::env::var("BBL_PLUGIN_PATH").is_err() {
        error!("BBL_PLUGIN_PATH is no set");
    }

    #[cfg(target_os = "linux")]
    println!("cargo::rustc-link-arg=-lstdc++");

    if let Ok(cmake_install_prefix) = std::env::var("CMAKE_INSTALL_PREFIX") {
        println!("cargo:rustc-link-search={}/lib", cmake_install_prefix);
    }

    let _dst = Config::new("oiio", "bbl-oiio")
        .define("BBL_LANGUAGES", "rust")
        .build();
}
