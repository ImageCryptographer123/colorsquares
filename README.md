# ColorSquares

Library to encrypt and decrypt onto and from a image

# Usage

Add the library to your project from this git repository:

```toml
[dependencies]
colorsquares = { git = "https://github.com/ImageCryptographer123/colorsquares" }
```

Use the library as a `extern crate` in your project:

```rust
extern crate colorsquares;

fn main() {
   // Encrypt the input
    let encrypted_input = colorsquares::encrypts("DataYouWantToEncrypt");
    let watermark = "WatermarkForTheImage(bitcoin, ethereum, cardano)";

    // Generate the image
    if let Some(encoded_image) = colorsquares::create_img(&encrypted_input, watermark) {
        println!("Base64 Encoded Image:\n{}", encoded_image);
    } else {
        println!("Failed to create the image.");
    }

    match colorsquares::decrypts(&encrypted_input) {
        Some(original_text) => {
            println!("Decrypted Text: {}", original_text);
        }
        None => {
            println!("Failed to decrypt the data.");
            // You can add more detailed error messages here if needed.
        }
    }
}
```

Now, run `cargo run` to build and run the code

Output should be an output of a base64 image with the decrypted text
