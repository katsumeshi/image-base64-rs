extern crate rustc_serialize;
extern crate regex;
extern crate crypto;

// mod image_base64 {
use std::fs::File;
use rustc_serialize::base64::{FromBase64, ToBase64, MIME};
use rustc_serialize::hex::{ToHex};
use regex::Regex;
use std::io::Read;
use std::string::String;
use std::path::Path;
use std::io::Write;
use crypto::md5::Md5;
use crypto::digest::Digest;


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
        let base64 = to_base64(&format!("res/{}.{}", FILE_NAME, file_type)); 
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
        let img = from_base64(base64);
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
        let mut file_arr = vector_as_u8_4_array(file_vec);
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
}