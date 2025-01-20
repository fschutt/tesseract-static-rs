mod tess_base_api;
mod text;

pub use crate::leptonica_plumbing;
pub use crate::leptonica_sys;
pub use crate::tesseract_sys;
use crate::tesseract_sys::TessVersion;
use std::ffi::CStr;
pub use tess_base_api::{
    TessBaseApi, TessBaseApiGetAltoTextError, TessBaseApiGetHocrTextError,
    TessBaseApiGetLstmBoxTextError, TessBaseApiGetTsvTextError, TessBaseApiGetUtf8TextError,
    TessBaseApiGetWordStrBoxTextError, TessBaseApiInitError, TessBaseApiRecogniseError,
    TessBaseApiSetImageSafetyError, TessBaseApiSetVariableError,
};
pub use text::Text;

/// Wrapper for [`Version`](https://tesseract-ocr.github.io/tessapi/5.x/a02438.html#a3785779c909fcdd77e24b340f5913e4b)
///
/// Returns the version identifier as a static string.
pub fn version() -> &'static CStr {
    unsafe { CStr::from_ptr(TessVersion()) }
}
