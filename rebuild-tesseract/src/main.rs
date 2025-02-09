use std::env;
use std::path::{Path, PathBuf};
use fs_extra::dir::CopyOptions;

/// Function to download leptonica from GitHub, will not re-compile if already cached
pub fn download_leptonica() -> PathBuf {
    let source = "https://github.com/DanBloomberg/leptonica/releases/download/1.85.0/leptonica-1.85.0.tar.gz";
    let target = std::env::current_exe().unwrap().parent().unwrap().join("leptonica").to_path_buf();
    if !target.exists() {
        std::fs::create_dir_all(&target).unwrap();
        download_and_unpack(source, &target)
    } else {
        target.clone()
    }
}

/// Function to compile leptonica, will not re-compile if already cached
pub fn compile_leptonica(source_dir: &Path) -> (PathBuf, Vec<PathBuf>) {
    let out_dir = std::env::current_exe().unwrap().parent().expect("no out dir").to_path_buf();
    let base_dir = Path::new(&out_dir).join("leptonica");

    let _ = std::fs::create_dir_all(&base_dir);

    let _ = fs_extra::dir::copy(&source_dir, &base_dir, &CopyOptions::default());

    let base_dir = base_dir
        .join("leptonica-1.85.0");

    let expected_library_path = Path::new(&out_dir).join("leptonica.dll");

    if expected_library_path.exists() { // library already built
        let ret2 = vec![Path::new(&out_dir).join("include").join("leptonica")];
        return (expected_library_path, ret2);
    }

    eprintln!("base dir {}", base_dir.display());

    // Early exit if dir already exists
    // Disable all image I/O except bmp and pnm files
    let environ_h_path = base_dir.join("src").join("environ.h");
    let environ_h = std::fs::read_to_string(&environ_h_path)
        .unwrap()
        .replace(
            "#define  HAVE_LIBZ          1",
            "#define  HAVE_LIBZ          0",
        )
        .replace(
            "#ifdef  NO_CONSOLE_IO",
            "#define NO_CONSOLE_IO\n#ifdef  NO_CONSOLE_IO",
        );
    std::fs::write(environ_h_path, environ_h).unwrap();

    // configure cmake/Configure.cmake
    let configure_cmake_path = base_dir.join("cmake").join("Configure.cmake");
    std::fs::write(
        configure_cmake_path,
        include_bytes!("./leptonica-CmakeLists.txt"),
    )
    .unwrap();

    // Remove png, jpen, etc. from makefile.static
    let makefile_static_path = base_dir.join("prog").join("makefile.static");
    let makefile_static = std::fs::read_to_string(&makefile_static_path)
        .unwrap()
        .replace(
            "ALL_LIBS =	$(LEPTLIB) -ltiff -ljpeg -lpng -lz -lm",
            "ALL_LIBS =	$(LEPTLIB) -lm",
        );
    std::fs::write(makefile_static_path, makefile_static).unwrap();

    // Edit endianness.h to set endian
    #[cfg(target_endian = "big")]
    let target_endian = "#define L_BIG_ENDIAN\n";
    #[cfg(target_endian = "little")]
    let target_endian = "#define L_LITTLE_ENDIAN\n";
    std::fs::write(base_dir.join("src").join("endianness.h"), target_endian).unwrap();

    let dst = cmake::Config::new(&base_dir)
        .profile("Release")
        .always_configure(true)
        .configure_arg("-DBUILD_PROG=OFF")
        .configure_arg("-DSW_BUILD=OFF")
        .configure_arg("-DBUILD_SHARED_LIBS=ON")
        .configure_arg("-DENABLE_PNG=OFF")
        .configure_arg("-DENABLE_GIF=OFF")
        .configure_arg("-DENABLE_JPEG=OFF")
        .configure_arg("-DENABLE_TIFF=OFF")
        .configure_arg("-DENABLE_WEBP=OFF")
        .configure_arg("-DENABLE_OPENJPEG=OFF")
        .configure_arg("-DHAVE_LIBZ=0")
        .configure_arg("-DENABLE_LTO=OFF")
        .configure_arg("-DNO_CONSOLE_IO=ON")
        .build();

    #[cfg(target_os = "windows")]
    let library_path = dst.join("bin");
    #[cfg(not(target_os = "windows"))]
    let library_path = dst.join("lib");

    let library_path = library_path
    .join({
        #[cfg(target_os = "macos")] { "libleptonica.dylib" }
        #[cfg(target_os = "linux")] { "libleptonica.so" }
        #[cfg(target_os = "windows")] { "leptonica-1.85.0.dll" }
    });

    if !library_path.exists() {
        let files = list_files(&dst);
        for f in files {
            println!("{}", f.display());
        }
        panic!("leptonica DLL not found in {}", library_path.display());
    } else {
        let library_path = library_path
        .canonicalize()
        .unwrap();

        let bytes = std::fs::read(&library_path).unwrap();

        std::fs::write(Path::new(&out_dir).join("leptonica.dll"), &bytes).unwrap();
 
        (library_path, vec![dst.join("include").join("leptonica")])
    }
}

