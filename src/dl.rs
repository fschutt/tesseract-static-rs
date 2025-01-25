#![allow(non_camel_case_types, non_snake_case, dead_code, non_upper_case_globals)]

use dlopen::raw::Library;
use std::os::raw::{c_char, c_int, c_uchar};
use lazy_static::lazy_static;
use std::fs;
use std::path::{Path, PathBuf};

const TESSERACT_LIB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/tesseract.dll"));
const LEPTONICA_LIB: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/leptonica.dll"));

lazy_static! {
    /// The one-and-only static instance of `Api`. This is initialized on first access.
    static ref API: Api = {

        #[cfg(target_os = "macos")]
        const LEPTONICA_FILENAME: &str = "libleptonica.dylib";
        #[cfg(target_os = "linux")]
        const LEPTONICA_FILENAME: &str = "libleptonica.so";

        // NOTE: On windows, tesseract.dll links to "leptonica-1.85.0.dll"
        // with THAT EXACT NAME.
        #[cfg(target_os = "windows")]
        const LEPTONICA_FILENAME: &str = "leptonica-1.85.0.dll";

        #[cfg(target_os = "macos")]
        const TESSERACT_FILENAME: &str = "libtesseract.dylib";
        #[cfg(target_os = "linux")]
        const TESSERACT_FILENAME: &str = "libtesseract.so";
        #[cfg(target_os = "windows")]
        const TESSERACT_FILENAME: &str = "tesseract.dll";

        // 1. Choose/write the library files to a temp directory:
        let tempdir = std::env::current_exe().unwrap().parent().unwrap().join("tesseract");
        
        fs::create_dir_all(&tempdir)
            .expect("Failed to create temp directory for Tesseract/Leptonica libraries");

        // Full paths to the libraries we'll write out:
        let leptonica_path: PathBuf = tempdir.join(LEPTONICA_FILENAME);
        let tesseract_path: PathBuf = tempdir.join(TESSERACT_FILENAME);

        // Write the embedded bytes to disk:
        fs::write(&leptonica_path, &LEPTONICA_LIB)
            .expect("Failed to write Leptonica library to disk");
        fs::write(&tesseract_path, &TESSERACT_LIB)
            .expect("Failed to write Tesseract library to disk");

        // 2 & 3. Dynamically load them via `init()`:
        match init(&leptonica_path, &tesseract_path) {
            Ok(api) => api,
            Err(e) => panic!("Failed to init Tesseract/Leptonica API: {}", e),
        }
    };
}

/// Returns a reference to the lazily-initialized [`Api`].
///
/// On first call, this writes the libraries to a temp folder, loads them,
/// and stores the result in a static. Subsequent calls simply return the same `Api`.
pub fn get_api() -> &'static Api {
    &API
}

#[doc = " Object Access"]
pub type _bindgen_ty_37 = ::std::os::raw::c_uint;
#[doc = "< make/use a copy of the object"]
pub const L_COPY: _bindgen_ty_37 = 1;

// Assuming the necessary type definitions for Leptonica and Tesseract structs and enums
// These types should be defined as per the bindings.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TessBaseAPI {
    _unused: [u8; 0],
}

pub const TessOcrEngineMode_OEM_TESSERACT_ONLY: TessOcrEngineMode = 0;
pub const TessOcrEngineMode_OEM_LSTM_ONLY: TessOcrEngineMode = 1;
pub const TessOcrEngineMode_OEM_TESSERACT_LSTM_COMBINED: TessOcrEngineMode = 2;
pub const TessOcrEngineMode_OEM_DEFAULT: TessOcrEngineMode = 3;
pub type TessOcrEngineMode = ::std::os::raw::c_uint;
pub type TessPageIteratorLevel = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ETEXT_DESC {
    _unused: [u8; 0],
}

#[doc = "< clone of source pix"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Pix {
    pub _address: u8,
}

#[doc = "< pixa of bitmaps for 93 characters"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Pixa {
    pub _address: u8,
}

#[doc = " Array of pix"]
pub type PIXA = Pixa;

#[doc = "< bounding region for all images"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Box {
    pub _address: u8,
}

#[doc = " Basic rectangle"]
pub type BOX = Box;

#[doc = " Array of Box"]
pub type BOXA = Boxa;

#[doc = "< tile locations"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Boxa {
    pub _address: u8,
}

pub type l_int32 = c_int;
pub type l_uint8 = c_uchar;

