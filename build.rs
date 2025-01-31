#[cfg(windows)]
extern crate windres;

fn main() {
    #[cfg(debug_assertions)]
    unsafe {
        std::env::set_var("CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG", "true");
    }

    #[cfg(not(debug_assertions))] // Release mode
    if cfg!(target_os = "windows") {
        let name = env!("CARGO_PKG_NAME");
        let vers = env!("CARGO_PKG_VERSION");

        let mut res = winresource::WindowsResource::new();
        // res.set_icon("res/icon.ico").set_language(0x0009); // English
        // .set_language(0x0409); // English (US)
        res.set("CompanyName", "https://github.com/MrDwarf7");
        res.set("ProductName", name);

        let mut sv = vers.split('.').collect::<Vec<_>>();
        while sv.len() < 4 {
            sv.push("0");
        }

        let file_version = format!("{}, {}, {}, {}", sv[0], sv[1], sv[2], sv[3]);
        res.set("FileVersion", file_version.as_str());

        // windres::Build::new()
        //     .define("THE_PROJECT", Some(format!(r#""{name}"#).as_str()))
        //     .define("THE_VERSION", Some(format!(r#""{vers}"#).as_str()))
        //     .define("THE_FILEVESION", Some(file_version.as_str()))
        //     .compile("res/resource.rc").expect("Failed to compile resource file");
        // for entry in std::fs::read_dir("res").expect("Failed to read directory") {
        //     let entry = entry.expect("Failed to read entry");
        //     println!("cargo:rerun-if-changed={},", entry.path().display());
        // }

        res.compile().expect("Failed to compile resource file");
    }
}
