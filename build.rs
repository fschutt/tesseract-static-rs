use std::env;
use std::path::{Path, PathBuf};

fn main() {

    #[cfg(target_os = "macos")]
    let lept = "https://github.com/fschutt/tesseract-static-rs/releases/download/0.2.0/libleptonica.dylib";
    #[cfg(target_os = "macos")]
    let tess = "https://github.com/fschutt/tesseract-static-rs/releases/download/0.2.0/libtesseract.dylib";

    #[cfg(target_os = "linux")]
    let lept = "https://github.com/fschutt/tesseract-static-rs/releases/download/0.2.0/libleptonica.so";
    #[cfg(target_os = "linux")]
    let tess = "https://github.com/fschutt/tesseract-static-rs/releases/download/0.2.0/libtesseract.so";

    #[cfg(target_os = "windows")]
    let lept = "https://github.com/fschutt/tesseract-static-rs/releases/download/0.2.0/leptonica.dll";
    #[cfg(target_os = "windows")]
    let tess = "https://github.com/fschutt/tesseract-static-rs/releases/download/0.2.0/tesseract.dll";

    let lept_target = Path::new(&env::var("OUT_DIR").unwrap()).join("leptonica.dll");
    let tess_target = Path::new(&env::var("OUT_DIR").unwrap()).join("tesseract.dll");

    download(lept, &lept_target);
    download(tess, &tess_target);
}

fn download(url: &str, target: &PathBuf) {

    if target.exists() {
        return;
    }
    
    let client = reqwest::blocking::Client::builder()
    .timeout(std::time::Duration::from_secs(60))
    .build().unwrap();

    let body = client.get(url).send().unwrap().bytes().unwrap();
    std::fs::write(target, body.as_ref()).unwrap();
}
