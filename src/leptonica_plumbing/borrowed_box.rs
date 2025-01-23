use super::super::dl::Box;

/// Borrowed wrapper around Leptonica's [`Box`](https://tpgit.github.io/Leptonica/struct_box.html) structure
#[derive(Debug, PartialEq)]
pub struct BorrowedBox<'a>(&'a *mut Box);

impl<'a> AsRef<Box> for BorrowedBox<'a> {
    fn as_ref(&self) -> &Box {
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
    pub unsafe fn new(b: &'a *mut Box) -> Self {
        Self(b)
    }
}
