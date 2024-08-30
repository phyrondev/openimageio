use bbl_build::Config;
use log::error;

pub fn main() {
    if std::env::var("BBL_PLUGIN_PATH").is_err() {
        error!("BBL_PLUGIN_PATH is no set");
    }

    #[cfg(target_os = "linux")]
    println!("cargo::rustc-link-arg=-lstdc++");

    let _dst = Config::new("oiio", "bbl-oiio")
        .define("BBL_LANGUAGES", "rust")
        .build();

    println!("cargo:rerun-if-changed=bbl-oiio");
}
