extern crate crypto;
extern crate image_base64;

use crypto::digest::Digest;
use crypto::md5::Md5;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::MAIN_SEPARATOR;
use std::str;
use std::string::String;

static FILE_NAME: &'static str = "nyan";

#[test]
fn jpg_to_base64() {
    image_to_base64("jpg");
}

#[test]
fn gif_to_base64() {
    image_to_base64("gif");
}

#[test]
fn png_to_base64() {
    image_to_base64("png");
}

#[test]
fn webp_to_base64() {
    image_to_base64("webp");
}

#[test]
fn ico_to_base64() {
    image_to_base64("ico");
}

#[test]
fn tiff_to_base64() {
    image_to_base64("tiff");
}

#[test]
fn bmp_to_base64() {
    image_to_base64("bmp");
}

#[test]
fn exr_to_base64() {
    image_to_base64("exr");
}

#[test]
fn hdr_to_base64() {
    image_to_base64("hdr");
}

#[test]
fn flif_to_base64() {
    image_to_base64("flif");
}

#[test]
fn pbm_to_base64() {
    image_to_base64("pbm");
}

#[test]
fn pgm_to_base64() {
    image_to_base64("pgm");
}

#[test]
fn ppm_to_base64() {
    image_to_base64("ppm");
}

#[test]
fn ras_to_base64() {
    image_to_base64("ras");
}

#[test]
fn xbm_to_base64() {
    image_to_base64("xbm");
}

fn image_to_base64(extension: &str) {
    let path = format!("res{}{}_data", MAIN_SEPARATOR, extension);
    let mut file = match File::open(Path::new(&path)) {
        Err(why) => panic!("couldn't open {}: {}", &path, why),
        Ok(file) => file,
    };
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(_) => {}
    }
    let base64 =
        image_base64::to_base64(&format!("res{}{}.{}", MAIN_SEPARATOR, FILE_NAME, extension))
            .unwrap();
    assert_eq!(base64, buffer);
}

#[test]
fn base64_to_jpg() {
    base64_to_image("jpg");
    validate("jpg");
}

#[test]
fn base64_to_gif() {
    base64_to_image("gif");
    validate("gif");
}

#[test]
fn base64_to_png() {
    base64_to_image("png");
    validate("png");
}

#[test]
fn base64_to_webp() {
    base64_to_image("webp");
    validate("webp");
}

#[test]
fn base64_to_ico() {
    base64_to_image("ico");
    validate("ico");
}

#[test]
fn base64_to_tiff() {
    base64_to_image("tiff");
    validate("tiff");
}

#[test]
fn base64_to_bmp() {
    base64_to_image("bmp");
    validate("bmp");
}

#[test]
fn base64_to_exr() {
    base64_to_image("exr");
    validate("exr");
}

#[test]
fn base64_to_hdr() {
    base64_to_image("hdr");
    validate("hdr");
}

#[test]
fn base64_to_flif() {
    base64_to_image("flif");
    validate("flif");
}

#[test]
fn base64_to_pbm() {
    base64_to_image("pbm");
    validate("pbm");
}

#[test]
fn base64_to_pgm() {
    base64_to_image("pgm");
    validate("pgm");
}

#[test]
fn base64_to_ppm() {
    base64_to_image("ppm");
    validate("ppm");
}

#[test]
fn base64_to_ras() {
    base64_to_image("ras");
    validate("ras");
}

#[test]
fn base64_to_xbm() {
    base64_to_image("xbm");
    validate("xbm");
}

fn base64_to_image(extension: &str) {
    let mut original = match File::open(format!("res{}{}_data", MAIN_SEPARATOR, extension)) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };
    let mut base64 = String::new();
    match original.read_to_string(&mut base64) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(_) => {}
    }
    let img = image_base64::from_base64(base64);
    let mut output = File::create(&Path::new(&format!(
        "output{}{}.{}",
        MAIN_SEPARATOR, FILE_NAME, extension
    )))
    .unwrap();
    output.write_all(img.as_slice()).unwrap();
}

fn validate(extension: &str) {
    assert_eq!(
        get_file_size("res", extension),
        get_file_size("output", extension)
    );
    assert_eq!(get_hash("res", extension), get_hash("output", extension));
}

fn get_hash(dir: &str, extension: &str) -> String {
    let mut hasher = Md5::new();
    let mut file = match File::open(&format!(
        "{}{}{}.{}",
        dir, MAIN_SEPARATOR, FILE_NAME, extension
    )) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };
    let mut file_vec = Vec::new();
    match file.read_to_end(&mut file_vec) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(_) => {}
    }
    let file_arr = vector_as_u8_4_array(file_vec);
    hasher.input(&file_arr);
    hasher.result_str()
}

fn get_file_size(dir: &str, extension: &str) -> u64 {
    let meta = match fs::metadata(&format!(
        "{}{}{}.{}",
        dir, MAIN_SEPARATOR, FILE_NAME, extension
    )) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(meta) => meta,
    };
    meta.len()
}

fn vector_as_u8_4_array(vector: Vec<u8>) -> [u8; 4] {
    let mut arr = [0u8; 4];
    for (place, element) in arr.iter_mut().zip(vector.iter()) {
        *place = *element;
    }
    arr
}
