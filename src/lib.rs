extern crate regex;
extern crate rustc_serialize;

use regex::Regex;
use rustc_serialize::base64::{ToBase64, MIME};
use rustc_serialize::hex::ToHex;
use std::fs::File;
use std::io::Read;
use std::string::String;

pub fn get_file_type(hex: &str) -> &str {
    if Regex::new(r"^ffd8ffe").unwrap().is_match(hex) {
        return "jpeg";
    } else if Regex::new(r"^49492a00").unwrap().is_match(hex) {
        return "tif";
    } else if Regex::new(r"^4d4d002a").unwrap().is_match(hex) {
        return "tiff";
    } else if Regex::new("r^424d").unwrap().is_match(hex) {
        return "bmp";
    } else if Regex::new(r"^89504e47").unwrap().is_match(hex) {
        return "png";
    } else if Regex::new(r"^47494638").unwrap().is_match(hex) {
        return "gif";
    } else if Regex::new(r"^00000100").unwrap().is_match(hex) {
        return "ico";
    } else if Regex::new(r"^52494646").unwrap().is_match(hex) {
        return "webp";
    } else if 
        Regex::new(r"^503120").unwrap().is_match(hex) || 
        Regex::new(r"^5032").unwrap().is_match(hex)  || 
        Regex::new(r"^503320").unwrap().is_match(hex) {
        return "pbm"
    }
    panic!("Invalid file type")
}

pub fn to_base64(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    let hex = vec.to_hex();
    return to_base64_with_extension(path, get_file_type(&hex));
}

pub fn to_base64_with_extension(path: &str, extension: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    let base64 = vec.to_base64(MIME);
    return format!(
        "data:image/{};base64,{}",
        extension,
        base64.replace("\r\n", "")
    );
}

pub fn from_base64(base64: String) -> Vec<u8> {
    let offset = base64.find(',').unwrap_or(base64.len()) + 1;
    let mut value = base64;
    value.drain(..offset);
    return base64::decode(&value).unwrap();
}
