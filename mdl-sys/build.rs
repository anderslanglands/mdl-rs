use cmake;
use std::collections::HashMap;
use std::path::Path;

pub fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("build-settings").required(false))
        .unwrap();
    settings.merge(config::Environment::new()).ok();

    let settings_map = settings
        .try_into::<HashMap<String, String>>()
        .unwrap_or(HashMap::new());

    if settings_map.contains_key("mdl_root")
    {
        let mdl_root = &settings_map["mdl_root"];

        let inc_mdl = Path::new(&mdl_root).join("include");

        let dst_capi = cmake::Config::new("mdl-capi")
            .define("INC_MDL", &inc_mdl)
            .always_configure(false)
            .build();

        println!("cargo:rustc-link-search=native={}", dst_capi.display());

        #[cfg(target_os = "linux")]
        println!("cargo:rustc-link-lib=dylib=stdc++");
        #[cfg(target_os = "macos")]
        println!("cargo:rustc-link-lib=dylib=c++");

        println!("cargo:rustc-link-lib=static=mdl-capi");
    } else {
        panic!("MDL_ROOT must be set to root of MDL installation");
    }
}
