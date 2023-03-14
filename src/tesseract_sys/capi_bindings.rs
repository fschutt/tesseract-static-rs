/* automatically generated by rust-bindgen 0.64.0 */

pub type __int64_t = ::std::os::raw::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut ::std::os::raw::c_uchar,
    pub _size: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___sbuf() {
    const UNINIT: ::std::mem::MaybeUninit<__sbuf> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__sbuf>(),
        16usize,
        concat!("Size of: ", stringify!(__sbuf))
    );
    assert_eq!(
        ::std::mem::align_of::<__sbuf>(),
        8usize,
        concat!("Alignment of ", stringify!(__sbuf))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._base) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sbuf),
            "::",
            stringify!(_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._size) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sbuf),
            "::",
            stringify!(_size)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILE {
    pub _p: *mut ::std::os::raw::c_uchar,
    pub _r: ::std::os::raw::c_int,
    pub _w: ::std::os::raw::c_int,
    pub _flags: ::std::os::raw::c_short,
    pub _file: ::std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::std::os::raw::c_int,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub _read: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: fpos_t,
            arg3: ::std::os::raw::c_int,
        ) -> fpos_t,
    >,
    pub _write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::std::os::raw::c_int,
    pub _ubuf: [::std::os::raw::c_uchar; 3usize],
    pub _nbuf: [::std::os::raw::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::std::os::raw::c_int,
    pub _offset: fpos_t,
}
#[test]
fn bindgen_test_layout___sFILE() {
    const UNINIT: ::std::mem::MaybeUninit<__sFILE> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__sFILE>(),
        152usize,
        concat!("Size of: ", stringify!(__sFILE))
    );
    assert_eq!(
        ::std::mem::align_of::<__sFILE>(),
        8usize,
        concat!("Alignment of ", stringify!(__sFILE))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._p) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_p)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._r) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_r)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._w) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_w)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._flags) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._file) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_file)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._bf) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_bf)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._lbfsize) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_lbfsize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._cookie) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_cookie)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._close) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_close)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._read) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_read)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._seek) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_seek)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._write) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_write)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._ub) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_ub)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._extra) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_extra)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._ur) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_ur)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._ubuf) as usize - ptr as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_ubuf)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._nbuf) as usize - ptr as usize },
        119usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_nbuf)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._lb) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_lb)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._blksize) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_blksize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._offset) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_offset)
        )
    );
}
pub type FILE = __sFILE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TessResultRenderer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TessBaseAPI {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TessPageIterator {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TessResultIterator {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TessMutableIterator {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TessChoiceIterator {
    _unused: [u8; 0],
}
pub const TessOcrEngineMode_OEM_TESSERACT_ONLY: TessOcrEngineMode = 0;
pub const TessOcrEngineMode_OEM_LSTM_ONLY: TessOcrEngineMode = 1;
pub const TessOcrEngineMode_OEM_TESSERACT_LSTM_COMBINED: TessOcrEngineMode = 2;
pub const TessOcrEngineMode_OEM_DEFAULT: TessOcrEngineMode = 3;
pub type TessOcrEngineMode = ::std::os::raw::c_uint;
pub const TessPageSegMode_PSM_OSD_ONLY: TessPageSegMode = 0;
pub const TessPageSegMode_PSM_AUTO_OSD: TessPageSegMode = 1;
pub const TessPageSegMode_PSM_AUTO_ONLY: TessPageSegMode = 2;
pub const TessPageSegMode_PSM_AUTO: TessPageSegMode = 3;
pub const TessPageSegMode_PSM_SINGLE_COLUMN: TessPageSegMode = 4;
pub const TessPageSegMode_PSM_SINGLE_BLOCK_VERT_TEXT: TessPageSegMode = 5;
pub const TessPageSegMode_PSM_SINGLE_BLOCK: TessPageSegMode = 6;
pub const TessPageSegMode_PSM_SINGLE_LINE: TessPageSegMode = 7;
pub const TessPageSegMode_PSM_SINGLE_WORD: TessPageSegMode = 8;
pub const TessPageSegMode_PSM_CIRCLE_WORD: TessPageSegMode = 9;
pub const TessPageSegMode_PSM_SINGLE_CHAR: TessPageSegMode = 10;
pub const TessPageSegMode_PSM_SPARSE_TEXT: TessPageSegMode = 11;
pub const TessPageSegMode_PSM_SPARSE_TEXT_OSD: TessPageSegMode = 12;
pub const TessPageSegMode_PSM_RAW_LINE: TessPageSegMode = 13;
pub const TessPageSegMode_PSM_COUNT: TessPageSegMode = 14;
pub type TessPageSegMode = ::std::os::raw::c_uint;
pub const TessPageIteratorLevel_RIL_BLOCK: TessPageIteratorLevel = 0;
pub const TessPageIteratorLevel_RIL_PARA: TessPageIteratorLevel = 1;
pub const TessPageIteratorLevel_RIL_TEXTLINE: TessPageIteratorLevel = 2;
pub const TessPageIteratorLevel_RIL_WORD: TessPageIteratorLevel = 3;
pub const TessPageIteratorLevel_RIL_SYMBOL: TessPageIteratorLevel = 4;
pub type TessPageIteratorLevel = ::std::os::raw::c_uint;
pub const TessPolyBlockType_PT_UNKNOWN: TessPolyBlockType = 0;
pub const TessPolyBlockType_PT_FLOWING_TEXT: TessPolyBlockType = 1;
pub const TessPolyBlockType_PT_HEADING_TEXT: TessPolyBlockType = 2;
pub const TessPolyBlockType_PT_PULLOUT_TEXT: TessPolyBlockType = 3;
pub const TessPolyBlockType_PT_EQUATION: TessPolyBlockType = 4;
pub const TessPolyBlockType_PT_INLINE_EQUATION: TessPolyBlockType = 5;
pub const TessPolyBlockType_PT_TABLE: TessPolyBlockType = 6;
pub const TessPolyBlockType_PT_VERTICAL_TEXT: TessPolyBlockType = 7;
pub const TessPolyBlockType_PT_CAPTION_TEXT: TessPolyBlockType = 8;
pub const TessPolyBlockType_PT_FLOWING_IMAGE: TessPolyBlockType = 9;
pub const TessPolyBlockType_PT_HEADING_IMAGE: TessPolyBlockType = 10;
pub const TessPolyBlockType_PT_PULLOUT_IMAGE: TessPolyBlockType = 11;
pub const TessPolyBlockType_PT_HORZ_LINE: TessPolyBlockType = 12;
pub const TessPolyBlockType_PT_VERT_LINE: TessPolyBlockType = 13;
pub const TessPolyBlockType_PT_NOISE: TessPolyBlockType = 14;
pub const TessPolyBlockType_PT_COUNT: TessPolyBlockType = 15;
pub type TessPolyBlockType = ::std::os::raw::c_uint;
pub const TessOrientation_ORIENTATION_PAGE_UP: TessOrientation = 0;
pub const TessOrientation_ORIENTATION_PAGE_RIGHT: TessOrientation = 1;
pub const TessOrientation_ORIENTATION_PAGE_DOWN: TessOrientation = 2;
pub const TessOrientation_ORIENTATION_PAGE_LEFT: TessOrientation = 3;
pub type TessOrientation = ::std::os::raw::c_uint;
pub const TessParagraphJustification_JUSTIFICATION_UNKNOWN: TessParagraphJustification = 0;
pub const TessParagraphJustification_JUSTIFICATION_LEFT: TessParagraphJustification = 1;
pub const TessParagraphJustification_JUSTIFICATION_CENTER: TessParagraphJustification = 2;
pub const TessParagraphJustification_JUSTIFICATION_RIGHT: TessParagraphJustification = 3;
pub type TessParagraphJustification = ::std::os::raw::c_uint;
pub const TessWritingDirection_WRITING_DIRECTION_LEFT_TO_RIGHT: TessWritingDirection = 0;
pub const TessWritingDirection_WRITING_DIRECTION_RIGHT_TO_LEFT: TessWritingDirection = 1;
pub const TessWritingDirection_WRITING_DIRECTION_TOP_TO_BOTTOM: TessWritingDirection = 2;
pub type TessWritingDirection = ::std::os::raw::c_uint;
pub const TessTextlineOrder_TEXTLINE_ORDER_LEFT_TO_RIGHT: TessTextlineOrder = 0;
pub const TessTextlineOrder_TEXTLINE_ORDER_RIGHT_TO_LEFT: TessTextlineOrder = 1;
pub const TessTextlineOrder_TEXTLINE_ORDER_TOP_TO_BOTTOM: TessTextlineOrder = 2;
pub type TessTextlineOrder = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ETEXT_DESC {
    _unused: [u8; 0],
}
pub type TessCancelFunc = ::std::option::Option<
    unsafe extern "C" fn(
        cancel_this: *mut ::std::os::raw::c_void,
        words: ::std::os::raw::c_int,
    ) -> bool,
>;
pub type TessProgressFunc = ::std::option::Option<
    unsafe extern "C" fn(
        ths: *mut ETEXT_DESC,
        left: ::std::os::raw::c_int,
        right: ::std::os::raw::c_int,
        top: ::std::os::raw::c_int,
        bottom: ::std::os::raw::c_int,
    ) -> bool,
>;
extern "C" {
    pub fn TessVersion() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessDeleteText(text: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn TessDeleteTextArray(arr: *mut *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn TessDeleteIntArray(arr: *const ::std::os::raw::c_int);
}
extern "C" {
    pub fn TessTextRendererCreate(
        outputbase: *const ::std::os::raw::c_char,
    ) -> *mut TessResultRenderer;
}
extern "C" {
    pub fn TessHOcrRendererCreate(
        outputbase: *const ::std::os::raw::c_char,
    ) -> *mut TessResultRenderer;
}
extern "C" {
    pub fn TessHOcrRendererCreate2(
        outputbase: *const ::std::os::raw::c_char,
        font_info: ::std::os::raw::c_int,
    ) -> *mut TessResultRenderer;
}
extern "C" {
    pub fn TessAltoRendererCreate(
        outputbase: *const ::std::os::raw::c_char,
    ) -> *mut TessResultRenderer;
}
extern "C" {
    pub fn TessTsvRendererCreate(
        outputbase: *const ::std::os::raw::c_char,
    ) -> *mut TessResultRenderer;
}
extern "C" {
    pub fn TessPDFRendererCreate(
        outputbase: *const ::std::os::raw::c_char,
        datadir: *const ::std::os::raw::c_char,
        textonly: ::std::os::raw::c_int,
    ) -> *mut TessResultRenderer;
}
extern "C" {
    pub fn TessUnlvRendererCreate(
        outputbase: *const ::std::os::raw::c_char,
    ) -> *mut TessResultRenderer;
}
extern "C" {
    pub fn TessBoxTextRendererCreate(
        outputbase: *const ::std::os::raw::c_char,
    ) -> *mut TessResultRenderer;
}
extern "C" {
    pub fn TessLSTMBoxRendererCreate(
        outputbase: *const ::std::os::raw::c_char,
    ) -> *mut TessResultRenderer;
}
extern "C" {
    pub fn TessWordStrBoxRendererCreate(
        outputbase: *const ::std::os::raw::c_char,
    ) -> *mut TessResultRenderer;
}
extern "C" {
    pub fn TessDeleteResultRenderer(renderer: *mut TessResultRenderer);
}
extern "C" {
    pub fn TessResultRendererInsert(
        renderer: *mut TessResultRenderer,
        next: *mut TessResultRenderer,
    );
}
extern "C" {
    pub fn TessResultRendererNext(renderer: *mut TessResultRenderer) -> *mut TessResultRenderer;
}
extern "C" {
    pub fn TessResultRendererBeginDocument(
        renderer: *mut TessResultRenderer,
        title: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessResultRendererAddImage(
        renderer: *mut TessResultRenderer,
        api: *mut TessBaseAPI,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessResultRendererEndDocument(
        renderer: *mut TessResultRenderer,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessResultRendererExtention(
        renderer: *mut TessResultRenderer,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessResultRendererTitle(
        renderer: *mut TessResultRenderer,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessResultRendererImageNum(renderer: *mut TessResultRenderer) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPICreate() -> *mut TessBaseAPI;
}
extern "C" {
    pub fn TessBaseAPIDelete(handle: *mut TessBaseAPI);
}
extern "C" {
    pub fn TessBaseAPIGetOpenCLDevice(
        handle: *mut TessBaseAPI,
        device: *mut *mut ::std::os::raw::c_void,
    ) -> usize;
}
extern "C" {
    pub fn TessBaseAPISetInputName(handle: *mut TessBaseAPI, name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn TessBaseAPIGetInputName(handle: *mut TessBaseAPI) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPISetInputImage(handle: *mut TessBaseAPI, pix: *mut Pix);
}
extern "C" {
    pub fn TessBaseAPIGetInputImage(handle: *mut TessBaseAPI) -> *mut Pix;
}
extern "C" {
    pub fn TessBaseAPIGetSourceYResolution(handle: *mut TessBaseAPI) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIGetDatapath(handle: *mut TessBaseAPI) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPISetOutputName(handle: *mut TessBaseAPI, name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn TessBaseAPISetVariable(
        handle: *mut TessBaseAPI,
        name: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPISetDebugVariable(
        handle: *mut TessBaseAPI,
        name: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIGetIntVariable(
        handle: *const TessBaseAPI,
        name: *const ::std::os::raw::c_char,
        value: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIGetBoolVariable(
        handle: *const TessBaseAPI,
        name: *const ::std::os::raw::c_char,
        value: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIGetDoubleVariable(
        handle: *const TessBaseAPI,
        name: *const ::std::os::raw::c_char,
        value: *mut f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIGetStringVariable(
        handle: *const TessBaseAPI,
        name: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPIPrintVariables(handle: *const TessBaseAPI, fp: *mut FILE);
}
extern "C" {
    pub fn TessBaseAPIPrintVariablesToFile(
        handle: *const TessBaseAPI,
        filename: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIInit1(
        handle: *mut TessBaseAPI,
        datapath: *const ::std::os::raw::c_char,
        language: *const ::std::os::raw::c_char,
        oem: TessOcrEngineMode,
        configs: *mut *mut ::std::os::raw::c_char,
        configs_size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIInit2(
        handle: *mut TessBaseAPI,
        datapath: *const ::std::os::raw::c_char,
        language: *const ::std::os::raw::c_char,
        oem: TessOcrEngineMode,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIInit3(
        handle: *mut TessBaseAPI,
        datapath: *const ::std::os::raw::c_char,
        language: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIInit4(
        handle: *mut TessBaseAPI,
        datapath: *const ::std::os::raw::c_char,
        language: *const ::std::os::raw::c_char,
        mode: TessOcrEngineMode,
        configs: *mut *mut ::std::os::raw::c_char,
        configs_size: ::std::os::raw::c_int,
        vars_vec: *mut *mut ::std::os::raw::c_char,
        vars_values: *mut *mut ::std::os::raw::c_char,
        vars_vec_size: usize,
        set_only_non_debug_params: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIInit5(
        handle: *mut TessBaseAPI,
        data: *const ::std::os::raw::c_char,
        data_size: ::std::os::raw::c_int,
        language: *const ::std::os::raw::c_char,
        mode: TessOcrEngineMode,
        configs: *mut *mut ::std::os::raw::c_char,
        configs_size: ::std::os::raw::c_int,
        vars_vec: *mut *mut ::std::os::raw::c_char,
        vars_values: *mut *mut ::std::os::raw::c_char,
        vars_vec_size: usize,
        set_only_non_debug_params: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIGetInitLanguagesAsString(
        handle: *const TessBaseAPI,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPIGetLoadedLanguagesAsVector(
        handle: *const TessBaseAPI,
    ) -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPIGetAvailableLanguagesAsVector(
        handle: *const TessBaseAPI,
    ) -> *mut *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPIInitForAnalysePage(handle: *mut TessBaseAPI);
}
extern "C" {
    pub fn TessBaseAPIReadConfigFile(
        handle: *mut TessBaseAPI,
        filename: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn TessBaseAPIReadDebugConfigFile(
        handle: *mut TessBaseAPI,
        filename: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn TessBaseAPISetPageSegMode(handle: *mut TessBaseAPI, mode: TessPageSegMode);
}
extern "C" {
    pub fn TessBaseAPIGetPageSegMode(handle: *const TessBaseAPI) -> TessPageSegMode;
}
extern "C" {
    pub fn TessBaseAPIRect(
        handle: *mut TessBaseAPI,
        imagedata: *const ::std::os::raw::c_uchar,
        bytes_per_pixel: ::std::os::raw::c_int,
        bytes_per_line: ::std::os::raw::c_int,
        left: ::std::os::raw::c_int,
        top: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPIClearAdaptiveClassifier(handle: *mut TessBaseAPI);
}
extern "C" {
    pub fn TessBaseAPISetImage(
        handle: *mut TessBaseAPI,
        imagedata: *const ::std::os::raw::c_uchar,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        bytes_per_pixel: ::std::os::raw::c_int,
        bytes_per_line: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn TessBaseAPISetImage2(handle: *mut TessBaseAPI, pix: *mut Pix);
}
extern "C" {
    pub fn TessBaseAPISetSourceResolution(handle: *mut TessBaseAPI, ppi: ::std::os::raw::c_int);
}
extern "C" {
    pub fn TessBaseAPISetRectangle(
        handle: *mut TessBaseAPI,
        left: ::std::os::raw::c_int,
        top: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn TessBaseAPIGetThresholdedImage(handle: *mut TessBaseAPI) -> *mut Pix;
}
extern "C" {
    pub fn TessBaseAPIGetRegions(handle: *mut TessBaseAPI, pixa: *mut *mut Pixa) -> *mut Boxa;
}
extern "C" {
    pub fn TessBaseAPIGetTextlines(
        handle: *mut TessBaseAPI,
        pixa: *mut *mut Pixa,
        blockids: *mut *mut ::std::os::raw::c_int,
    ) -> *mut Boxa;
}
extern "C" {
    pub fn TessBaseAPIGetTextlines1(
        handle: *mut TessBaseAPI,
        raw_image: ::std::os::raw::c_int,
        raw_padding: ::std::os::raw::c_int,
        pixa: *mut *mut Pixa,
        blockids: *mut *mut ::std::os::raw::c_int,
        paraids: *mut *mut ::std::os::raw::c_int,
    ) -> *mut Boxa;
}
extern "C" {
    pub fn TessBaseAPIGetStrips(
        handle: *mut TessBaseAPI,
        pixa: *mut *mut Pixa,
        blockids: *mut *mut ::std::os::raw::c_int,
    ) -> *mut Boxa;
}
extern "C" {
    pub fn TessBaseAPIGetWords(handle: *mut TessBaseAPI, pixa: *mut *mut Pixa) -> *mut Boxa;
}
extern "C" {
    pub fn TessBaseAPIGetConnectedComponents(
        handle: *mut TessBaseAPI,
        cc: *mut *mut Pixa,
    ) -> *mut Boxa;
}
extern "C" {
    pub fn TessBaseAPIGetComponentImages(
        handle: *mut TessBaseAPI,
        level: TessPageIteratorLevel,
        text_only: ::std::os::raw::c_int,
        pixa: *mut *mut Pixa,
        blockids: *mut *mut ::std::os::raw::c_int,
    ) -> *mut Boxa;
}
extern "C" {
    pub fn TessBaseAPIGetComponentImages1(
        handle: *mut TessBaseAPI,
        level: TessPageIteratorLevel,
        text_only: ::std::os::raw::c_int,
        raw_image: ::std::os::raw::c_int,
        raw_padding: ::std::os::raw::c_int,
        pixa: *mut *mut Pixa,
        blockids: *mut *mut ::std::os::raw::c_int,
        paraids: *mut *mut ::std::os::raw::c_int,
    ) -> *mut Boxa;
}
extern "C" {
    pub fn TessBaseAPIGetThresholdedImageScaleFactor(
        handle: *const TessBaseAPI,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIAnalyseLayout(handle: *mut TessBaseAPI) -> *mut TessPageIterator;
}
extern "C" {
    pub fn TessBaseAPIRecognize(
        handle: *mut TessBaseAPI,
        monitor: *mut ETEXT_DESC,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIProcessPages(
        handle: *mut TessBaseAPI,
        filename: *const ::std::os::raw::c_char,
        retry_config: *const ::std::os::raw::c_char,
        timeout_millisec: ::std::os::raw::c_int,
        renderer: *mut TessResultRenderer,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIProcessPage(
        handle: *mut TessBaseAPI,
        pix: *mut Pix,
        page_index: ::std::os::raw::c_int,
        filename: *const ::std::os::raw::c_char,
        retry_config: *const ::std::os::raw::c_char,
        timeout_millisec: ::std::os::raw::c_int,
        renderer: *mut TessResultRenderer,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIGetIterator(handle: *mut TessBaseAPI) -> *mut TessResultIterator;
}
extern "C" {
    pub fn TessBaseAPIGetMutableIterator(handle: *mut TessBaseAPI) -> *mut TessMutableIterator;
}
extern "C" {
    pub fn TessBaseAPIGetUTF8Text(handle: *mut TessBaseAPI) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPIGetHOCRText(
        handle: *mut TessBaseAPI,
        page_number: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPIGetAltoText(
        handle: *mut TessBaseAPI,
        page_number: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPIGetTsvText(
        handle: *mut TessBaseAPI,
        page_number: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPIGetBoxText(
        handle: *mut TessBaseAPI,
        page_number: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPIGetLSTMBoxText(
        handle: *mut TessBaseAPI,
        page_number: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPIGetWordStrBoxText(
        handle: *mut TessBaseAPI,
        page_number: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPIGetUNLVText(handle: *mut TessBaseAPI) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPIMeanTextConf(handle: *mut TessBaseAPI) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIAllWordConfidences(handle: *mut TessBaseAPI) -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIAdaptToWordStr(
        handle: *mut TessBaseAPI,
        mode: TessPageSegMode,
        wordstr: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIClear(handle: *mut TessBaseAPI);
}
extern "C" {
    pub fn TessBaseAPIEnd(handle: *mut TessBaseAPI);
}
extern "C" {
    pub fn TessBaseAPIIsValidWord(
        handle: *mut TessBaseAPI,
        word: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIGetTextDirection(
        handle: *mut TessBaseAPI,
        out_offset: *mut ::std::os::raw::c_int,
        out_slope: *mut f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIGetUnichar(
        handle: *mut TessBaseAPI,
        unichar_id: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessBaseAPIClearPersistentCache(handle: *mut TessBaseAPI);
}
extern "C" {
    pub fn TessBaseAPIDetectOrientationScript(
        handle: *mut TessBaseAPI,
        orient_deg: *mut ::std::os::raw::c_int,
        orient_conf: *mut f32,
        script_name: *mut *const ::std::os::raw::c_char,
        script_conf: *mut f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPISetMinOrientationMargin(handle: *mut TessBaseAPI, margin: f64);
}
extern "C" {
    pub fn TessBaseAPINumDawgs(handle: *const TessBaseAPI) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessBaseAPIOem(handle: *const TessBaseAPI) -> TessOcrEngineMode;
}
extern "C" {
    pub fn TessBaseGetBlockTextOrientations(
        handle: *mut TessBaseAPI,
        block_orientation: *mut *mut ::std::os::raw::c_int,
        vertical_writing: *mut *mut bool,
    );
}
extern "C" {
    pub fn TessPageIteratorDelete(handle: *mut TessPageIterator);
}
extern "C" {
    pub fn TessPageIteratorCopy(handle: *const TessPageIterator) -> *mut TessPageIterator;
}
extern "C" {
    pub fn TessPageIteratorBegin(handle: *mut TessPageIterator);
}
extern "C" {
    pub fn TessPageIteratorNext(
        handle: *mut TessPageIterator,
        level: TessPageIteratorLevel,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessPageIteratorIsAtBeginningOf(
        handle: *const TessPageIterator,
        level: TessPageIteratorLevel,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessPageIteratorIsAtFinalElement(
        handle: *const TessPageIterator,
        level: TessPageIteratorLevel,
        element: TessPageIteratorLevel,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessPageIteratorBoundingBox(
        handle: *const TessPageIterator,
        level: TessPageIteratorLevel,
        left: *mut ::std::os::raw::c_int,
        top: *mut ::std::os::raw::c_int,
        right: *mut ::std::os::raw::c_int,
        bottom: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessPageIteratorBlockType(handle: *const TessPageIterator) -> TessPolyBlockType;
}
extern "C" {
    pub fn TessPageIteratorGetBinaryImage(
        handle: *const TessPageIterator,
        level: TessPageIteratorLevel,
    ) -> *mut Pix;
}
extern "C" {
    pub fn TessPageIteratorGetImage(
        handle: *const TessPageIterator,
        level: TessPageIteratorLevel,
        padding: ::std::os::raw::c_int,
        original_image: *mut Pix,
        left: *mut ::std::os::raw::c_int,
        top: *mut ::std::os::raw::c_int,
    ) -> *mut Pix;
}
extern "C" {
    pub fn TessPageIteratorBaseline(
        handle: *const TessPageIterator,
        level: TessPageIteratorLevel,
        x1: *mut ::std::os::raw::c_int,
        y1: *mut ::std::os::raw::c_int,
        x2: *mut ::std::os::raw::c_int,
        y2: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessPageIteratorOrientation(
        handle: *mut TessPageIterator,
        orientation: *mut TessOrientation,
        writing_direction: *mut TessWritingDirection,
        textline_order: *mut TessTextlineOrder,
        deskew_angle: *mut f32,
    );
}
extern "C" {
    pub fn TessPageIteratorParagraphInfo(
        handle: *mut TessPageIterator,
        justification: *mut TessParagraphJustification,
        is_list_item: *mut ::std::os::raw::c_int,
        is_crown: *mut ::std::os::raw::c_int,
        first_line_indent: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn TessResultIteratorDelete(handle: *mut TessResultIterator);
}
extern "C" {
    pub fn TessResultIteratorCopy(handle: *const TessResultIterator) -> *mut TessResultIterator;
}
extern "C" {
    pub fn TessResultIteratorGetPageIterator(
        handle: *mut TessResultIterator,
    ) -> *mut TessPageIterator;
}
extern "C" {
    pub fn TessResultIteratorGetPageIteratorConst(
        handle: *const TessResultIterator,
    ) -> *const TessPageIterator;
}
extern "C" {
    pub fn TessResultIteratorGetChoiceIterator(
        handle: *const TessResultIterator,
    ) -> *mut TessChoiceIterator;
}
extern "C" {
    pub fn TessResultIteratorNext(
        handle: *mut TessResultIterator,
        level: TessPageIteratorLevel,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessResultIteratorGetUTF8Text(
        handle: *const TessResultIterator,
        level: TessPageIteratorLevel,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessResultIteratorConfidence(
        handle: *const TessResultIterator,
        level: TessPageIteratorLevel,
    ) -> f32;
}
extern "C" {
    pub fn TessResultIteratorWordRecognitionLanguage(
        handle: *const TessResultIterator,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessResultIteratorWordFontAttributes(
        handle: *const TessResultIterator,
        is_bold: *mut ::std::os::raw::c_int,
        is_italic: *mut ::std::os::raw::c_int,
        is_underlined: *mut ::std::os::raw::c_int,
        is_monospace: *mut ::std::os::raw::c_int,
        is_serif: *mut ::std::os::raw::c_int,
        is_smallcaps: *mut ::std::os::raw::c_int,
        pointsize: *mut ::std::os::raw::c_int,
        font_id: *mut ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessResultIteratorWordIsFromDictionary(
        handle: *const TessResultIterator,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessResultIteratorWordIsNumeric(
        handle: *const TessResultIterator,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessResultIteratorSymbolIsSuperscript(
        handle: *const TessResultIterator,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessResultIteratorSymbolIsSubscript(
        handle: *const TessResultIterator,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessResultIteratorSymbolIsDropcap(
        handle: *const TessResultIterator,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessChoiceIteratorDelete(handle: *mut TessChoiceIterator);
}
extern "C" {
    pub fn TessChoiceIteratorNext(handle: *mut TessChoiceIterator) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessChoiceIteratorGetUTF8Text(
        handle: *const TessChoiceIterator,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TessChoiceIteratorConfidence(handle: *const TessChoiceIterator) -> f32;
}
extern "C" {
    pub fn TessMonitorCreate() -> *mut ETEXT_DESC;
}
extern "C" {
    pub fn TessMonitorDelete(monitor: *mut ETEXT_DESC);
}
extern "C" {
    pub fn TessMonitorSetCancelFunc(monitor: *mut ETEXT_DESC, cancelFunc: TessCancelFunc);
}
extern "C" {
    pub fn TessMonitorSetCancelThis(
        monitor: *mut ETEXT_DESC,
        cancelThis: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn TessMonitorGetCancelThis(monitor: *mut ETEXT_DESC) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn TessMonitorSetProgressFunc(monitor: *mut ETEXT_DESC, progressFunc: TessProgressFunc);
}
extern "C" {
    pub fn TessMonitorGetProgress(monitor: *mut ETEXT_DESC) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TessMonitorSetDeadlineMSecs(monitor: *mut ETEXT_DESC, deadline: ::std::os::raw::c_int);
}
