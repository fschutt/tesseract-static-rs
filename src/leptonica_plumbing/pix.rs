use super::super::dl::{get_api, Pix as LeptPix};
use std::convert::AsRef;
use std::{ffi::CStr, num::TryFromIntError};
use thiserror::Error;

/// Wrapper around Leptonica's [`Pix`](https://tpgit.github.io/Leptonica/struct_pix.html) structure
#[derive(Debug)]
pub struct Pix(*mut LeptPix);

/// Error returned by Pix::read_mem
#[derive(Debug, Error, PartialEq)]
pub enum PixReadMemError {
    #[error("Pix::read_mem returned null")]
    NullPtr,
    #[error("Failed to convert image size")]
    ImageSizeConversion(#[from] TryFromIntError),
}

/// Error returned by Pix::read
#[derive(Debug, Error)]
#[error("Pix::read returned null")]
pub struct PixReadError();

impl Drop for Pix {
    fn drop(&mut self) {
        unsafe {
            (get_api().pixDestroy)(&mut self.0);
        }
    }
}

impl AsRef<*mut LeptPix> for Pix {
    fn as_ref(&self) -> &*mut LeptPix {
        &self.0
    }
}

impl AsRef<LeptPix> for Pix {
    fn as_ref(&self) -> &LeptPix {
        unsafe { &*self.0 }
    }
}

impl Pix {
    /// Create a new instance from a pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid `Pix` struct.
    ///
    /// The structure must not be mutated or freed outside of the Rust code.
    ///
    /// It must be safe for Rust to free the pointer. If this is not the case consider using [super::BorrowedPix::new].
    pub unsafe fn new_from_pointer(ptr: *mut LeptPix) -> Self {
        Self(ptr)
    }

    /// Wrapper for [`pixRead`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#a84634846cbb5e01df667d6e9241dfc53)
    ///
    /// Read an image from a filename
    pub fn read(filename: &CStr) -> Result<Self, PixReadError> {
        let ptr = unsafe { (get_api().pixRead)(filename.as_ptr()) };
        if ptr.is_null() {
            Err(PixReadError())
        } else {
            Ok(Self(ptr))
        }
    }

    /// Wrapper for [`pixReadMem`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#a027a927dc3438192e3bdae8c219d7f6a)
    ///
    /// Read an image from memory
    pub fn read_mem(img: &[u8]) -> Result<Self, PixReadMemError> {
        let ptr = unsafe { (get_api().pixReadMem)(img.as_ptr(), img.len()) };
        if ptr.is_null() {
            Err(PixReadMemError::NullPtr)
        } else {
            Ok(Self(ptr))
        }
    }
}

#[test]
fn read_error_test() {
    let path = std::ffi::CString::new("fail").unwrap();
    assert!(Pix::read(&path).is_err());
}

#[test]
fn read_mem_error_test() {
    assert_eq!(Pix::read_mem(&[]).err(), Some(PixReadMemError::NullPtr));
}
