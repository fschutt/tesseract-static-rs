extern crate thiserror;

use crate::leptonica_sys::{pixaDestroy, pixaReadMultipageTiff};
use std::ffi::CStr;

/// Wrapper around Leptonica's [`Pixa`](https://tpgit.github.io/Leptonica/struct_pixa.html) structure
#[derive(Debug, PartialEq)]
pub struct Pixa(*mut crate::leptonica_sys::Pixa);

impl Drop for Pixa {
    fn drop(&mut self) {
        unsafe {
            pixaDestroy(&mut self.0);
        }
    }
}

impl AsRef<crate::leptonica_sys::Pixa> for Pixa {
    fn as_ref(&self) -> &crate::leptonica_sys::Pixa {
        unsafe { &*self.0 }
    }
}

impl AsMut<crate::leptonica_sys::Pixa> for Pixa {
    fn as_mut(&mut self) -> &mut crate::leptonica_sys::Pixa {
        unsafe { &mut *self.0 }
    }
}

impl Pixa {
    /// Create a new Pixa from a pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid Pixa struct.
    /// The Pixa struct must not be mutated whilst the wrapper exists.
    pub unsafe fn new_from_pointer(p: *mut crate::leptonica_sys::Pixa) -> Self {
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
    pub fn get_pix(&mut self, i: isize) -> Option<crate::leptonica_plumbing::BorrowedPix> {
        let lpixa: &mut crate::leptonica_sys::Pixa = self.as_mut();
        if unsafe { crate::leptonica_sys::pixaGetCount(lpixa) }
            <= std::convert::TryFrom::try_from(i).ok()?
        {
            None
        } else {
            unsafe {
                Some(crate::leptonica_plumbing::BorrowedPix::new(
                    crate::leptonica_sys::pixaGetPix(
                        lpixa,
                        i as _,
                        crate::leptonica_sys::L_COPY as _,
                    ),
                ))
            }
        }
    }
}
