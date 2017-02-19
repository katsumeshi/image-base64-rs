extern crate rustc_serialize;
extern crate regex;

// mod image_base64 {
use std::fs::File;
use rustc_serialize::base64::{FromBase64, ToBase64, MIME};
use rustc_serialize::hex::{ToHex};
use regex::Regex;
use std::io::Read;
use std::string::String;
use std::path::Path;
use std::io::Write;

pub fn to_base64(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    let base64 = vec.to_base64(MIME);
    let hex = vec.to_hex();
    return format!("data:image/{};base64,{}", get_file_type(&hex), base64);
}

pub fn from_base64(base64: String) {
    let offset = base64.find(',').unwrap_or(base64.len())+1;
    let mut value = base64; // todo: base64 should be immutable
    value.drain(..offset);
    let img = value.from_base64().unwrap();
    let mut file = File::create(&Path::new("output.jpg")).unwrap();
    file.write_all(img.as_slice()).unwrap();
}

fn get_file_type(hex: &str) -> &str {
    if Regex::new(r"^ffd8ffe0").unwrap().is_match(hex) { 
        return "jpg" 
    } else if Regex::new(r"^89504e47").unwrap().is_match(hex) {  
        return "png" 
    } else if Regex::new(r"^47494638").unwrap().is_match(hex) { 
        return "gif"
    } 
    panic!("invalid file type")
}
// }