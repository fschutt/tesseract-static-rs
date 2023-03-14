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
        .set_image_from_mem(include_bytes!("./testocr.png"))
        .unwrap()
        .get_hocr_text(1)
        .unwrap()
}

fn main() {
    use rayon::prelude::*;
    let now = std::time::Instant::now();
    for i in 0..10 {
        println!("{i} {}", unsafe { run_tesseract_test() }.len());
    }
    println!("single thread: {:?}", std::time::Instant::now() - now);
    let now = std::time::Instant::now();

    let i: usize = [0; 10]
        .par_iter()
        .map(|i| unsafe { run_tesseract_test() }.len())
        .sum();

    println!("{i}");
    println!("multi thread: {:?}", std::time::Instant::now() - now);

    /*
        #include <tesseract/baseapi.h>
    #include <leptonica/allheaders.h>

    int main()
    {
        char *outText;

        tesseract::TessBaseAPI *api = new tesseract::TessBaseAPI();
    // Initialize tesseract-ocr with English, without specifying tessdata path
        if (api->Init(NULL, "eng")) {
            fprintf(stderr, "Could not initialize tesseract.\n");
            exit(1);
        }

    // Open input image with leptonica library
        Pix *image = pixRead("/tesseract/test/testing/trainingital.tif");
        api->SetImage(image);
        api->SetVariable("lstm_choice_mode", "2");
    // Get HOCR result
        outText = api->GetHOCRText(0);
        printf("HOCR alternative symbol choices  per character :\n%s", outText);

    // Destroy used object and release memory
        api->End();
        delete [] outText;
        pixDestroy(&image);

        return 0;
    }

        */
    println!("Hello, world!");
}
