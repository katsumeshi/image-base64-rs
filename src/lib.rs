extern crate imghdr;

use imghdr::Type;
use std::fs::File;
use std::io::Read;
use std::string::String;
use std::option::Option;

/// Returns a file's extension based on the its hexidecimal representation.
fn get_file_extension(vec: &[u8]) -> Option<&'static str> {
   match imghdr::from_bytes(vec) {
        // Gif 87a and 89a Files
        Some(Type::Gif) => return Some("gif"),
        // TIFF files
        Some(Type::Tiff) => return Some("tiff"),
        // Sun Raster files
        Some(Type::Rast) => return Some("ras"),
        // X Bitmap files
        Some(Type::Xbm) => return Some("xbm"),
        // JPEG data in JFIF or Exif formats
        Some(Type::Jpeg) => return Some("jpeg"),
        // BMP files
        Some(Type::Bmp) => return Some("bmp"),
        // Portable Network Graphics
        Some(Type::Png) => return Some("png"),
        // WebP files
        Some(Type::Webp) => return Some("webp"),
        // OpenEXR files
        Some(Type::Exr) => return Some("exr"),
        // BGP (Better Portable Graphics) files
        Some(Type::Bgp) => return Some("bgp"),
        // PBM (Portable bitmap) files
        Some(Type::Pbm) => return Some("pbm"),
        // PGM (Portable graymap) files
        Some(Type::Pgm) => return Some("pgm"),
        // PPM (Portable pixmap) files
        Some(Type::Ppm) => return Some("ppm"),
        // SGI image library files
        Some(Type::Rgb) => return Some("rgb"),
        // HDR files (RGBE)
        Some(Type::Rgbe) => return Some("hdr"),
        // FLIF (Free Lossless Image Format) files
        Some(Type::Flif) => return Some("flif"),
        // ICO files
        Some(Type::Ico) => return Some("ico"),
        _ => return None
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
    if let Some(file_type) = get_file_extension(vec) {
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
    let encoded = base64::encode(vec);
    return format!(
        "data:image/{};base64,{}",
        extension,
        encoded.replace("\r\n", "")
    );
}

/// Converts a base64 encoded string to an image buffer.
pub fn from_base64(base64: String) -> Vec<u8> {
    let offset = base64.find(',').unwrap_or(base64.len()) + 1;
    let mut value = base64;
    value.drain(..offset);
    return base64::decode(&value).unwrap();
}
