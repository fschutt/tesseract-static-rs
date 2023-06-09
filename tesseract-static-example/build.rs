fn main() {
    let (leptonica_lib, _leptonica_includes) =
        tesseract_static_build::compile_leptonica(&tesseract_static_build::download_leptonica());
    let (tesseract_lib, _tesseract_includes) =
        tesseract_static_build::compile_tesseract(&tesseract_static_build::download_tesseract());
    tesseract_static_build::print_cargo_link_includes(&leptonica_lib, &tesseract_lib);
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-search=/usr/lib64");
}