pub struct Api {
    leptonica_handle: Library,
    tesseract_handle: Library,

    // Leptonica functions
    pub boxDestroy: unsafe extern "C" fn(*mut *mut BOX),
    pub boxCreateValid: unsafe extern "C" fn(l_int32, l_int32, l_int32, l_int32) -> *mut BOX,
    pub boxaDestroy: unsafe extern "C" fn(*mut *mut BOXA),
    pub boxaCreate: unsafe extern "C" fn(l_int32) -> *mut BOXA,
    pub boxaGetCount: unsafe extern "C" fn(*const BOXA) -> l_int32,
    pub boxaGetBox: unsafe extern "C" fn(*mut BOXA, l_int32, l_int32) -> *mut BOX,
    pub pixDestroy: unsafe extern "C" fn(*mut *mut Pix),
    pub pixRead: unsafe extern "C" fn(*const c_char) -> *mut Pix,
    pub pixReadMem: unsafe extern "C" fn(*const l_uint8, usize) -> *mut Pix,
    pub pixaDestroy: unsafe extern "C" fn(*mut *mut PIXA),
    pub pixaReadMultipageTiff: unsafe extern "C" fn(*const c_char) -> *mut PIXA,
    pub pixaGetCount: unsafe extern "C" fn(*mut PIXA) -> l_int32,
    pub pixaGetPix: unsafe extern "C" fn(*mut PIXA, l_int32, l_int32) -> *mut Pix,
    pub getLeptonicaVersion: unsafe extern "C" fn() -> *mut c_char,
    pub getImagelibVersions: unsafe extern "C" fn() -> *mut c_char,

    // Tesseract functions
    pub TessBaseAPIDelete: unsafe extern "C" fn(*mut TessBaseAPI),
    pub TessBaseAPICreate: unsafe extern "C" fn() -> *mut TessBaseAPI,
    pub TessDeleteIntArray: unsafe extern "C" fn(*const c_int),
    pub TessBaseAPIInit3: unsafe extern "C" fn(*mut TessBaseAPI, *const c_char, *const c_char) -> c_int,
    pub TessBaseAPIInit2: unsafe extern "C" fn(*mut TessBaseAPI, *const c_char, *const c_char, TessOcrEngineMode) -> c_int,
    pub TessBaseAPISetImage2: unsafe extern "C" fn(*mut TessBaseAPI, *mut Pix),
    pub TessBaseAPISetImage: unsafe extern "C" fn(*mut TessBaseAPI, *const c_uchar, c_int, c_int, c_int, c_int),
    pub TessBaseAPISetSourceResolution: unsafe extern "C" fn(*mut TessBaseAPI, c_int),
    pub TessBaseAPISetVariable: unsafe extern "C" fn(*mut TessBaseAPI, *const c_char, *const c_char) -> c_int,
    pub TessBaseAPIRecognize: unsafe extern "C" fn(*mut TessBaseAPI, *mut ETEXT_DESC) -> c_int,
    pub TessBaseAPIGetUTF8Text: unsafe extern "C" fn(*mut TessBaseAPI) -> *mut c_char,
    pub TessBaseAPIGetHOCRText: unsafe extern "C" fn(*mut TessBaseAPI, c_int) -> *mut c_char,
    pub TessBaseAPIGetInputImage: unsafe extern "C" fn(*mut TessBaseAPI) -> *mut Pix,
    pub TessBaseAPIGetSourceYResolution: unsafe extern "C" fn(*mut TessBaseAPI) -> c_int,
    pub TessBaseAPISetRectangle: unsafe extern "C" fn(*mut TessBaseAPI, c_int, c_int, c_int, c_int),
    pub TessBaseAPIGetAltoText: unsafe extern "C" fn(*mut TessBaseAPI, c_int) -> *mut c_char,
    pub TessBaseAPIGetTsvText: unsafe extern "C" fn(*mut TessBaseAPI, c_int) -> *mut c_char,
    pub TessBaseAPIGetLSTMBoxText: unsafe extern "C" fn(*mut TessBaseAPI, c_int) -> *mut c_char,
    pub TessBaseAPIGetWordStrBoxText: unsafe extern "C" fn(*mut TessBaseAPI, c_int) -> *mut c_char,
    pub TessBaseAPIMeanTextConf: unsafe extern "C" fn(*mut TessBaseAPI) -> c_int,
    pub TessBaseAPIAllWordConfidences: unsafe extern "C" fn(*mut TessBaseAPI) -> *mut c_int,
    pub TessBaseAPIGetComponentImages: unsafe extern "C" fn(
        *mut TessBaseAPI,
        TessPageIteratorLevel,
        c_int,
        *mut *mut PIXA,
        *mut *mut c_int,
    ) -> *mut BOXA,
    pub TessDeleteText: unsafe extern "C" fn(*const c_char),
    pub TessVersion: unsafe extern "C" fn() -> *const c_char,
}

