#image-base64-rs

## Synopsis

Convert image to base64, and vise versa

## Code Example

```
extern crate image_base64;

fn main() {
  let base64 = "base64 String";
  let image = image_base64::from_base64(base64);
  
  let image_path = "local image file path"
  let base64 = image_base64::to_base64(image_path); 
}
```

## Installation

cargo install --git https://github.com/katsumeshi/image-base64-rs.git image-base64-rs

## License

MIT
