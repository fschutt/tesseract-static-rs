use crate::dl::{TessOcrEngineMode,
    TessOcrEngineMode_OEM_DEFAULT, 
    TessOcrEngineMode_OEM_LSTM_ONLY,
    TessOcrEngineMode_OEM_TESSERACT_LSTM_COMBINED, 
    TessOcrEngineMode_OEM_TESSERACT_ONLY
};
use crate::tesseract_plumbing::{
    TessBaseApiInitError, 
    TessBaseApiSetVariableError,
    TessBaseApiRecogniseError,
    TessBaseApiGetUtf8TextError,
    TessBaseApiGetHocrTextError,
    TessBaseApiGetTsvTextError,
    TessBaseApiSetImageSafetyError,
    TessBaseApi,
};
use std::ffi::CString;
use std::ffi::NulError;
use std::os::raw::c_int;
use std::str;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InitializeError {
    #[error("Conversion to CString failed")]
    CStringError(#[from] NulError),
    #[error("TessBaseApi failed to initialize")]
    TessBaseAPIInitError(#[from] TessBaseApiInitError),
}

#[derive(Debug, Error)]
pub enum SetImageError {
    #[error("Conversion to CString failed")]
    CStringError(#[from] NulError),
    #[error("Failed to read image")]
    PixReadError(#[from] crate::leptonica_plumbing::PixReadError),
}

#[derive(Debug, Error)]
pub enum SetVariableError {
    #[error("Conversion to CString failed")]
    CStringError(#[from] NulError),
    #[error("TessBaseApi failed to set variable")]
    TessBaseAPISetVariableError(#[from] TessBaseApiSetVariableError),
}

#[derive(Debug, Error)]
pub enum TesseractError {
    #[error("Failed to set language")]
    InitializeError(#[from] InitializeError),
    #[error("Failed to set image")]
    SetImageError(#[from] SetImageError),
    #[error("Errored whilst recognizing")]
    RecognizeError(#[from] TessBaseApiRecogniseError),
    #[error("Errored whilst getting text")]
    GetTextError(#[from] TessBaseApiGetUtf8TextError),
    #[error("Errored whilst getting HOCR text")]
    GetHOCRTextError(#[from] TessBaseApiGetHocrTextError),
    #[error("Errored whilst getting TSV text")]
    GetTsvTextError(#[from] TessBaseApiGetTsvTextError),
    #[error("Errored whilst setting frame")]
    SetFrameError(#[from] TessBaseApiSetImageSafetyError),
    #[error("Errored whilst setting image from mem")]
    SetImgFromMemError(#[from] crate::leptonica_plumbing::PixReadMemError),
    #[error("Errored whilst setting variable")]
    SetVariableError(#[from] SetVariableError),
}

/// https://tesseract-ocr.github.io/tessapi/5.x/a01818.html#a04550a0ed1279562027bf2fc92c421aead84e1ef94e50df1622b4fcd189c6c00b
pub enum OcrEngineMode {
    /// Run Tesseract only - fastest; deprecated
    Default,
    /// Run just the LSTM line recognizer.
    LstmOnly,
    /// Run the LSTM recognizer, but allow fallback
    /// to Tesseract when things get difficult.
    /// deprecated
    TesseractLstmCombined,
    /// Specify this mode,
    /// to indicate that any of the above modes
    /// should be automatically inferred from the
    /// variables in the language-specific config,
    /// command-line configs, or if not specified
    /// in any of the above should be set to the
    /// default OEM_TESSERACT_ONLY.
    TesseractOnly,
}

impl OcrEngineMode {
    fn to_value(&self) -> TessOcrEngineMode {
        match *self {
            OcrEngineMode::Default => TessOcrEngineMode_OEM_DEFAULT,
            OcrEngineMode::LstmOnly => TessOcrEngineMode_OEM_LSTM_ONLY,
            OcrEngineMode::TesseractLstmCombined => TessOcrEngineMode_OEM_TESSERACT_LSTM_COMBINED,
            OcrEngineMode::TesseractOnly => TessOcrEngineMode_OEM_TESSERACT_ONLY,
        }
    }
}

pub struct Tesseract(TessBaseApi);

impl Tesseract {
    pub fn new(datapath: Option<&str>, language: Option<&str>) -> Result<Self, InitializeError> {
        let mut tess = Tesseract(TessBaseApi::create());
        let datapath = match datapath {
            Some(i) => Some(CString::new(i)?),
            None => None,
        };
        let language = match language {
            Some(i) => Some(CString::new(i)?),
            None => None,
        };

        tess.0.init_2(datapath.as_deref(), language.as_deref())?;
        Ok(tess)
    }

    pub fn new_with_oem(
        datapath: Option<&str>,
        language: Option<&str>,
        oem: OcrEngineMode,
    ) -> Result<Self, InitializeError> {
        let mut tess = Tesseract(TessBaseApi::create());
        let datapath = match datapath {
            Some(i) => Some(CString::new(i)?),
            None => None,
        };
        let language = match language {
            Some(i) => Some(CString::new(i)?),
            None => None,
        };

        tess.0
            .init_4(datapath.as_deref(), language.as_deref(), oem.to_value())?;
        Ok(tess)
    }

    pub fn set_image(mut self, filename: &str) -> Result<Self, SetImageError> {
        let pix = crate::leptonica_plumbing::Pix::read(&CString::new(filename)?)?;
        self.0.set_image_2(&pix);
        Ok(self)
    }
    pub fn set_frame(
        mut self,
        frame_data: &[u8],
        width: i32,
        height: i32,
        bytes_per_pixel: i32,
        bytes_per_line: i32,
    ) -> Result<Self, TessBaseApiSetImageSafetyError> {
        self.0
            .set_image(frame_data, width, height, bytes_per_pixel, bytes_per_line)?;
        Ok(self)
    }
    pub fn set_image_from_mem(
        mut self,
        img: &[u8],
    ) -> Result<Self, crate::leptonica_plumbing::PixReadMemError> {
        let pix = crate::leptonica_plumbing::Pix::read_mem(img)?;
        self.0.set_image_2(&pix);
        Ok(self)
    }

    pub fn set_rectangle(mut self, left: i32, top: i32, width: i32, height: i32) -> Self {
        self.0.set_rectangle(left, top, width, height);
        self
    }

    pub fn set_source_resolution(mut self, ppi: i32) -> Self {
        self.0.set_source_resolution(ppi);
        self
    }

    pub fn set_variable(mut self, name: &str, value: &str) -> Result<Self, SetVariableError> {
        self.0
            .set_variable(&CString::new(name)?, &CString::new(value)?)?;
        Ok(self)
    }
    pub fn recognize(
        mut self,
    ) -> Result<Self, TessBaseApiRecogniseError> {
        self.0.recognize()?;
        Ok(self)
    }
    pub fn get_text(
        &mut self,
    ) -> Result<String, TessBaseApiGetUtf8TextError> {
        Ok(self
            .0
            .get_utf8_text()?
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }
    pub fn mean_text_conf(&mut self) -> i32 {
        self.0.mean_text_conf()
    }

    /// Get the text encoded as HTML with bounding box tags
    ///
    /// See [img.html](../img.html) for an example.
    pub fn get_hocr_text(
        &mut self,
        page: c_int,
    ) -> Result<String, TessBaseApiGetHocrTextError> {
        Ok(self
            .0
            .get_hocr_text(page)?
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }

    /// Get the text encoded as TSV, including bounding boxes, confidence
    ///
    /// See [char* TessBaseAPI::GetTSVText](https://github.com/tesseract-ocr/tesseract/blob/cdebe13d81e2ad2a83be533886750f5491b25262/src/api/baseapi.cpp#L1398)
    pub fn get_tsv_text(
        &mut self,
        page: c_int,
    ) -> Result<String, TessBaseApiGetTsvTextError> {
        Ok(self
            .0
            .get_tsv_text(page)?
            .as_ref()
            .to_string_lossy()
            .into_owned())
    }
}

pub fn ocr(filename: &str, language: &str) -> Result<String, TesseractError> {
    Ok(Tesseract::new(None, Some(language))?
        .set_image(filename)?
        .recognize()?
        .get_text()?)
}

pub fn ocr_from_frame(
    frame_data: &[u8],
    width: i32,
    height: i32,
    bytes_per_pixel: i32,
    bytes_per_line: i32,
    language: &str,
) -> Result<String, TesseractError> {
    Ok(Tesseract::new(None, Some(language))?
        .set_frame(frame_data, width, height, bytes_per_pixel, bytes_per_line)?
        .recognize()?
        .get_text()?)
}

#[test]
fn hocr_test() -> Result<(), TesseractError> {
    let mut cube = Tesseract::new(None, Some("eng"))?.set_image("img.png")?;
    assert!(&cube.get_hocr_text(0)?.contains("<div class='ocr_page'"));
    Ok(())
}

#[test]
#[ignore] // Many systems do not have legacy Tesseract data available
fn oem_test() -> Result<(), TesseractError> {
    let only_tesseract_str =
        Tesseract::new_with_oem(None, Some("eng"), OcrEngineMode::TesseractOnly)?
            .set_image("img.png")?
            .recognize()?
            .get_text()?;

    let only_lstm_str = Tesseract::new_with_oem(None, Some("eng"), OcrEngineMode::LstmOnly)?
        .set_image("img.png")?
        .recognize()?
        .get_text()?;

    assert_ne!(only_tesseract_str, only_lstm_str);
    Ok(())
}
