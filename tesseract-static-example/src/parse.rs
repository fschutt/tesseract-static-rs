use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ParsedHocr {
    pub bounds: Rect,
    pub careas: Vec<HocrArea>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HocrArea {
    pub bounds: Rect,
    pub paragraphs: Vec<HocrParagraph>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HocrParagraph {
    pub bounds: Rect,
    pub lines: Vec<HocrLine>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HocrLine {
    pub bounds: Rect,
    pub words: Vec<HocrWord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HocrWord {
    pub bounds: Rect,
    pub confidence: f32,
    pub text: String,
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Rect {
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
}

impl Rect {
    pub fn zero() -> Self {
        Self::default()
    }

    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        x <= self.max_x && x >= self.min_x && y <= self.max_y && y >= self.min_y
    }

    pub fn overlaps(&self, other: &Rect) -> bool {
        if self.max_x < other.min_x || self.min_x > other.max_x {
            return false;
        }
        if self.max_y < other.min_y || self.min_y > other.max_y {
            return false;
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HocrParseError(String, String);

impl ParsedHocr {
    pub fn new(hocr_tesseract: &str) -> Result<Self, HocrParseError> {
        use kuchiki::traits::TendrilSink;

        let document = kuchiki::parse_html().one(hocr_tesseract);

        let page_node = document
            .select(".ocr_page")
            .map_err(|_| {
                HocrParseError(hocr_tesseract.to_string(), "no .ocr_page found".to_string())
            })?
            .next()
            .ok_or_else(|| {
                HocrParseError(hocr_tesseract.to_string(), "no .ocr_page found".to_string())
            })?;

        let infos = page_node
            .as_node()
            .0
            .as_element()
            .ok_or_else(|| {
                HocrParseError(
                    hocr_tesseract.to_string(),
                    ".ocr_page is not of type ElementNode".to_string(),
                )
            })?
            .attributes
            .borrow()
            .get("title")
            .ok_or_else(|| {
                HocrParseError(hocr_tesseract.to_string(), ".ocr_page has no <title>".to_string())
            })?
            .to_string();

        let page_bounds = get_bbox(&infos).ok_or_else(|| {
            HocrParseError(
                hocr_tesseract.to_string(),
                ".ocr_page has invalid <bounds>".to_string(),
            )
        })?;

        let careas =
            page_node
                .as_node()
                .select(".ocr_carea")
                .and_then(|carea_node| {
                    carea_node
                        .map(|carea_node| {
                            let infos = carea_node
                                .as_node()
                                .0
                                .as_element()
                                .ok_or_else(|| ())?
                                .attributes
                                .borrow()
                                .get("title")
                                .ok_or_else(|| ())?
                                .to_string();

                            let carea_bounds = get_bbox(&infos).ok_or_else(|| ())?;

                            let paragraphs =
                                page_node
                                    .as_node()
                                    .select(".ocr_par")
                                    .and_then(|ocr_par_node| {
                                        ocr_par_node
                                            .map(|ocr_par_node| {
                                                let infos = ocr_par_node
                                                    .as_node()
                                                    .0
                                                    .as_element()
                                                    .ok_or_else(|| ())?
                                                    .attributes
                                                    .borrow()
                                                    .get("title")
                                                    .ok_or_else(|| ())?
                                                    .to_string();

                                                let paragraph_bounds =
                                                    get_bbox(&infos).ok_or_else(|| ())?;

                                                let lines =
                                                    ocr_par_node
                                                        .as_node()
                                                        .select(".ocr_line")
                                                        .and_then(|ocr_line_node| {
                                                            ocr_line_node
                            .map(|ocr_line_node| {
                                let infos = ocr_line_node.as_node().0.as_element()
                                .ok_or_else(|| ())?
                                .attributes
                                .borrow()
                                .get("title")
                                .ok_or_else(|| ())?
                                .to_string();
                        
                                let line_bounds = get_bbox(&infos)
                                .ok_or_else(|| ())?;
            
                                let words = ocr_line_node.as_node()
                                .select(".ocrx_word")
                                .and_then(|ocr_word_node| {
                                    ocr_word_node.map(|ocr_word_node| {
                                        
                                        let infos = ocr_word_node.as_node().0.as_element()
                                        .ok_or_else(|| ())?
                                        .attributes
                                        .borrow()
                                        .get("title")
                                        .ok_or_else(|| ())?
                                        .to_string();
    
                                        let word_bounds = get_bbox(&infos)
                                        .ok_or_else(|| ())?;
                    
                                        let text = ocr_word_node.as_node()
                                        .text_contents()
                                        .trim()
                                        .to_string();
            
                                        Ok(HocrWord {
                                            bounds: word_bounds,
                                            text,
                                            confidence: 100.0,
                                        })
                                    })
                                    .collect::<Result<Vec<_>, _>>()
                                }).unwrap_or_default();
            
                                Ok(HocrLine {
                                    bounds: line_bounds,
                                    words,
                                })
                            })
                            .collect::<Result<Vec<_>, _>>()
                                                        })
                                                        .unwrap_or_default();

                                                Ok(HocrParagraph {
                                                    bounds: paragraph_bounds,
                                                    lines,
                                                })
                                            })
                                            .collect::<Result<Vec<_>, _>>()
                                    })
                                    .unwrap_or_default();

                            Ok(HocrArea {
                                bounds: carea_bounds,
                                paragraphs,
                            })
                        })
                        .collect::<Result<Vec<_>, _>>()
                })
                .unwrap_or_default();

        Ok(Self {
            bounds: page_bounds,
            careas,
        })
    }

    pub fn get_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();

        for ca in self.careas.iter() {
            for pa in ca.paragraphs.iter() {
                for li in pa.lines.iter() {
                    lines.push(
                        li.words
                            .iter()
                            .map(|w| w.text.clone())
                            .collect::<Vec<_>>()
                            .join(" "),
                    );
                }
                lines.push(String::new());
            }
        }

        if lines.last().cloned() == Some(String::new()) {
            lines.pop();
        }

        lines
    }

    pub fn get_text(&self) -> String {
        self.get_lines().join("")
    }
}

// "image "unknown"; bbox 0 0 640 480; ppageno 1; scan_res 70 70"
// parse_info("bbox", s) == Some("0 0 640 480")
fn parse_info<'a>(key: &str, input: &'a str) -> Option<&'a str> {
    input
        .split(';')
        .find_map(|s| {
            if !s.trim().starts_with(key) {
                None
            } else {
                s.split(key).nth(1)
            }
        })
        .map(|r| r.trim())
}

fn get_bbox(s: &str) -> Option<Rect> {
    let bounds_string = parse_info("bbox", s.trim())?;
    let numbers = bounds_string
        .split_whitespace()
        .filter_map(|s| s.parse::<f32>().ok())
        .collect::<Vec<_>>();
    if !numbers.len() == 4 {
        return None;
    }
    Some(Rect {
        min_x: numbers[0],
        min_y: numbers[1],
        max_x: numbers[2],
        max_y: numbers[3],
    })
}

#[test]
fn test_ocr_parsing() {
    let string = r#"
    <div class='ocr_page' id='page_2' title='image "unknown"; bbox 0 0 640 480; ppageno 1; scan_res 70 70'>
    <div class='ocr_carea' id='block_2_1' title="bbox 36 92 618 361">
     <p class='ocr_par' id='par_2_1' lang='eng' title="bbox 36 92 618 184">
      <span class='ocr_line' id='line_2_1' title="bbox 36 92 580 122; baseline 0 -6; x_size 30; x_descenders 6; x_ascenders 6">
       <span class='ocrx_word' id='word_2_1' title='bbox 36 92 96 116; x_wconf 94'>This</span>
       <span class='ocrx_word' id='word_2_2' title='bbox 109 92 129 116; x_wconf 94'>is</span>
       <span class='ocrx_word' id='word_2_3' title='bbox 141 98 156 116; x_wconf 94'>a</span>
       <span class='ocrx_word' id='word_2_4' title='bbox 169 92 201 116; x_wconf 94'>lot</span>
       <span class='ocrx_word' id='word_2_5' title='bbox 212 92 240 116; x_wconf 96'>of</span>
       <span class='ocrx_word' id='word_2_6' title='bbox 251 92 282 116; x_wconf 96'>12</span>
       <span class='ocrx_word' id='word_2_7' title='bbox 296 92 364 122; x_wconf 96'>point</span>
       <span class='ocrx_word' id='word_2_8' title='bbox 374 93 427 116; x_wconf 96'>text</span>
       <span class='ocrx_word' id='word_2_9' title='bbox 437 93 463 116; x_wconf 96'>to</span>
       <span class='ocrx_word' id='word_2_10' title='bbox 474 93 526 116; x_wconf 96'>test</span>
       <span class='ocrx_word' id='word_2_11' title='bbox 536 92 580 116; x_wconf 96'>the</span>
      </span>
      <span class='ocr_line' id='line_2_2' title="bbox 36 126 618 157; baseline 0 -7; x_size 31; x_descenders 7; x_ascenders 6">
       <span class='ocrx_word' id='word_2_12' title='bbox 36 132 81 150; x_wconf 95'>ocr</span>
       <span class='ocrx_word' id='word_2_13' title='bbox 91 126 160 150; x_wconf 96'>code</span>
       <span class='ocrx_word' id='word_2_14' title='bbox 172 126 223 150; x_wconf 95'>and</span>
       <span class='ocrx_word' id='word_2_15' title='bbox 236 132 286 150; x_wconf 95'>see</span>
       <span class='ocrx_word' id='word_2_16' title='bbox 299 126 314 150; x_wconf 92'>if</span>
       <span class='ocrx_word' id='word_2_17' title='bbox 325 126 339 150; x_wconf 96'>it</span>
       <span class='ocrx_word' id='word_2_18' title='bbox 348 126 433 150; x_wconf 96'>works</span>
       <span class='ocrx_word' id='word_2_19' title='bbox 445 132 478 150; x_wconf 75'>on</span>
       <span class='ocrx_word' id='word_2_20' title='bbox 500 126 529 150; x_wconf 75'>all</span>
       <span class='ocrx_word' id='word_2_21' title='bbox 541 127 618 157; x_wconf 96'>types</span>
      </span>
      <span class='ocr_line' id='line_2_3' title="bbox 36 160 223 184; baseline 0 0; x_size 31.214842; x_descenders 7.2148418; x_ascenders 6">
       <span class='ocrx_word' id='word_2_22' title='bbox 36 160 64 184; x_wconf 96'>of</span>
       <span class='ocrx_word' id='word_2_23' title='bbox 72 160 113 184; x_wconf 96'>file</span>
       <span class='ocrx_word' id='word_2_24' title='bbox 123 160 223 184; x_wconf 96'>format.</span>
      </span>
     </p>
 
     <p class='ocr_par' id='par_2_2' lang='eng' title="bbox 36 194 597 361">
      <span class='ocr_line' id='line_2_4' title="bbox 36 194 585 225; baseline 0 -7; x_size 31; x_descenders 7; x_ascenders 6">
       <span class='ocrx_word' id='word_2_25' title='bbox 36 194 91 218; x_wconf 95'>The</span>
       <span class='ocrx_word' id='word_2_26' title='bbox 102 194 177 224; x_wconf 95'>quick</span>
       <span class='ocrx_word' id='word_2_27' title='bbox 189 194 274 218; x_wconf 95'>brown</span>
       <span class='ocrx_word' id='word_2_28' title='bbox 287 194 339 225; x_wconf 96'>dog</span>
       <span class='ocrx_word' id='word_2_29' title='bbox 348 194 456 225; x_wconf 96'>jumped</span>
       <span class='ocrx_word' id='word_2_30' title='bbox 468 200 531 218; x_wconf 96'>over</span>
       <span class='ocrx_word' id='word_2_31' title='bbox 540 194 585 218; x_wconf 96'>the</span>
      </span>
      <span class='ocr_line' id='line_2_5' title="bbox 37 228 585 259; baseline 0 -7; x_size 31; x_descenders 7; x_ascenders 6">
       <span class='ocrx_word' id='word_2_32' title='bbox 37 228 92 259; x_wconf 96'>lazy</span>
       <span class='ocrx_word' id='word_2_33' title='bbox 103 228 153 252; x_wconf 96'>fox.</span>
       <span class='ocrx_word' id='word_2_34' title='bbox 165 228 220 252; x_wconf 96'>The</span>
       <span class='ocrx_word' id='word_2_35' title='bbox 232 228 307 258; x_wconf 95'>quick</span>
       <span class='ocrx_word' id='word_2_36' title='bbox 319 228 404 252; x_wconf 95'>brown</span>
       <span class='ocrx_word' id='word_2_37' title='bbox 417 228 468 259; x_wconf 95'>dog</span>
       <span class='ocrx_word' id='word_2_38' title='bbox 478 228 585 259; x_wconf 95'>jumped</span>
      </span>
      <span class='ocr_line' id='line_2_6' title="bbox 36 262 597 293; baseline 0 -7; x_size 31; x_descenders 7; x_ascenders 6">
       <span class='ocrx_word' id='word_2_39' title='bbox 36 268 99 286; x_wconf 96'>over</span>
       <span class='ocrx_word' id='word_2_40' title='bbox 109 262 153 286; x_wconf 96'>the</span>
       <span class='ocrx_word' id='word_2_41' title='bbox 165 262 221 293; x_wconf 96'>lazy</span>
       <span class='ocrx_word' id='word_2_42' title='bbox 231 262 281 286; x_wconf 96'>fox.</span>
       <span class='ocrx_word' id='word_2_43' title='bbox 294 262 349 286; x_wconf 96'>The</span>
       <span class='ocrx_word' id='word_2_44' title='bbox 360 262 435 292; x_wconf 96'>quick</span>
       <span class='ocrx_word' id='word_2_45' title='bbox 447 262 532 286; x_wconf 95'>brown</span>
       <span class='ocrx_word' id='word_2_46' title='bbox 545 262 597 293; x_wconf 95'>dog</span>
      </span>
      <span class='ocr_line' id='line_2_7' title="bbox 43 296 561 327; baseline 0 -7; x_size 31; x_descenders 7; x_ascenders 6">
       <span class='ocrx_word' id='word_2_47' title='bbox 43 296 150 327; x_wconf 96'>jumped</span>
       <span class='ocrx_word' id='word_2_48' title='bbox 162 302 226 320; x_wconf 96'>over</span>
       <span class='ocrx_word' id='word_2_49' title='bbox 235 296 279 320; x_wconf 96'>the</span>
       <span class='ocrx_word' id='word_2_50' title='bbox 292 296 347 327; x_wconf 96'>lazy</span>
       <span class='ocrx_word' id='word_2_51' title='bbox 357 296 407 320; x_wconf 96'>fox.</span>
       <span class='ocrx_word' id='word_2_52' title='bbox 420 296 475 320; x_wconf 96'>The</span>
       <span class='ocrx_word' id='word_2_53' title='bbox 486 296 561 326; x_wconf 96'>quick</span>
      </span>
      <span class='ocr_line' id='line_2_8' title="bbox 37 330 561 361; baseline 0 -7; x_size 31; x_descenders 7; x_ascenders 6">
       <span class='ocrx_word' id='word_2_54' title='bbox 37 330 122 354; x_wconf 96'>brown</span>
       <span class='ocrx_word' id='word_2_55' title='bbox 135 330 187 361; x_wconf 96'>dog</span>
       <span class='ocrx_word' id='word_2_56' title='bbox 196 330 304 361; x_wconf 96'>jumped</span>
       <span class='ocrx_word' id='word_2_57' title='bbox 316 336 379 354; x_wconf 95'>over</span>
       <span class='ocrx_word' id='word_2_58' title='bbox 388 330 433 354; x_wconf 96'>the</span>
       <span class='ocrx_word' id='word_2_59' title='bbox 445 330 500 361; x_wconf 96'>lazy</span>
       <span class='ocrx_word' id='word_2_60' title='bbox 511 330 561 354; x_wconf 96'>fox.</span>
      </span>
     </p>
    </div>
   </div>
    "#;

    let parsed = ParsedHocr::new(&string).unwrap();

    assert_eq!(
        parsed.get_lines(),
        vec![
            "This is a lot of 12 point text to test the".to_string(),
            "ocr code and see if it works on all types".to_string(),
            "of file format.".to_string(),
            "".to_string(),
            "The quick brown dog jumped over the".to_string(),
            "lazy fox. The quick brown dog jumped".to_string(),
            "over the lazy fox. The quick brown dog".to_string(),
            "jumped over the lazy fox. The quick".to_string(),
            "brown dog jumped over the lazy fox.".to_string(),
        ]
    );
}