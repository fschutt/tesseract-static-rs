#[path = "leptonica_plumbing/lib.rs"]
pub mod leptonica_plumbing;
#[path = "leptonica_sys/lib.rs"]
pub mod leptonica_sys;
#[path = "libtesseract_rs/mod.rs"]
pub mod tesseract;
#[path = "tesseract_sys/lib.rs"]
pub mod tesseract_sys;

/// eng.traineddata bytes
pub const TRAINEDDATA_ENG: &[u8] = include_bytes!("../eng.traineddata");
/// deu.traineddata bytes
pub const TRAINEDDATA_DEU: &[u8] = include_bytes!("../deu.traineddata");