fn init(leptonica_path: &Path, tesseract_path: &Path) -> Result<Api, String> {

    let leptonica_handle = Library::open(leptonica_path)
        .map_err(|e| format!("Failed to open Leptonica library at {}: {}", leptonica_path.display(), e))?;

    let tesseract_handle = Library::open(tesseract_path)
        .map_err(|e| format!("Failed to open Tesseract library at {}: {}", tesseract_path.display(), e))?;

    Ok(Api {
        // Leptonica functions
        boxDestroy: unsafe { leptonica_handle.symbol("boxDestroy") }
            .map_err(|e| format!("Failed to load boxDestroy: {}", e))?,
        boxCreateValid: unsafe { leptonica_handle.symbol("boxCreateValid") }
            .map_err(|e| format!("Failed to load boxCreateValid: {}", e))?,
        boxaDestroy: unsafe { leptonica_handle.symbol("boxaDestroy") }
            .map_err(|e| format!("Failed to load boxaDestroy: {}", e))?,
        boxaCreate: unsafe { leptonica_handle.symbol("boxaCreate") }
            .map_err(|e| format!("Failed to load boxaCreate: {}", e))?,
        boxaGetCount: unsafe { leptonica_handle.symbol("boxaGetCount") }
            .map_err(|e| format!("Failed to load boxaGetCount: {}", e))?,
        boxaGetBox: unsafe { leptonica_handle.symbol("boxaGetBox") }
            .map_err(|e| format!("Failed to load boxaGetBox: {}", e))?,
        pixDestroy: unsafe { leptonica_handle.symbol("pixDestroy") }
            .map_err(|e| format!("Failed to load pixDestroy: {}", e))?,
        pixRead: unsafe { leptonica_handle.symbol("pixRead") }
            .map_err(|e| format!("Failed to load pixRead: {}", e))?,
        pixReadMem: unsafe { leptonica_handle.symbol("pixReadMem") }
            .map_err(|e| format!("Failed to load pixReadMem: {}", e))?,
        pixaDestroy: unsafe { leptonica_handle.symbol("pixaDestroy") }
            .map_err(|e| format!("Failed to load pixaDestroy: {}", e))?,
        pixaReadMultipageTiff: unsafe { leptonica_handle.symbol("pixaReadMultipageTiff") }
            .map_err(|e| format!("Failed to load pixaReadMultipageTiff: {}", e))?,
        pixaGetCount: unsafe { leptonica_handle.symbol("pixaGetCount") }
            .map_err(|e| format!("Failed to load pixaGetCount: {}", e))?,
        pixaGetPix: unsafe { leptonica_handle.symbol("pixaGetPix") }
            .map_err(|e| format!("Failed to load pixaGetPix: {}", e))?,
        getLeptonicaVersion: unsafe { leptonica_handle.symbol("getLeptonicaVersion") }
            .map_err(|e| format!("Failed to load getLeptonicaVersion: {}", e))?,
        getImagelibVersions: unsafe { leptonica_handle.symbol("getImagelibVersions") }
            .map_err(|e| format!("Failed to load getImagelibVersions: {}", e))?,

        // Tesseract functions
        TessBaseAPIDelete: unsafe { tesseract_handle.symbol("TessBaseAPIDelete") }
            .map_err(|e| format!("Failed to load TessBaseAPIDelete: {}", e))?,
        TessBaseAPICreate: unsafe { tesseract_handle.symbol("TessBaseAPICreate") }
            .map_err(|e| format!("Failed to load TessBaseAPICreate: {}", e))?,
        TessDeleteIntArray: unsafe { tesseract_handle.symbol("TessDeleteIntArray") }
            .map_err(|e| format!("Failed to load TessDeleteIntArray: {}", e))?,
        TessBaseAPIInit3: unsafe { tesseract_handle.symbol("TessBaseAPIInit3") }
            .map_err(|e| format!("Failed to load TessBaseAPIInit3: {}", e))?,
        TessBaseAPIInit2: unsafe { tesseract_handle.symbol("TessBaseAPIInit2") }
            .map_err(|e| format!("Failed to load TessBaseAPIInit2: {}", e))?,
        TessBaseAPISetImage2: unsafe { tesseract_handle.symbol("TessBaseAPISetImage2") }
            .map_err(|e| format!("Failed to load TessBaseAPISetImage2: {}", e))?,
        TessBaseAPISetImage: unsafe { tesseract_handle.symbol("TessBaseAPISetImage") }
            .map_err(|e| format!("Failed to load TessBaseAPISetImage: {}", e))?,
        TessBaseAPISetSourceResolution: unsafe { tesseract_handle.symbol("TessBaseAPISetSourceResolution") }
            .map_err(|e| format!("Failed to load TessBaseAPISetSourceResolution: {}", e))?,
        TessBaseAPISetVariable: unsafe { tesseract_handle.symbol("TessBaseAPISetVariable") }
            .map_err(|e| format!("Failed to load TessBaseAPISetVariable: {}", e))?,
        TessBaseAPIRecognize: unsafe { tesseract_handle.symbol("TessBaseAPIRecognize") }
            .map_err(|e| format!("Failed to load TessBaseAPIRecognize: {}", e))?,
        TessBaseAPIGetUTF8Text: unsafe { tesseract_handle.symbol("TessBaseAPIGetUTF8Text") }
            .map_err(|e| format!("Failed to load TessBaseAPIGetUTF8Text: {}", e))?,
        TessBaseAPIGetHOCRText: unsafe { tesseract_handle.symbol("TessBaseAPIGetHOCRText") }
            .map_err(|e| format!("Failed to load TessBaseAPIGetHOCRText: {}", e))?,
        TessBaseAPIGetInputImage: unsafe { tesseract_handle.symbol("TessBaseAPIGetInputImage") }
            .map_err(|e| format!("Failed to load TessBaseAPIGetInputImage: {}", e))?,
        TessBaseAPIGetSourceYResolution: unsafe { tesseract_handle.symbol("TessBaseAPIGetSourceYResolution") }
            .map_err(|e| format!("Failed to load TessBaseAPIGetSourceYResolution: {}", e))?,
        TessBaseAPISetRectangle: unsafe { tesseract_handle.symbol("TessBaseAPISetRectangle") }
            .map_err(|e| format!("Failed to load TessBaseAPISetRectangle: {}", e))?,
        TessBaseAPIGetAltoText: unsafe { tesseract_handle.symbol("TessBaseAPIGetAltoText") }
            .map_err(|e| format!("Failed to load TessBaseAPIGetAltoText: {}", e))?,
        TessBaseAPIGetTsvText: unsafe { tesseract_handle.symbol("TessBaseAPIGetTsvText") }
            .map_err(|e| format!("Failed to load TessBaseAPIGetTsvText: {}", e))?,
        TessBaseAPIGetLSTMBoxText: unsafe { tesseract_handle.symbol("TessBaseAPIGetLSTMBoxText") }
            .map_err(|e| format!("Failed to load TessBaseAPIGetLSTMBoxText: {}", e))?,
        TessBaseAPIGetWordStrBoxText: unsafe { tesseract_handle.symbol("TessBaseAPIGetWordStrBoxText") }
            .map_err(|e| format!("Failed to load TessBaseAPIGetWordStrBoxText: {}", e))?,
        TessBaseAPIMeanTextConf: unsafe { tesseract_handle.symbol("TessBaseAPIMeanTextConf") }
            .map_err(|e| format!("Failed to load TessBaseAPIMeanTextConf: {}", e))?,
        TessBaseAPIAllWordConfidences: unsafe { tesseract_handle.symbol("TessBaseAPIAllWordConfidences") }
            .map_err(|e| format!("Failed to load TessBaseAPIAllWordConfidences: {}", e))?,
        TessBaseAPIGetComponentImages: unsafe { tesseract_handle.symbol("TessBaseAPIGetComponentImages") }
            .map_err(|e| format!("Failed to load TessBaseAPIGetComponentImages: {}", e))?,
        TessDeleteText: unsafe { tesseract_handle.symbol("TessDeleteText") }
            .map_err(|e| format!("Failed to load TessDeleteText: {}", e))?,
        TessVersion: unsafe { tesseract_handle.symbol("TessVersion") }
            .map_err(|e| format!("Failed to load TessVersion: {}", e))?,

        leptonica_handle,
        tesseract_handle,
    })
}
