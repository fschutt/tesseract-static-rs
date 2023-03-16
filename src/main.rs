#[path = "leptonica_plumbing/lib.rs"]
pub mod leptonica_plumbing;
#[path = "leptonica_sys/lib.rs"]
pub mod leptonica_sys;
#[path = "libtesseract_rs/mod.rs"]
pub mod tesseract;
#[path = "tesseract_sys/lib.rs"]
pub mod tesseract_sys;

unsafe fn run_tesseract_test() -> String {
    crate::tesseract::Tesseract::new(Some("."), Some("eng"))
        .unwrap()
        .set_image_from_mem(include_bytes!("./testocr.pnm"))
        .unwrap()
        .get_hocr_text(1)
        .unwrap()
}

fn main() {
    use rayon::prelude::*;

    println!("{}", unsafe { run_tesseract_test() });

    let now = std::time::Instant::now();
    for i in 0..10 {
        println!("{i} {}", unsafe { run_tesseract_test() }.len());
    }
    println!("single thread: {:?}", std::time::Instant::now() - now);
    let now = std::time::Instant::now();

    let i: usize = [0; 10]
        .par_iter()
        .map(|_| unsafe { run_tesseract_test() }.len())
        .sum();

    println!("{i}");
    println!("multi thread: {:?}", std::time::Instant::now() - now);
    println!("Hello, world!");
}
