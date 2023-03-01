extern crate leptonica_sys;
extern crate thiserror;

use leptonica_sys::{pixaDestroy, pixaReadMultipageTiff};
use std::ffi::CStr;

/// Wrapper around Leptonica's [`Pixa`](https://tpgit.github.io/Leptonica/struct_pixa.html) structure
#[derive(Debug, PartialEq)]
pub struct Pixa(*mut leptonica_sys::Pixa);

impl Drop for Pixa {
    fn drop(&mut self) {
        unsafe {
            pixaDestroy(&mut self.0);
        }
    }
}

impl AsRef<leptonica_sys::Pixa> for Pixa {
    fn as_ref(&self) -> &leptonica_sys::Pixa {
        unsafe { &*self.0 }
    }
}

impl Pixa {
    /// Create a new Pixa from a pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid Pixa struct.
    /// The Pixa struct must not be mutated whilst the wrapper exists.
    pub unsafe fn new_from_pointer(p: *mut leptonica_sys::Pixa) -> Self {
        Self(p)
    }

    /// Wrapper for [`pixaReadMultipageTiff`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#a4a52e686cf67f0e5bfda661fc3a3fb7b)
    pub fn read_multipage_tiff(filename: &CStr) -> Option<Self> {
        let ptr = unsafe { pixaReadMultipageTiff(filename.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }

    /// Safely borrow the nth item
    pub fn get_pix(&self, i: isize) -> Option<crate::BorrowedPix> {
        let lpixa: &leptonica_sys::Pixa = self.as_ref();
        if lpixa.n <= std::convert::TryFrom::try_from(i).ok()? {
            None
        } else {
            unsafe { Some(crate::BorrowedPix::new(*lpixa.pix.offset(i))) }
        }
    }
}

#[test]
fn read_multipage_tiff_test() {
    let pixa =
        Pixa::read_multipage_tiff(CStr::from_bytes_with_nul(b"multipage.tiff\0").unwrap()).unwrap();
    assert_eq!(pixa.as_ref().n, 2);
    assert_eq!(pixa.get_pix(0).unwrap().as_ref().w, 165);
    assert_eq!(pixa.get_pix(0).unwrap().as_ref().h, 67);
    assert_eq!(pixa.get_pix(1).unwrap().as_ref().w, 165);
    assert_eq!(pixa.get_pix(1).unwrap().as_ref().h, 67);
    assert_eq!(pixa.get_pix(2), None);
}
