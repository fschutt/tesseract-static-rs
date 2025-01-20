use tesseract_static::tesseract::Tesseract;

const TRAINING_DATA: &[u8] = include_bytes!("../eng.traineddata");

fn main() {

    // setup training data
    let parent = std::env::temp_dir();
    std::fs::write(&parent.join("eng.traineddata"), &TRAINING_DATA[..]).unwrap();

    let hocr_xml = Tesseract::new(Some(&parent.display().to_string()), Some("eng"))
    .unwrap()
    .set_image_from_mem(include_bytes!("../testocr.pnm"))
    .unwrap()
    .get_hocr_text(1)
    .unwrap();

    let hocr = tesseract_static::parse::ParsedHocr::new(&hocr_xml).unwrap();

    println!("{hocr:#?}"); // parsed hOCR text from tesseract, includes rect bounds
}