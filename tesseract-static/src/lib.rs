extern crate link_cplusplus;

#[path = "leptonica_plumbing/lib.rs"]
pub mod leptonica_plumbing;
#[path = "leptonica_sys/lib.rs"]
pub mod leptonica_sys;
#[path = "libtesseract_rs/mod.rs"]
pub mod tesseract;
#[path = "tesseract_sys/lib.rs"]
pub mod tesseract_sys;
