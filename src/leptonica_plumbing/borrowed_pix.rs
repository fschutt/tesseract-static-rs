use std::marker::PhantomData;
use super::super::dl::Pix;

/// Borrowed wrapper around Leptonica's [`Pix`](https://tpgit.github.io/Leptonica/struct_pix.html) structure
#[derive(Debug, PartialEq)]
pub struct BorrowedPix<'a> {
    raw: *mut Pix,
    phantom: PhantomData<&'a *mut Pix>,
}

impl<'a> AsRef<Pix> for BorrowedPix<'a> {
    fn as_ref(&self) -> &Pix {
        unsafe { &*self.raw }
    }
}

impl<'a> BorrowedPix<'a> {
    /// Create a new BorrowedPix from a pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid Pix struct.
    /// The pix must not be mutated whilst the BorrowedPix exists.
    pub unsafe fn new(p: *mut Pix) -> Self {
        Self {
            raw: p,
            phantom: PhantomData,
        }
    }
}
