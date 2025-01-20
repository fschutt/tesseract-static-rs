use crate::leptonica_sys::{boxaCreate, boxaDestroy, l_int32};

/// Wrapper around Leptonica's [`Boxa`](https://tpgit.github.io/Leptonica/struct_boxa.html) structure
#[derive(Debug, PartialEq)]
pub struct Boxa(*mut crate::leptonica_sys::Boxa);

impl Drop for Boxa {
    fn drop(&mut self) {
        unsafe {
            boxaDestroy(&mut self.0);
        }
    }
}

impl AsRef<crate::leptonica_sys::Boxa> for Boxa {
    fn as_ref(&self) -> &crate::leptonica_sys::Boxa {
        unsafe { &*self.0 }
    }
}

impl AsMut<crate::leptonica_sys::Boxa> for Boxa {
    fn as_mut(&mut self) -> &mut crate::leptonica_sys::Boxa {
        unsafe { &mut *self.0 }
    }
}

impl Boxa {
    /// Create a new Boxa from a pointer
    ///
    /// # Safety
    ///
    /// The pointer must be to a valid Boxa struct.
    /// The Boxa struct must not be mutated whilst the wrapper exists.
    pub unsafe fn new_from_pointer(p: *mut crate::leptonica_sys::Boxa) -> Self {
        Self(p)
    }

    /// Wrapper for [`boxaCreate`](https://tpgit.github.io/Leptonica/boxbasic_8c.html#ae59916b7506831be9bf2119dea063253)
    ///
    /// Input: n (initial number of ptrs) Return: boxa, or null on error
    pub fn create(n: l_int32) -> Option<Boxa> {
        let ptr = unsafe { boxaCreate(n) };
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }

    /// Safely borrow the nth item
    pub fn get<'b>(&'b mut self, i: isize) -> Option<crate::leptonica_plumbing::BorrowedBox<'b>> {
        let lboxa: &mut crate::leptonica_sys::Boxa = self.as_mut();
        if unsafe { crate::leptonica_sys::boxaGetCount(lboxa) }
            <= std::convert::TryFrom::try_from(i).ok()?
        {
            None
        } else {
            unsafe {
                Some(crate::leptonica_plumbing::BorrowedBox::new(
                    core::mem::transmute(&crate::leptonica_sys::boxaGetBox(
                        lboxa,
                        i as i32,
                        crate::leptonica_sys::L_COPY as _,
                    )),
                ))
            }
        }
    }
}
