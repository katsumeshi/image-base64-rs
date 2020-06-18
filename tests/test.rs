extern crate rustc_serialize;
extern crate crypto;
extern crate image_base64;

use std::fs::File;
use std::io::Read;
use std::string::String;
use std::path::Path;
use std::io::Write;
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::fs;
use std::str;

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

fn image_to_base64(file_type : &str) {
    let mut file = match File::open(format!("res/{}_data", file_type)) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(_) => {},
    }
    let base64 = image_base64::to_base64(&format!("res/{}.{}", FILE_NAME, file_type)); 
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

fn base64_to_image(file_type : &str) {
    let mut original = match File::open(format!("res/{}_data", file_type)) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };
    let mut base64 = String::new();
    match original.read_to_string(&mut base64) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(_) => {},
    }
    let img = image_base64::from_base64(base64);
    let mut output = File::create(&Path::new(&format!("output/{}.{}", FILE_NAME, file_type))).unwrap();
    output.write_all(img.as_slice()).unwrap();
}

fn validate(file_type : &str) {
    assert_eq!(get_file_size("res", file_type), get_file_size("output", file_type));
    assert_eq!(get_hash("res", file_type), get_hash("output", file_type));
}

fn get_hash(dir : &str, file_type : &str) -> String {
    let mut hasher = Md5::new();
    let mut file = match File::open(&format!("{}/{}.{}", dir, FILE_NAME, file_type)) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };
    let mut file_vec = Vec::new();
    match file.read_to_end(&mut file_vec) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(_) => {},
    }
    let file_arr = vector_as_u8_4_array(file_vec);
    hasher.input(&file_arr);
    hasher.result_str()
}

fn get_file_size(dir : &str, file_type : &str) -> u64 {
    let meta = match fs::metadata(&format!("{}/{}.{}",dir, FILE_NAME, file_type)) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(meta) => meta,
    };
    meta.len()
}

fn vector_as_u8_4_array(vector: Vec<u8>) -> [u8;4] {
    let mut arr = [0u8;4];
    for (place, element) in arr.iter_mut().zip(vector.iter()) {
        *place = *element;
    }
    arr
}

#[test]
fn png_url_to_base64() {
    url_to_base64("png");
}

#[test]
fn jpg_url_to_base64() {
    url_to_base64("jpg");
}

#[test]
fn gif_url_to_base64() {
    url_to_base64("gif");
}

fn url_to_base64(file_type: &str) {
    let result = image_base64::from_url(
        format!(
            "{}{}",
            "https://raw.githubusercontent.com/katsumeshi/image-base64-rs/master/res/nyan.",
            file_type
        )
        .as_ref(),
    );

    let mut file = match File::open(format!("res/{}_data", file_type)) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(_) => {}
    }

    assert_eq!(result, buffer);
}
