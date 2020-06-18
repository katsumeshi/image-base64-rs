extern crate reqwest;
extern crate rustc_serialize;

use reqwest::blocking;
use rustc_serialize::base64::{FromBase64, ToBase64, MIME};
use rustc_serialize::hex::ToHex;
use std::fs::File;
use std::io::Read;
use std::string::String;

pub fn get_file_type(hex: &str) -> &str {
    match &hex[..8] {
        r"ffd8ffe0" => "jpeg",
        r"89504e47" => "png",
        r"47494638" => "gif",
        _ => panic!("invalid file type"),
    }
}

pub fn to_base64(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    let base64 = vec.to_base64(MIME);
    let hex = vec.to_hex();
    return format!(
        "data:image/{};base64,{}",
        get_file_type(&hex),
        base64.replace("\r\n", "")
    );
}

pub fn from_base64(base64: String) -> Vec<u8> {
    let offset = base64.find(',').unwrap_or(base64.len()) + 1;
    let mut value = base64;
    value.drain(..offset);
    value.from_base64().unwrap()
}

pub fn from_url(image_url: &str) -> String {
    let result = reqwest::blocking::get(image_url).unwrap().bytes().unwrap();

    let base64 = result.to_base64(MIME);
    let hex = result.to_hex();

    format!(
        "data:image/{};base64,{}",
        get_file_type(&hex),
        base64.replace("\r\n", "")
    )
}
