use fs_extra::dir::CopyOptions;
use std::env;
use std::path::{Path, PathBuf};

pub fn download_leptonica() -> PathBuf {
    let source = "https://github.com/fschutt/tesseract-static-rs/files/11274620/leptonica-1b1d7ade6d753e8af2d023cff15d7862cd7b8413.tar.gz";
    let target = Path::new(&env::var("OUT_DIR").unwrap()).join("leptonica");
    std::fs::create_dir_all(&target).unwrap();
    download_and_unpack(source, &target)
}

pub fn compile_leptonica(source_dir: &Path) -> (PathBuf, Vec<PathBuf>) {
    let out_dir = std::env::var("OUT_DIR").expect("no out dir");
    let base_dir = Path::new(&out_dir).join("leptonica");

    let _ = std::fs::create_dir_all(&base_dir);

    let _ = fs_extra::dir::copy(&source_dir, &base_dir, &CopyOptions::default());

    let base_dir = base_dir
        .join("leptonica")
        .join("leptonica-1b1d7ade6d753e8af2d023cff15d7862cd7b8413");

    eprintln!("base dir {}", base_dir.display());

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
        include_bytes!("../leptonica-CmakeLists.txt"),
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
        .always_configure(true)
        .configure_arg("-DBUILD_PROG=OFF")
        .configure_arg("-DSW_BUILD=OFF")
        .configure_arg("-DBUILD_SHARED_LIBS=OFF")
        .configure_arg("-DENABLE_PNG=OFF")
        .configure_arg("-DENABLE_GIF=OFF")
        .configure_arg("-DENABLE_JPEG=OFF")
        .configure_arg("-DENABLE_TIFF=OFF")
        .configure_arg("-DENABLE_WEBP=OFF")
        .configure_arg("-DENABLE_OPENJPEG=OFF")
        .configure_arg("-DHAVE_LIBZ=0")
        .configure_arg("-DNO_CONSOLE_IO=ON")
        .build();

    eprintln!("library path {}", dst.display());

    let library_path = dst
        .join("lib")
        .join({
            #[cfg(not(target_os = "windows"))]
            {
                "libleptonica.a"
            }
            #[cfg(target_os = "windows")]
            {
                "leptonica-1.84.0.lib"
            }
        })
        .canonicalize()
        .unwrap();

    (library_path, vec![dst.join("include").join("leptonica")])
}

pub fn download_tesseract() -> PathBuf {
    let source = "https://github.com/tesseract-ocr/tesseract/archive/refs/tags/5.3.0.tar.gz";
    let target = Path::new(&env::var("OUT_DIR").unwrap()).join("tesseract");
    std::fs::create_dir_all(&target).unwrap();
    download_and_unpack(source, &target)
}

pub fn download_and_unpack(url: &str, target: &PathBuf) -> PathBuf {
    use flate2::read::GzDecoder;
    use std::fs::File;
    use tar::Archive;

    let body = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    std::fs::write(target.join("out.tar.gz"), body.as_ref()).unwrap();
    let tar_gz = File::open(&target.join("out.tar.gz")).unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(target).unwrap();
    target.clone()
}

pub fn compile_tesseract(source_dir: &Path) -> (PathBuf, Vec<PathBuf>) {
    let out_dir = std::env::var("OUT_DIR").expect("no out dir");
    let base_dir = Path::new(&out_dir).join("tesseract");

    let _ = std::fs::create_dir_all(&base_dir);

    let _ = fs_extra::dir::copy(&source_dir, &base_dir, &CopyOptions::default());

    let base_dir = base_dir.join("tesseract").join("tesseract-5.3.0");

    let cmakelists = std::fs::read_to_string(base_dir.join("CMakeLists.txt"))
        .unwrap()
        .replace("set(HAVE_TIFFIO_H ON)", "");
    std::fs::write(base_dir.join("CMakeLists.txt"), cmakelists).unwrap();

    let dst = cmake::Config::new(&base_dir)
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
        .configure_arg("-DBUILD_SHARED_LIBS=OFF")
        .configure_arg("-DENABLE_LTO=ON")
        .configure_arg("-DDISABLE_ARCHIVE=ON")
        .configure_arg("-DDISABLE_CURL=ON")
        .configure_arg("-DUSE_SYSTEM_ICU=ON")
        .configure_arg("-DINSTALL_CONFIGS=ON")
        .configure_arg("-DBUILD_PROG=OFF")
        .configure_arg("-DSW_BUILD=OFF")
        .build();

    eprintln!("library path tesseract {}", dst.display());

    let library_path = dst
        .join("lib")
        .join({
            #[cfg(not(target_os = "windows"))]
            {
                "libtesseract.a"
            }
            #[cfg(target_os = "windows")]
            {
                "tesseract53.lib"
            }
        })
        .canonicalize()
        .unwrap();

    (library_path, vec![dst.join("include").join("tesseract")])
}

pub fn print_cargo_link_includes(leptonica_lib: &Path, tesseract_lib: &Path) {
    #[cfg(target_os = "macos")]
    {
        println!(
            "cargo:rustc-link-search={}",
            leptonica_lib.parent().unwrap().display()
        );
        println!(
            "cargo:rustc-link-search={}",
            tesseract_lib.parent().unwrap().display()
        );
        println!("cargo:rustc-link-lib=static=tesseract");
        println!("cargo:rustc-link-lib=static=leptonica");
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-arg={}", leptonica_lib.display());
        println!("cargo:rustc-link-arg={}", tesseract_lib.display());
    }

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-arg={}", leptonica_lib.display());
        println!("cargo:rustc-link-arg={}", tesseract_lib.display());
    }
}
