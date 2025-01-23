use super::super::dl::{l_int32, Box as LeptBox, get_api};
use thiserror::Error;

/// Wrapper around Leptonica's [`Box`](https://tpgit.github.io/Leptonica/struct_box.html) structure
#[derive(Debug, PartialEq)]
pub struct Box(*mut LeptBox);

/// Error returned by Box::create_valid
#[derive(Debug, Error)]
#[error("Box::create_valid returned null")]
pub struct BoxCreateValidError();

impl Drop for Box {
    fn drop(&mut self) {
        unsafe {
            (get_api().boxDestroy)(&mut self.0);
        }
    }
}

impl AsRef<LeptBox> for Box {
    fn as_ref(&self) -> &LeptBox {
        unsafe { &*self.0 }
    }
}

impl Box {
    /// Convinience wrapper for [Self::create_valid]
    pub fn new(
        x: l_int32,
        y: l_int32,
        w: l_int32,
        h: l_int32,
    ) -> Result<Self, BoxCreateValidError> {
        Self::create_valid(x, y, w, h)
    }

    /// Wrapper for [`boxCreateValid`](https://tpgit.github.io/Leptonica/boxbasic_8c.html#a435610d86a8562dc60bfd75fe0a15420)
    ///
    /// Input: x, y, w, h Return: box
    pub fn create_valid(
        x: l_int32,
        y: l_int32,
        w: l_int32,
        h: l_int32,
    ) -> Result<Self, BoxCreateValidError> {
        let ptr = unsafe { (get_api().boxCreateValid)(x, y, w, h) };
        if ptr.is_null() {
            Err(BoxCreateValidError())
        } else {
            Ok(Self(ptr))
        }
    }
}
