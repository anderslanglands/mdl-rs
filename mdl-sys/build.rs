use cmake;
use std::collections::HashMap;
use std::path::Path;

pub fn main() {
    let mut settings = config::Config::default();
    let mdl_root = std::env::var("MDL_ROOT")
        .expect("MDL_ROOT env var must be set to root of MDL installation");

    let inc_mdl = Path::new(&mdl_root).join("include");

    let dst_capi = cmake::Config::new("mdl-capi")
        .define("INC_MDL", &inc_mdl)
        .always_configure(false)
        .build();

    println!("cargo:rustc-link-search=native={}", dst_capi.display());

    println!("cargo:rustc-link-lib=static=mdl-capi");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");

}
