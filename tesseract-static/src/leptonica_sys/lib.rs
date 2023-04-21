#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/leptonica_sys/bindings.rs"
));

#[cfg(test)]
mod tests {
    use super::{pixFreeData, pixRead};
    use std::ffi::CStr;

    #[test]
    fn image_size() {
        unsafe {
            let image =
                pixRead(CStr::from_bytes_with_nul_unchecked(b"../test image.png\0").as_ptr());
            assert_eq!((*image).w, 1000);
            assert_eq!((*image).h, 500);
            pixFreeData(image);
        }
    }

    #[test]
    #[allow(path_statements)]
    fn defined_constants() {
        super::IFF_UNKNOWN;
        super::IFF_BMP;
    }
}
