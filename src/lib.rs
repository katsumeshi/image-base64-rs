extern crate regex;
extern crate rustc_serialize;

use regex::Regex;
use rustc_serialize::base64::{ToBase64, MIME};
use rustc_serialize::hex::ToHex;
use std::fs::File;
use std::io::Read;
use std::string::String;
use std::option::Option;

/// Returns a file's extension based on the its hexidecimal representation.
/// 
/// Note: TIF files will be considered as TIFF.
fn get_file_type(hex: &str) -> Option<&str> {
    if Regex::new(r"^ffd8ffe").unwrap().is_match(hex) {
        return Some("jpeg");
    } else if 
        Regex::new(r"^49492a00").unwrap().is_match(hex) || 
        Regex::new(r"^4d4d002a").unwrap().is_match(hex) {
        return Some("tiff");
    } else if Regex::new(r"^424d").unwrap().is_match(hex) {
        return Some("bmp");
    } else if Regex::new(r"^89504e47").unwrap().is_match(hex) {
        return Some("png");
    } else if Regex::new(r"^47494638").unwrap().is_match(hex) {
        return Some("gif");
    } else if Regex::new(r"^00000100").unwrap().is_match(hex) {
        return Some("ico");
    } else if Regex::new(r"^52494646").unwrap().is_match(hex) {
        return Some("webp");
    } else {
        None
    }
}

/// Converts an image file to a base64 encoded string.
pub fn to_base64(path: &str) -> Option<String> {
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    return to_base64_from_memory(&vec);
}

/// Converts an image buffer to a base64 encoded string.
pub fn to_base64_from_memory(vec: &[u8]) -> Option<String> {
    let hex = vec.to_hex();
    if let Some(file_type) = get_file_type(&hex) {
        Some(to_base64_from_memory_with_extension(vec, file_type))
    } else {
        None
    }
}

/// Converts an image file to a base64 encoded string with a specified extension.
pub fn to_base64_with_extension(path: &str, extension: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    return to_base64_from_memory_with_extension(&vec, extension);
}

/// Converts an image buffer to a base64 encoded string with a specified extension.
pub fn to_base64_from_memory_with_extension(vec: &[u8], extension: &str) -> String {
    let base64 = vec.to_base64(MIME);
    return format!(
        "data:image/{};base64,{}",
        extension,
        base64.replace("\r\n", "")
    );
}

/// Converts a base64 encoded string to an image buffer.
pub fn from_base64(base64: String) -> Vec<u8> {
    let offset = base64.find(',').unwrap_or(base64.len()) + 1;
    let mut value = base64;
    value.drain(..offset);
    return base64::decode(&value).unwrap();
}
