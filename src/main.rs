extern crate rustc_serialize;
extern crate regex;

use std::path::Path;

use rustc_serialize::base64::{FromBase64, ToBase64, MIME};
use rustc_serialize::hex::{ToHex};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use regex::Regex;
use std::str;

fn main() {

    // ------------------------------------------------------
	let mut file = File::open("res/test.jpg").unwrap();
    let mut v = Vec::new();
    let _ = file.read_to_end(&mut v);

    let b64 = v.to_base64(MIME);
    let hex = v.to_hex();
    let fmt_base64 = format!("data:image/{};base64,{}", get_type(&hex), b64);
    let offset = fmt_base64.find(',').unwrap_or(fmt_base64.len())+1;
    let mut header = fmt_base64;
    let value: String = header.drain(..offset).collect();
    // let ggg = fmt_base64.drain(..header).collect();

     println!("{}", header);
     println!("{}", value);

    
    // let a = fmt_base64.find("data:image/");
    // println!("{}", None < a);
    
    // let a = fmt_base64.find("data:image/");
    // println!("{}", None < a);

	// let img = b64.from_base64().unwrap();
    // let mut file = File::create(&Path::new("test.jpg")).unwrap();
    // file.write_all(img.as_slice());
}

fn get_type(file: &str) -> &str {
    if Regex::new(r"^ffd8ffe0").unwrap().is_match(file) { "jpg" }
    else if Regex::new(r"^89504e47").unwrap().is_match(file){ "png" }
    else if Regex::new(r"^47494638").unwrap().is_match(file){ "gif" }
    else { panic!("invalid file") }
}