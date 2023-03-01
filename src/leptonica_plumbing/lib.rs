mod borrowed_box;
mod borrowed_pix;
mod r#box;
mod boxa;
mod pix;
mod pixa;

use self::leptonica_sys::{free, getImagelibVersions, getLeptonicaVersion};
pub use leptonica_sys;
use libc::c_char;
use std::ffi::{c_void, CStr};

pub use borrowed_box::BorrowedBox;
pub use borrowed_pix::BorrowedPix;
pub use boxa::Boxa;
pub use pix::{Pix, PixReadError, PixReadMemError};
pub use pixa::Pixa;
pub use r#box::{Box, BoxCreateValidError};

/// Wrapper for leptonica strings
#[derive(Debug)]
pub struct LeptonicaString {
    value: *mut c_char,
}

impl AsRef<str> for LeptonicaString {
    fn as_ref(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.value)
                .to_str()
                .expect("failed to get leptonica string")
        }
    }
}

impl AsRef<CStr> for LeptonicaString {
    fn as_ref(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.value) }
    }
}

impl Drop for LeptonicaString {
    fn drop(&mut self) {
        unsafe { free(self.value as *mut c_void) }
    }
}

/// Wrapper for [`getLeptonicaVersion`](https://github.com/DanBloomberg/leptonica/blob/1.82.0/src/utils1.c#L970-L982)
///
/// Returns the version identifier as a LeptonicaString.
pub fn get_version() -> LeptonicaString {
    LeptonicaString {
        value: unsafe { getLeptonicaVersion() },
    }
}

/// Wrapper for [`getImagelibVersions`](https://github.com/DanBloomberg/leptonica/blob/1.82.0/src/libversions.c#L82-L102)
///
/// Returns the image lib version identifiers as a LeptonicaString.
pub fn get_imagelib_versions() -> LeptonicaString {
    LeptonicaString {
        value: unsafe { getImagelibVersions() },
    }
}
