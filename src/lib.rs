
pub mod dl;
pub mod leptonica_plumbing;
pub mod tesseract_plumbing;
#[path = "libtesseract_rs/mod.rs"]
pub mod tesseract;

/*
    #[path = "leptonica_sys/lib.rs"]
    pub mod leptonica_sys;
    #[path = "libtesseract_rs/mod.rs"]
    pub mod tesseract;
    #[path = "tesseract_sys/lib.rs"]
    pub mod tesseract_sys;
*/

#[cfg(feature = "parse")]
pub mod parse;