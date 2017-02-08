extern crate rustc_serialize;
extern crate regex;

// use std::path::Path;

// use std::io::Write;
use std::str;
// use std::vec::Drain;

mod image_base64 {
    use std::fs::File;
    use rustc_serialize::base64::{ToBase64, MIME};
    use rustc_serialize::hex::{ToHex};
    use regex::Regex;
    use std::io::Read;
    use std::string::String;

    pub fn to_base64(path: &str) -> String {
        let mut file = File::open(path).unwrap();
        let mut vec = Vec::new();
        let _ = file.read_to_end(&mut vec);
        let base64 = vec.to_base64(MIME);
        let hex = vec.to_hex();
        return format!("data:image/{};base64,{}", get_file_type(&hex), base64);
    }

    fn get_file_type(hex: &str) -> &str {
        if Regex::new(r"^ffd8ffe0").unwrap().is_match(hex) { "jpg" }
        else if Regex::new(r"^89504e47").unwrap().is_match(hex){ "png" }
        else if Regex::new(r"^47494638").unwrap().is_match(hex){ "gif" }
        else { panic!("invalid file") }
    }
}

fn main() {

    // ------------------------------------------------------

    let base64 = image_base64::to_base64("res/test.jpg");
    let offset = base64.find(',').unwrap_or(base64.len())+1;
    let mut header = base64; // todo: base64 should be immutable
    let value:String = header.drain(..offset).collect();

    //  println!("{}", base64);
    //  println!("--------------------------------------");
     println!("{}", header);
     println!("--------------------------------------");
     println!("{}", value);

    
    // let a = fmt_base64.find("data:image/");
    // println!("{}", None < a);
    
    // let a = fmt_base64.find("data:image/");
    // println!("{}", None < a);

	// let img = b64.from_base64().unwrap();
    // let mut file = File::create(&Path::new("test.jpg")).unwrap();
    // file.write_all(img.as_slice());
}