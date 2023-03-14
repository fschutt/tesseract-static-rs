use crate::leptonica_sys;

/// Borrowed wrapper around Leptonica's [`Box`](https://tpgit.github.io/Leptonica/struct_box.html) structure
#[derive(Debug, PartialEq)]
pub struct BorrowedBox<'a>(&'a *mut leptonica_sys::Box);

impl<'a> AsRef<leptonica_sys::Box> for BorrowedBox<'a> {
    fn as_ref(&self) -> &leptonica_sys::Box {
        unsafe { &**self.0 }
    }
}

impl<'a> BorrowedBox<'a> {
    /// Create a new BorrowedBox from a pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid Box struct.
    /// The box must not be mutated whilst the BorrowedBox exists.
    pub unsafe fn new(b: &'a *mut leptonica_sys::Box) -> Self {
        Self(b)
    }
}
