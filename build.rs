use std::env;
use std::path::{Path, PathBuf};

/*

fn generate_tesseract_bindings() {
    let path1 = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/third_party/tesseract/include/"
    ));
    let path1 = path1.canonicalize().unwrap();
    let clang_extra_include = vec![
        path1.display().to_string(),
        "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/include/c++/v1/".to_string(),
    ];

    let mut capi_bindings = bindgen::Builder::default()
        .header(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/tesseract_sys/wrapper_capi.h"
        ))
        .detect_include_paths(true)
        .allowlist_function("^Tess.*")
        .blocklist_type("Boxa")
        .blocklist_type("Pix")
        .blocklist_type("Pixa")
        .blocklist_type("_IO_FILE")
        .blocklist_type("_IO_codecvt")
        .blocklist_type("_IO_marker")
        .blocklist_type("_IO_wide_data");

    for inc in clang_extra_include.iter() {
        capi_bindings = capi_bindings.clang_arg(format!("-I{}", inc));
    }

    let bindings = capi_bindings // if this fails on Mac, run "xcode-select --install" to install standard C headers
        .generate()
        .expect("Unable to generate capi bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/tesseract_sys/capi_bindings.rs"
        ))
        .expect("Couldn't write capi bindings!");

    fs::write(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/tesseract_sys/public_types_bindings.rs"
        ),
        public_types_bindings(&clang_extra_include),
    )
    .expect("Couldn't write public types bindings!");
}

fn generate_leptonica_bindings() {
    let include_path = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/third_party/leptonica/src/"
    ))
    .canonicalize()
    .unwrap();
    let bindings = bindgen::Builder::default()
        .header(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/third_party/leptonica/src/allheaders.h"
        ))
        .detect_include_paths(true)
        .clang_arg(format!("-I{}", include_path.display().to_string()));

    let bindings = bindings
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/leptonica_sys/bindings.rs"
    ));
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}

#[cfg(not(target_os = "macos"))]
fn public_types_bindings(clang_extra_include: &[String]) -> String {
    let mut public_types_bindings = bindgen::Builder::default()
        .header(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/tesseract_sys/wrapper_public_types.hpp"
        ))
        .detect_include_paths(true)
        .allowlist_var("^k.*")
        .allowlist_var("^tesseract::k.*")
        .blocklist_item("^kPolyBlockNames")
        .blocklist_item("^tesseract::kPolyBlockNames");

    for inc in clang_extra_include {
        public_types_bindings = public_types_bindings.clang_arg(format!("-I{}", *inc));
    }

    public_types_bindings
        .generate()
        .expect("Unable to generate public types bindings")
        .to_string()
        .replace("tesseract_k", "k")
}

// MacOS clang is incompatible with Bindgen and constexpr
// https://github.com/rust-lang/rust-bindgen/issues/1948
// Hardcode the constants rather than reading them dynamically
#[cfg(target_os = "macos")]
fn public_types_bindings(_clang_extra_include: &[String]) -> &'static str {
    include_str!("src/tesseract_sys/public_types_bindings_mac.rs")
}

*/

// let (leptonica_lib, _leptonica_includes) = compile_leptonica(&download_leptonica());
// let (tesseract_lib, _tesseract_includes) = compile_tesseract(&download_tesseract());
// generate_leptonica_bindings(&leptonica_includes);
// generate_tesseract_bindings(&tesseract_includes);

fn main() {}
