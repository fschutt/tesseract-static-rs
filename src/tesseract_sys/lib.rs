#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::leptonica_sys::*;

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/tesseract_sys/capi_bindings.rs"
));
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/tesseract_sys/public_types_bindings.rs"
));
