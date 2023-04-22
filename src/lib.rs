use std::fs::File;
use std::io::Read;
use std::string::String;


pub enum FileType {
    JPEG,
    PNG,
    GIF
}

impl FileType {
    fn as_str(&self) -> &'static str {
        match self {
            FileType::JPEG => "jpeg",
            FileType::PNG => "png",
            FileType::GIF => "gif"
        }
    }
}

impl From<&str> for FileType {
    fn from(item: &str) -> Self {
        match item {
            "jpeg" => FileType::JPEG,
            "png" => FileType::PNG,
            "gif" => FileType::GIF,
            _ => FileType::JPEG
        }
    }
}

pub fn to_base64(path: &str, file_type: FileType) -> String {
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    let b64 = base64::encode(&vec);
    return format!("data:image/{};base64,{}", file_type.as_str(), b64.replace("\r\n", ""));
}

pub fn from_base64(base64: String) -> Result<Vec<u8>, base64::DecodeError> {
    let offset = base64.find(',').unwrap_or(base64.len()) + 1;
    let mut value = base64;
    value.drain(..offset);
    return base64::decode(&value)
}
