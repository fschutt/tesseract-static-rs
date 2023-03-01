#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/*
extern crate bindgen;

#[cfg(target_os = "macos")]
use pkg_config;
use std::env;
use std::path::{PathBuf, Path};
use std::process::{Process, Stdio};
#[cfg(windows)]
use vcpkg;

// const MINIMUM_LEPT_VERSION: &str = "1.80.0";

#[cfg(windows)]
fn find_leptonica_system_lib() -> Option<String> {
    println!("cargo:rerun-if-env-changed=LEPTONICA_INCLUDE_PATH");
    println!("cargo:rerun-if-env-changed=LEPTONICA_LINK_PATHS");
    println!("cargo:rerun-if-env-changed=LEPTONICA_LINK_LIBS");

    let vcpkg = || {
        let lib = vcpkg::Config::new().find_package("leptonica").unwrap();

        let include = lib
            .include_paths
            .iter()
            .map(|x| x.to_string_lossy())
            .collect::<String>();
        Some(include)
    };

    let include_path = env::var("LEPTONICA_INCLUDE_PATH").ok();
    let link_paths = env::var("LEPTONICA_LINK_PATHS").ok();
    let link_paths = link_paths.as_deref().map(|x| x.split(','));
    let link_libs = env::var("LEPTONICA_LINK_LIBS").ok();
    let link_libs = link_libs.as_deref().map(|x| x.split(','));
    if let (Some(include_path), Some(link_paths), Some(link_libs)) =
        (include_path, link_paths, link_libs)
    {
        for link_path in link_paths {
            println!("cargo:rustc-link-search={}", link_path)
        }

        for link_lib in link_libs {
            println!("cargo:rustc-link-lib={}", link_lib)
        }

        Some(include_path)
    } else {
        vcpkg()
    }
}

// we sometimes need additional search paths, which we get using pkg-config
// we can use leptonica installed anywhere on Linux.
// if you change install path(--prefix) to `configure` script.
// set `export PKG_CONFIG_PATH=/path-to-lib/pkgconfig` before.
#[cfg(any(target_os = "macos", target_os = "linux"))]
fn find_leptonica_system_lib() -> Option<String> {
    let pk = pkg_config::Config::new().probe("lept").unwrap();
    // Tell cargo to tell rustc to link the system proj shared library.
    println!("cargo:rustc-link-search=native={:?}", pk.link_paths[0]);
    println!("cargo:rustc-link-lib=static=lept");

    let mut include_path = pk.include_paths[0].clone();
    // The include file used in this project has "leptonica" as part of
    // the header file already
    include_path.pop();
    Some(include_path.to_string_lossy().to_string())
}

#[cfg(all(not(windows), not(target_os = "macos"), not(target_os = "linux")))]
fn find_leptonica_system_lib() -> Option<String> {
    println!("cargo:rustc-link-lib=static=lept");
    None
}

fn download_leptonica() {
    let tempdir = Path::new(env!("CARGO_TARGET_DIR"));
    let _ = std::fs::create_dir_all(tempdir.join("third_party"));

        std::process::Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("https://github.com/DanBloomberg/leptonica.git")
        .current_dir(&tempdir.join("third_party"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
    
    std::process::Command::new("git")
        .arg("fetch")
        .arg("origin")
        .arg(LEPTONICA_COMMIT)
        .current_dir(&tempdir.join("third_party").join("leptonica"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    std::process::Command::new("git")
        .arg("checkout")
        .arg(LEPTONICA_COMMIT)
        .current_dir(&tempdir.join("third_party").join("leptonica"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
}

fn main() {

    let config = download_leptonica();
    let clang_extra_include = find_leptonica_system_lib();

    let mut bindings = bindgen::Builder::default().header("wrapper.h");

    if let Some(include_path) = clang_extra_include {
        bindings = bindings.clang_arg(format!("-I{}", include_path));
    }

    let bindings = bindings
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

*/
#[cfg(test)]
mod tests {
    use super::{pixFreeData, pixRead};
    use std::ffi::CStr;

    #[test]
    fn image_size() {
        unsafe {
            let image =
                pixRead(CStr::from_bytes_with_nul_unchecked(b"../test image.png\0").as_ptr());
            assert_eq!((*image).w, 1000);
            assert_eq!((*image).h, 500);
            pixFreeData(image);
        }
    }

    #[test]
    #[allow(path_statements)]
    fn defined_constants() {
        super::IFF_UNKNOWN;
        super::IFF_BMP;
    }
}
