use tesseract_static::tesseract::Tesseract;

mod parse;

unsafe fn run_tesseract_test() -> String {
    let text = Tesseract::new(
        Some(&std::env::temp_dir().display().to_string()),
        Some("eng"),
    )
    .unwrap()
    .set_image_from_mem(include_bytes!("../../testocr.pnm"))
    .unwrap()
    .get_hocr_text(1)
    .unwrap();

    parse::ParsedHocr::new(&text).ok()
    .and_then(|parsed| serde_json::to_string_pretty(&parsed).ok())
    .unwrap_or_default()
}

fn main() {
    use rayon::prelude::*;

    let dir = std::env::temp_dir().join("eng.traineddata");
    std::fs::write(&dir, include_bytes!("../../eng.traineddata")).unwrap();

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