/// Function to download tesseract from GitHub to OUT_DIR, will not re-download if already cached
pub fn download_tesseract() -> PathBuf {
    let source = "https://github.com/tesseract-ocr/tesseract/archive/refs/tags/5.5.0.tar.gz";
    let target = std::env::current_exe().unwrap().parent().unwrap().join("tesseract").to_path_buf();;
    if !target.exists() {
        std::fs::create_dir_all(&target).unwrap();
        download_and_unpack(source, &target)
    } else {
        target.clone()
    }
}


fn _list_files(vec: &mut Vec<PathBuf>, path: &Path) {
    use std::fs::metadata;
    use std::fs;
    if metadata(&path).unwrap().is_dir() {
        let paths = fs::read_dir(&path).unwrap();
        for path_result in paths {
            let full_path = path_result.unwrap().path();
            if metadata(&full_path).unwrap().is_dir() {
                _list_files(vec, &full_path);
            } else {
                vec.push(full_path);
            }
        }
    }
}

fn list_files(path: &Path) -> Vec<PathBuf> {
    let mut vec = Vec::new();
    _list_files(&mut vec,&path);
    vec
}

/// Will download and unpack a URL to a path, WARNING: no caching!
pub fn download_and_unpack(url: &str, target: &PathBuf) -> PathBuf {
    use flate2::read::GzDecoder;
    use std::fs::File;
    use tar::Archive;

    let client = reqwest::blocking::Client::builder()
    .timeout(std::time::Duration::from_secs(60))
    .build().unwrap();
    let body = client.get(url).send().unwrap().bytes().unwrap();
    std::fs::write(target.join("out.tar.gz"), body.as_ref()).unwrap();
    let tar_gz = File::open(&target.join("out.tar.gz")).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(target).unwrap();
    target.clone()
}

