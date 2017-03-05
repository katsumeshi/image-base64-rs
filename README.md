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

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
image-base64 = "0.1"
```

## License

MIT
