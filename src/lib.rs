extern crate link_cplusplus;

/// Called from the build.rs script to always rebuild the crate.
/// 
/// Inline, so the opimizer can later on remove the references to 
/// TESSERACT_LIB and LEPTONICA_LIB from the binary
#[inline(always)]
pub fn print_cargo_link_includes() {

    #[cfg(not(windows))]
    const TESSERACT_LIB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/lib/libtesseract.a"));
    #[cfg(windows)]
    const TESSERACT_LIB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/lib/tesseract55.lib"));

    #[cfg(not(windows))]
    const LEPTONICA_LIB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/lib/libleptonica.a"));
    #[cfg(windows)]
    const LEPTONICA_LIB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/lib/leptonica-1.85.0.lib"));

    let parent_dir = std::env::var("OUT_DIR")
    .expect("no OUT_DIR set, function must be invoked from build script");

    #[cfg(target_os = "macos")]
    {
        let lept_path = std::path::Path::new(&parent_dir).join("libleptonica.a");
        let tess_path = std::path::Path::new(&parent_dir).join("libtesseract.a");
        
        let _ = std::fs::write(&lept_path, LEPTONICA_LIB);
        let _ = std::fs::write(&tess_path, TESSERACT_LIB);

        println!("cargo:rustc-link-search={parent_dir}");
        println!("cargo:rustc-link-lib=static=tesseract");
        println!("cargo:rustc-link-lib=static=leptonica");
    }

    #[cfg(target_os = "linux")]
    {
        let lept_path = std::path::Path::new(&parent_dir).join("libleptonica.a");
        let tess_path = std::path::Path::new(&parent_dir).join("libtesseract.a");

        let _ = std::fs::write(&lept_path, LEPTONICA_LIB);
        let _ = std::fs::write(&tess_path, TESSERACT_LIB);
    
        println!("cargo:rustc-link-arg={}", lept_path.display());
        println!("cargo:rustc-link-arg={}", tess_path.display());
        println!("cargo:rustc-link-search=/usr/lib64");
    }
    
    #[cfg(target_os = "windows")]
    {
        let lept_path = std::path::Path::new(&parent_dir).join("leptonica-1.85.0.lib");
        let tess_path = std::path::Path::new(&parent_dir).join("tesseract55.lib");

        let lept_path = lept_path.display().to_string().replace(r#"\\?\"#, "");
        let tess_path = tess_path.display().to_string().replace(r#"\\?\"#, "");
    
        println!("cargo:rustc-link-arg={lept_path}");
        println!("cargo:rustc-link-arg={tess_path}");
    }

    // This function should re-run on every build
    std::env::set_var("TESSERACT_REBUILD", format!("{:?}", std::time::Instant::now()));
    println!("cargo:rerun-if-env-changed=TESSERACT_REBUILD");
    println!("cargo:rerun-if-changed=NULL"); // https://stackoverflow.com/a/76743504
}

#[path = "leptonica_plumbing/lib.rs"]
pub mod leptonica_plumbing;
#[path = "leptonica_sys/lib.rs"]
pub mod leptonica_sys;
#[path = "libtesseract_rs/mod.rs"]
pub mod tesseract;
#[path = "tesseract_sys/lib.rs"]
pub mod tesseract_sys;
#[cfg(feature = "parse")]
pub mod parse;