/// Function to compile tesseract into OUT_DIR, will not re-compile if cached
/// 
/// Set `disable_avx` to true if encountering build problems in a VM.
pub fn compile_tesseract(source_dir: &Path, disable_avx: bool) -> (PathBuf, Vec<PathBuf>) {
    let out_dir = std::env::current_exe().unwrap().parent().expect("no out dir").to_path_buf();
    let base_dir = Path::new(&out_dir).join("tesseract");

    let _ = std::fs::create_dir_all(&base_dir);

    let _ = fs_extra::dir::copy(&source_dir, &base_dir, &CopyOptions::default());

    let base_dir = base_dir.join("tesseract").join("tesseract-5.5.0");

    let expected_library_path = Path::new(&out_dir).join("tesseract.dll");
    if expected_library_path.exists() {
        let ret2 = vec![base_dir.join("include").join("tesseract")];
        return (expected_library_path, ret2);
    }

    let cmakelists = std::fs::read_to_string(base_dir.join("CMakeLists.txt"))
        .unwrap()
        .replace("set(HAVE_TIFFIO_H ON)", "");

    // If run in a VM, disable AVX extensions
    let cmakelists = if disable_avx {
        let off = &[
            "set(HAVE_AVX FALSE)",
            "set(HAVE_AVX2 FALSE)",
            "set(HAVE_AVX512F FALSE)",
            "set(HAVE_FMA FALSE)",
            "set(HAVE_NEON FALSE)",
            "set(HAVE_SSE4_1 FALSE)",
        ].join("\r\n");
    
        let line = "endif(CMAKE_SYSTEM_PROCESSOR MATCHES \"x86|x86_64|AMD64|amd64|i386|i686\")";
    
        cmakelists.replace(line, &(line.to_string() + "\r\n" + off))
    } else {
        cmakelists
    };

    std::fs::write(base_dir.join("CMakeLists.txt"), cmakelists).unwrap();

    if disable_avx {

        let line2 = "#if defined(HAVE_AVX) || defined(HAVE_AVX2) || defined(HAVE_FMA) || defined(HAVE_SSE4_1)";
        let off2 = &[
            "#undef HAVE_AVX",
            "#undef HAVE_AVX2",
            "#undef HAVE_FMA",
            "#undef HAVE_SSE4_1",
            "#undef HAVE_AVX512F",
            "#undef HAVE_NEON",
            "#undef HAVE_RVV",
        ].join("\r\n");

        let simddetect = std::fs::read_to_string(base_dir.join("src").join("arch").join("simddetect.cpp"))
            .unwrap()
            .replace(line2, &(off2.to_string() + "\r\n" + line2 + "\r\n"));
        std::fs::write(base_dir.join("src").join("arch").join("simddetect.cpp"), simddetect).unwrap();
    }

    let dst = cmake::Config::new(&base_dir)
        .profile("Release")
        .always_configure(true)
        .configure_arg("-DHAVE_LIBARCHIVE=OFF")
        .configure_arg("-DHAVELIBCURL=OFF")
        .configure_arg("-DHAVE_TIFFIO_H=OFF")
        .configure_arg("-DGRAPHICS_DISABLED=ON")
        .configure_arg("-DDISABLED_LEGACY_ENGINE=ON")
        .configure_arg("-DUSE_OPENCL=OFF")
        .configure_arg("-DOPENMP_BUILD=OFF")
        .configure_arg("-DBUILD_TRAINING_TOOLS=OFF")
        .configure_arg("-DBUILD_TESTS=OFF")
        .configure_arg("-DBUILD_SHARED_LIBS=ON")
        .configure_arg("-DENABLE_LTO=OFF")
        .configure_arg("-DDISABLE_ARCHIVE=ON")
        .configure_arg("-DDISABLE_CURL=ON")
        .configure_arg("-DBUILD_PROG=OFF")
        .configure_arg("-DSW_BUILD=OFF")
        .configure_arg("-DUSE_SYSTEM_ICU=OFF")
        .configure_arg("-DINSTALL_CONFIGS=ON") // OFF?
        .configure_arg("-DFAST_FLOAT=ON")
        .configure_arg("-DENABLE_OPENCL=OFF")
        .build();

    eprintln!("library path tesseract {}", dst.display());

    #[cfg(target_os = "windows")]
    let library_path = dst.join("bin");
    #[cfg(not(target_os = "windows"))]
    let library_path = dst.join("lib");

    let library_path = library_path
    .join({
        #[cfg(target_os = "macos")] { "libtesseract.dylib" }
        #[cfg(target_os = "linux")] { "libtesseract.so" }
        #[cfg(target_os = "windows")] { "tesseract55.dll" }
    });
 
    if !library_path.exists() {
        let files = list_files(&dst);
        for f in files {
            println!("{}", f.display());
        }
        panic!("tesseract DLL not found in {}", library_path.display());
    } else {
        let library_path = library_path
        .canonicalize()
        .unwrap();

        let bytes = std::fs::read(&library_path).unwrap();

        std::fs::write(Path::new(&out_dir).join("tesseract.dll"), &bytes).unwrap();
 
        (library_path, vec![dst.join("include").join("tesseract")])
    }
}

/// NOTE: This function should be called from a build.rs script and 
/// has to run on EVERY build (it caches results automatically internally)
fn main() {

    let target = env!("TARGET");
    std::env::set_var("TARGET", target);

    let target = env!("HOST");
    std::env::set_var("HOST", target);

    let (_, _leptonica_includes) =
        compile_leptonica(&download_leptonica());
    let (_, _tesseract_includes) =
        compile_tesseract(&download_tesseract(), cfg!(feature = "disable_avx"));
}
