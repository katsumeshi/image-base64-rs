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

fn get_file_type(hex: &str) -> &str {
    if Regex::new(r"^ffd8ffe0").unwrap().is_match(hex) { 
        return "jpeg" 
    } else if Regex::new(r"^89504e47").unwrap().is_match(hex) {  
        return "png" 
    } else if Regex::new(r"^47494638").unwrap().is_match(hex) { 
        return "gif"
    } 
    panic!("invalid file type")
}

 fn to_base64(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    let base64 = vec.to_base64(MIME);
    let hex = vec.to_hex();
    return format!("data:image/{};base64,{}", get_file_type(&hex), base64.replace("\r\n", ""));
}

 fn from_base64(base64: String) -> Vec<u8> {
    let offset = base64.find(',').unwrap_or(base64.len())+1;
    let mut value = base64;
    value.drain(..offset);
    return value.from_base64().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::io::Error;

    #[test]
    fn jpg_to_base64() {
        // let mut f = File::open("res/jpg_data").unwrap();
        let mut buffer = String::new();
    let mut file = match File::open("res/jpg_data") {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}", why),
        Ok(_) => print!("{} contains:\n", s),
    }

    println!("aaaaaa{}", s);
        // println!("{:?}", buffer);

        let base64 = to_base64("res/nyan.jpg"); 
        assert_eq!(base64, s);
       
        
        // let img = from_base64(base64);
        // let mut file = File::create(&Path::new("output.jpg")).unwrap();
        // file.write_all(img.as_slice()).unwrap();
    }

    // #[test]
    // fn png_to_base64() {
    //     let base64 = to_base64("res/nyan.png");
    //     // let img = from_base64(base64);
    //     // let mut file = File::create(&Path::new("output.jpg")).unwrap();
    //     // file.write_all(img.as_slice()).unwrap();
    // }
    // #[test]
    // fn gif_to_base64() {
    //     let base64 = to_base64("res/nyan.gif");
    //     // let img = from_base64(base64);
    //     // let mut file = File::create(&Path::new("output.jpg")).unwrap();
    //     // file.write_all(img.as_slice()).unwrap();
    // }
}