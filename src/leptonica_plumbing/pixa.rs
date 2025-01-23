extern crate thiserror;

use super::super::dl::{get_api, Pixa as LeptPixa};
use std::ffi::CStr;

/// Wrapper around Leptonica's [`Pixa`](https://tpgit.github.io/Leptonica/struct_pixa.html) structure
#[derive(Debug, PartialEq)]
pub struct Pixa(*mut LeptPixa);

impl Drop for Pixa {
    fn drop(&mut self) {
        unsafe {
            (get_api().pixaDestroy)(&mut self.0);
        }
    }
}

impl AsRef<LeptPixa> for Pixa {
    fn as_ref(&self) -> &LeptPixa {
        unsafe { &*self.0 }
    }
}

impl AsMut<LeptPixa> for Pixa {
    fn as_mut(&mut self) -> &mut LeptPixa {
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
    pub unsafe fn new_from_pointer(p: *mut LeptPixa) -> Self {
        Self(p)
    }

    /// Wrapper for [`pixaReadMultipageTiff`](https://tpgit.github.io/Leptonica/leptprotos_8h.html#a4a52e686cf67f0e5bfda661fc3a3fb7b)
    pub fn read_multipage_tiff(filename: &CStr) -> Option<Self> {
        let ptr = unsafe { (get_api().pixaReadMultipageTiff)(filename.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }
}
