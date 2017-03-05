#image-base64-rs

<p align="left">
    <a href="https://crates.io/crates/image-base64">
        <img src="https://img.shields.io/crates/v/image-base64.svg"
             alt="image-base64">
    </a>
</p>
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
