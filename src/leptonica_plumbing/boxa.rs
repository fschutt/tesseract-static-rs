use super::super::dl::{l_int32, get_api, Boxa as LeptBoxa};

/// Wrapper around Leptonica's [`Boxa`](https://tpgit.github.io/Leptonica/struct_boxa.html) structure
#[derive(Debug, PartialEq)]
pub struct Boxa(*mut LeptBoxa);

impl Drop for Boxa {
    fn drop(&mut self) {
        unsafe {
            (get_api().boxaDestroy)(&mut self.0);
        }
    }
}

impl AsRef<LeptBoxa> for Boxa {
    fn as_ref(&self) -> &LeptBoxa {
        unsafe { &*self.0 }
    }
}

impl AsMut<LeptBoxa> for Boxa {
    fn as_mut(&mut self) -> &mut LeptBoxa {
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
    pub unsafe fn new_from_pointer(p: *mut LeptBoxa) -> Self {
        Self(p)
    }

    /// Wrapper for [`boxaCreate`](https://tpgit.github.io/Leptonica/boxbasic_8c.html#ae59916b7506831be9bf2119dea063253)
    ///
    /// Input: n (initial number of ptrs) Return: boxa, or null on error
    pub fn create(n: l_int32) -> Option<Boxa> {
        let ptr = unsafe { (get_api().boxaCreate)(n) };
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }
}
