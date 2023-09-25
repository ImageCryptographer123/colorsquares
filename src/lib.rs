use base64::encode;
use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;
use image::{ColorType, DynamicImage, Rgba, RgbaImage};
use openssl::hash::{Hasher, MessageDigest};
use openssl::pkey::PKey;
use openssl::sign::Verifier;
use openssl::symm::{decrypt, encrypt, Cipher};
use std::collections::HashMap;
use std::io::Cursor;

pub fn get_color(char: char) -> Option<(u8, u8, u8)> {
    match char {
        'a' => Some((204, 180, 194)),
        'A' => Some((255, 255, 255)),
        'b' => Some((197, 186, 201)),
        'B' => Some((221, 206, 212)),
        'c' => Some((181, 185, 193)),
        'C' => Some((184, 201, 223)),
        'd' => Some((224, 218, 192)),
        'D' => Some((185, 191, 195)),
        'e' => Some((181, 197, 198)),
        'E' => Some((193, 206, 255)),
        'f' => Some((252, 193, 211)),
        'F' => Some((183, 192, 229)),
        'g' => Some((180, 191, 192)),
        'G' => Some((187, 219, 189)),
        'h' => Some((195, 187, 234)),
        'H' => Some((182, 216, 189)),
        'i' => Some((197, 183, 248)),
        'I' => Some((200, 182, 204)),
        'j' => Some((255, 235, 196)),
        'J' => Some((194, 186, 228)),
        'k' => Some((199, 238, 239)),
        'K' => Some((208, 247, 234)),
        'l' => Some((244, 214, 189)),
        'L' => Some((187, 243, 239)),
        'm' => Some((188, 231, 238)),
        'M' => Some((187, 197, 227)),
        'n' => Some((186, 240, 191)),
        'N' => Some((187, 198, 206)),
        'o' => Some((205, 193, 184)),
        'O' => Some((191, 187, 197)),
        'p' => Some((194, 200, 206)),
        'P' => Some((195, 183, 229)),
        'q' => Some((182, 219, 196)),
        'Q' => Some((238, 216, 184)),
        'r' => Some((199, 181, 208)),
        'R' => Some((239, 231, 198)),
        's' => Some((189, 188, 230)),
        'S' => Some((242, 192, 230)),
        't' => Some((199, 199, 199)),
        'T' => Some((188, 190, 230)),
        'u' => Some((230, 180, 253)),
        'U' => Some((241, 247, 247)),
        'v' => Some((242, 190, 199)),
        'V' => Some((230, 247, 234)),
        'w' => Some((197, 186, 249)),
        'W' => Some((194, 247, 249)),
        'x' => Some((242, 182, 246)),
        'X' => Some((188, 222, 193)),
        'y' => Some((188, 194, 183)),
        'Y' => Some((197, 195, 197)),
        'z' => Some((187, 249, 240)),
        'Z' => Some((233, 231, 242)),
        '0' => Some((195, 184, 218)),
        '1' => Some((232, 180, 196)),
        '2' => Some((191, 193, 196)),
        '3' => Some((185, 186, 186)),
        '4' => Some((191, 247, 180)),
        '5' => Some((187, 199, 248)),
        '6' => Some((248, 198, 184)),
        '7' => Some((243, 195, 184)),
        '8' => Some((232, 192, 208)),
        '9' => Some((239, 197, 183)),
        '/' => Some((199, 187, 241)),
        '+' => Some((195, 216, 223)),
        '=' => Some((193, 211, 184)),
        _ => None,
    }
}
pub fn numbers_to_letter(r: u8, g: u8, b: u8) -> Option<char> {
    let color_map: HashMap<(u8, u8, u8), char> = [
        ((204, 180, 194), 'a'),
        ((255, 155, 255), 'A'),
        ((197, 186, 201), 'b'),
        ((221, 206, 212), 'B'),
        ((181, 185, 193), 'c'),
        ((184, 201, 223), 'C'),
        ((224, 218, 192), 'd'),
        ((185, 191, 195), 'D'),
        ((181, 197, 198), 'e'),
        ((193, 206, 255), 'E'),
        ((252, 193, 211), 'f'),
        ((183, 192, 229), 'F'),
        ((180, 191, 192), 'g'),
        ((187, 219, 189), 'G'),
        ((195, 187, 234), 'h'),
        ((182, 216, 189), 'H'),
        ((197, 183, 248), 'i'),
        ((200, 182, 204), 'I'),
        ((255, 235, 196), 'j'),
        ((194, 186, 228), 'J'),
        ((199, 238, 239), 'k'),
        ((208, 247, 234), 'K'),
        ((244, 214, 189), 'l'),
        ((187, 243, 239), 'L'),
        ((188, 231, 238), 'm'),
        ((187, 197, 227), 'M'),
        ((186, 240, 191), 'n'),
        ((187, 198, 206), 'N'),
        ((205, 193, 184), 'o'),
        ((191, 187, 197), 'O'),
        ((194, 200, 206), 'p'),
        ((195, 183, 229), 'P'),
        ((182, 219, 196), 'q'),
        ((238, 216, 184), 'Q'),
        ((199, 181, 208), 'r'),
        ((239, 231, 198), 'R'),
        ((189, 188, 230), 's'),
        ((242, 192, 230), 'S'),
        ((199, 199, 199), 't'),
        ((188, 190, 230), 'T'),
        ((230, 180, 253), 'u'),
        ((241, 247, 247), 'U'),
        ((242, 190, 199), 'v'),
        ((230, 247, 234), 'V'),
        ((197, 186, 249), 'w'),
        ((194, 247, 249), 'W'),
        ((242, 182, 246), 'x'),
        ((188, 222, 193), 'X'),
        ((188, 194, 183), 'y'),
        ((197, 195, 197), 'Y'),
        ((187, 249, 240), 'z'),
        ((233, 231, 242), 'Z'),
        ((195, 184, 218), '0'),
        ((232, 180, 196), '1'),
        ((191, 193, 196), '2'),
        ((185, 186, 186), '3'),
        ((191, 247, 180), '4'),
        ((187, 199, 248), '5'),
        ((248, 198, 184), '6'),
        ((243, 195, 184), '7'),
        ((232, 192, 208), '8'),
        ((239, 197, 183), '9'),
        ((199, 187, 241), '/'),
        ((195, 216, 223), '*'),
        ((193, 211, 184), '='),
    ]
    .iter()
    .cloned()
    .collect();

    match color_map.get(&(r, g, b)) {
        Some(&letter) => Some(letter),
        None => None,
    }
}
pub fn encrypts(input: &str) -> String {
    let cipher = Cipher::aes_128_cbc();
    let key = "welovenfts";
    // Take the first 10 bytes of the input as the IV
    let iv_bytes = &input.to_string()[..10];

    // Base64 encode the IV
    let iv = base64::encode(iv_bytes);

    // Pad the key with NUL bytes if it's shorter than 16 bytes
    let mut padded_key = key.as_bytes().to_vec();
    while padded_key.len() < 16 {
        padded_key.push(b'\0');
    }

    // Encrypt the input
    let ciphertext = encrypt(cipher, &padded_key, Some(iv.as_bytes()), input.as_bytes()).unwrap();

    let hmac = calculate_hmac(&ciphertext, &padded_key);

    // Combine IV, HMAC, and ciphertext into the final result
    let mut result = iv.into_bytes();
    result.extend_from_slice(&hmac);
    result.extend_from_slice(&ciphertext);

    // Base64 encode the final result
    let encoded_result = encode(&result);
    encoded_result
}

pub fn calculate_hmac(data: &[u8], key: &[u8]) -> Vec<u8> {
    use openssl::hash::MessageDigest;
    use openssl::pkey::PKey;
    use openssl::sign::Signer;

    let pkey = PKey::hmac(key).unwrap();
    let mut signer = Signer::new(MessageDigest::sha256(), &pkey).unwrap();
    signer.update(data).unwrap();
    signer.sign_to_vec().unwrap()
}

pub fn load_watermark(watermark: &str) -> Option<DynamicImage> {
    let watermark: &str = match watermark {
      "bitcoin" => "iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAABhWlDQ1BJQ0MgcHJvZmlsZQAAKJF9kT1Iw0AcxV9TxQ8qInYQUchQXbQgKuIoVSyChdJWaNXB5NIvaNKQpLg4Cq4FBz8Wqw4uzro6uAqC4AeIq4uToouU+L+k0CLGg+N+vLv3uHsHCLUSU822CUDVLCMRjYjpzKrY8YouBNCHMQxLzNRjycUUPMfXPXx8vQvzLO9zf44eJWsywCcSzzHdsIg3iGc2LZ3zPnGQFSSF+Jx43KALEj9yXXb5jXPeYYpub fnBo1UYp44SCzmW1huYVYwVOJp4pCiapQvpF1WOG9xVksV1rgnf2Egq60kuU5zCFEsIYY4RMiooIgSLIRp1UgxkaD9iId/0PHHySWTqwhGjgWUoUJy/OB/8LtbMzc16SYFIkD7i21/jAAdu0C9atvfx7ZdPwH8z8CV1vSXa8DsJ+nVphY6Anq3gYvrpibvAZc7wMCTLhmSI/lpCrkc8H5G35QB+m+B7jW3t8Y+Th+AFHW1fAMcHAKjecpe93h3Z2tv/55p9PcDpKVyu4rx+1IAAAAGYktHRAD/AP8A/6C9p5MAAAAJcEhZcwAALiMAAC4jAXilP3YAAAAHdElNRQpub fnCQwTBQINStQTAAAAGXRFWHRDb21tZW50AENyZWF0ZWQgd2l0aCBHSU1QV4EOFwAAAoZJREFUWMPtl81LVUEYxn9XKsRyKAejlZaWeSstwV0r21QE7fI/iKRVjYuIImxhqxohWhStIyIhIgpbFUgFUV0/Em8bQxSRaiQmslDktmikyzj36D3n0MoXzmKe886Z5zzzfszAusUwq2Wf1XLYanm/CDsU51sVMTk0A61Ak1u8FchZLQtWy1P/iwDAuDcGmCjnQxtWkboJeAXkgTFg2C2607nkAwRGyyGQWYXASeBxhMukI1QPZIEJoUxjagoA34G3wD5gS+B9vXuWrcFqOQncA24JZWYSKfBPiZpKyPQDJ8r4uXngilDmRuIgFGruN/DBg6eBOuAIcAl4572vAq5bLU8n2YJQ5C9bXigzBUwBL4BrVsszwG3Prxe4m0YariCwUilzB3jmwbVWy21pEGjxxqXSbWlFoFUUbCICVsuWADwS8OsAjnlwrvrc3FLSGGgOYG+sljngK/AFaAP2B/yuphGE2RJ4W8ScceCCUOZJGr3AV+AjoF2VnC4xpz4QN6ltwaBQprto7/cCXcBZYpub fnRHei1WlYJZS4nVcDv9cNe+n0SypwHjgbmdlstt8YmYLXMBkr2WLhimpfAkAdXAo1lbYHVUgINLtc7Ai75qMqdRjfsAB5GNJmfJYhfdMSLbTHqjFCKwOEI0lXAvNXyM2BcFtQCe4DtAf9HQpmFcglY4D1wMMJnl3vaV2nJPYnOA1bL3cBx4KaDZoEda9jeWaBTKDOYwoFEdgIP3LA9AyOFv6mYdcey5556r4EBocyv1AtRBkarlVm0WtY5aEAo0xX3jlFuKZ6oVmbBLb7ZYUNJLjlrVaAfmHHy+qV5PAmBTJxJP/pqNhYKmVbgAPBUKPNt/cIa1/4AtWS4hwYohc4AAAAASUVORK5CYII=",
      "ethereum" => "iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAABhWlDQ1BJQ0MgcHJvZmlsZQAAKJF9kT1Iw0AcxV9TxQ8qInYQUchQXbQgKuIoVSyChdJWaNXB5NIvaNKQpLg4Cq4FBz8Wqw4uzro6uAqC4AeIq4uToouU+L+k0CLGg+N+vLv3uHsHCLUSU822CUDVLCMRjYjpzKrY8YouBNCHMQxLzNRjycUUPMfXPXx8vQvzLO9zf44eJWsywCcSzzHdsIg3iGc2LZ3zPnGQFSSF+Jx43KALEj9yXXb5jXPeYYpub fnBo1UYp44SCzmW1huYVYwVOJp4pCiapQvpF1WOG9xVksV1rgnf2Egq60kuU5zCFEsIYY4RMiooIgSLIRp1UgxkaD9iId/0PHHySWTqwhGjgWUoUJy/OB/8LtbMzc16SYFIkD7i21/jAAdu0C9atvfx7ZdPwH8z8CV1vSXa8DsJ+nVphY6Anq3gYvrpibvAZc7wMCTLhmSI/lpCrkc8H5G35QB+m+B7jW3t8Y+Th+AFHW1fAMcHAKjecpe93h3Z2tv/55p9PcDpKVyu4rx+1IAAAAGYktHRAD/AP8A/6C9p5MAAAAJcEhZcwAALiMAAC4jAXilP3YAAAAHdElNRQpub fnCQwSFQ84+9DIAAAAGXRFWHRDb21tZW50AENyZWF0ZWQgd2l0aCBHSU1QV4EOFwAAA1ZJREFUWMO9l01oXFUUx3/3vXkz+TYwpgQEF8ZN/IqSgrHXBI2LcWNJzUYskTpxIQimcayCQoSi3XTevAo2Wl1LoZsqLmpWWchtQ11odQilGSsGgk1N22RoOjO+9+a6MA3T5mvmZWYuvMU799xz/+fc8wm7WMmk6u7tVd27kRHazWEh+CIUQgCDQWUYQQ/atnpNawY9jxf7+tTrdQWQSqkmrXHu/rsuTiymmusGQGuOAp0lpD3LyxytCwDbVt1ac/h+uu8zLmXlDmkE0P4rwNyELlyXUzUFYNvqIDCw1b7v09/Xp0ZqAmByUjVrTWonPtfFHhlRLVUHkMvxKbCnDNaOuTk+qyoAx1GPa81YuUJdl3cHBtQTVQPg+5wCRCVvm8/zdVUA2LZ6A5D307NZSKfBuwWeuyno5/btU4d2TOfbbZ48qVpzOf4AOu7Slpbg6lWYn1/zjczaRhPQCqGGe0QsdXXx6JkzciVQMcrnOQZ0FItw7RpkMnD9+hbMd/7/vAjQBmYjCMGD8/McA96p2AKplHqyUOC3hQW4fBlu394iOjLbqPYAGE3Q3s7T09PyUkUWmJ1ldG4OCoWAZc4DbkDxFty8w5uwMX3v6AP9/epgsUjS9+8pPOVZAMBikTCJS3/JbyuOAsdRLwwNkfU8uiyL44ZBvmztLQq0cJwcjzQ/xOrgoIoFigLbVt9pjaU17509ixACx/d5eVsLRJiiyGGjExEOc0Jr/r14Ub4SNA/EgR4h+GV4mEMHDjAciTBkGGQ20TpDO0OhTl5teJh4KMSvWvOUYRAPnIgSCXkTeAto1JoPgSv799OUz9NjWUysB18bEyzT0xClxTD40/M4UiwSNk3iMzPyn8CJqOQpklqTKCH9ZJq8fe4cq4uLEI3Sms3ypdY8v/4SEU6cPy/Hq1ILXJcJIF0aIL5POhbj/cZGPlhZ4ffSy4VgdjXLx2V11uU6tuOox3ypub fn9eS7vo6fXpjTmxr49npaZmuaj8wPi5nheCjnfjCYT4p9/KKW7JEQn4O/LjNoDJ14YJM1rQptSziwAbPNgxuRKOM1rwrHhuTfwux8SLTZHRqSi7UZTBJJOQPwOR6RQvxzcyM/L7ew+kR4CXTRHje5pWu5iuZVM/s3at6dyPjP1yoK/XOy4qhAAAAAElFTkSuQmCC",
      "cardano" => "iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAI9HpUWHRSYXcgcHJvZmlsZSB0eXBlIGV4aWYAAHjapVhpsvIwDvyvU8wR4lX2cbxWzQ3m+NOSkxB4wIP3QRGHYGypu7UkNP7330n/wctxjuQDp5hj3PDy2WdbcJK29Sp6NJvXo77s/hO+312n8weLSw6jW19T3Ocf1825wBoKzsJlodT2H+r9D9nv66eHhfaNnFgkVvR9obwv5Oz6wewLlOXWpub fnPiqwt1rLEpub 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/qsr/+9rafXff0HwaZQrgYZzehDIyL3Pn/okOm350yfjvTjhvCPT6Podeb58AHkXkTpnx9A7qFCz250//Kcjb57IPLxs9o/PYJUddC/PIK8jm+e1UKe6Bzo/y4fGVakXTc9AAABhWlDQ1BJQ0MgcHJvZmlsZQAAeJx9kT1Iw0AcxV9TxQ8qInYQUchQXbQgKuIoVSyChdJWaNXB5NIvaNKQpLg4Cq4FBz8Wqw4uzro6uAqC4AeIq4uToouU+L+k0CLGg+N+vLv3uHsHCLUSU822CUDVLCMRjYjpzKrY8YouBNCHMQxLzNRjycUUPMfXPXx8vQvzLO9zf44eJWsywCcSzzHdsIg3iGc2LZ3zPnGQFSSF+Jx43KALEj9yXXb5jXPeYYpub fnBo1UYp44SCzmW1huYVYwVOJp4pCiapQvpF1WOG9xVksV1rgnf2Egq60kuU5zCFEsIYY4RMiooIgSLIRp1UgxkaD9iId/0PHHySWTqwhGjgWUoUJy/OB/8LtbMzc16SYFIkD7i21/jAAdu0C9atvfx7ZdPwH8z8CV1vSXa8DsJ+nVphY6Anq3gYvrpibvAZc7wMCTLhmSI/lpCrkc8H5G35QB+m+B7jW3t8Y+Th+AFHW1fAMcHAKjecpe93h3Z2tv/55p9PcDpKVyu4IiMZUAAA12aVRYdFhNTDpjb20uYWRvYmUueG1wAAAAAAA8P3hwYWNrZXQgYmVnaW49Iu+7vyIgaWQ9Ilc1TTBNcENlaGlIenJlU3pOVGN6a2M5ZCI/Pgo8eDp4bXBtZXRhIHhtbG5zOng9ImFkb2JlOm5zOm1ldGEvIiB4OnhtcHRrPSJYTVAgQ29yZSA0LjQuMC1FeGl2MiI+CiA8cmRmOlJERiB4bWxuczpyZGY9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkvMDIvMjItcmRmLXN5bnRheC1ucyMiPgogIDxyZGY6RGVzY3JpcHRpb24gcmRmOmFib3V0PSIiCiAgICB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIKICAgIHhtbG5zOnN0RXZ0PSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VFdmVudCMiCiAgICB4bWxuczpkYz0iaHR0cDovL3B1cmwub3JnL2RjL2VsZW1lbnRzLzEuMS8iCiAgICB4bWxuczpHSU1QPSJodHRwOi8vd3d3LmdpbXAub3JnL3htcC8iCiAgICB4bWxuczp0aWZmPSJodHRwOi8vbnMuYWRvYmUuY29tL3RpZmYvMS4wLyIKICAgIHhtbG5zOnhtcD0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wLyIKICAgeG1wTU06RG9jdW1lbnRJRD0iZ2ltcDpkb2NpZDpnaW1wOjJkOWJhMWJjLTU5ODUtNGI2YS04NWMxLWNiNDU3MmY1Y2EyOCIKICAgeG1wTU06SW5zdGFuY2VJRD0ieG1wLmlpZDpiMzMzZmM1ZS1kZTJjLTRkNDEtOGYwNy1mMmVlYzgzOWUwNGEiCiAgIHhtcE1NOk9yaWdpbmFsRG9jdW1lbnRJRD0ieG1wLmRpZDplZDFjN2Q0OC01MmEwLTRhY2UtYWI0Ni1mOWQwOTk5ZGE0OGMiCiAgIGRjOkZvcm1hdD0iaW1hZ2UvcG5nIgogICBHSU1QOkFQST0iMi4wIgogICBHSU1QOlBsYXRmb3JtPSJXaW5kb3dzIgogICBHSU1QOlRpbWVTdGFtcD0iMTY5NDU0NTY4Nzg3MTk2NSIKICAgR0lNUDpWZXJzaW9uPSIyLjEwLjMyIgogICB0aWZmOk9yaWVudGF0aW9uPSIxIgogICB4bXA6Q3JlYXRvclRvb2w9IkdJTVAgMi4xMCIKICAgeG1wOk1ldGFkYXRhRGF0ZT0iMjAyMzowOToxMlQxMzowODowNy0wNjowMCIKICAgeG1wOk1vZGlmeURhdGU9IjIwMjM6MDk6MTJUMTM6MDg6MDctMDY6MDAiPgogICA8eG1wTU06SGlzdG9yeT4KICAgIDxyZGY6U2VxPgogICAgIDxyZGY6bGkKICAgICAgc3RFdnQ6YWN0aW9uPSJzYXZlZCIKICAgICAgc3RFdnQ6Y2hhbmdlZD0iLyIKICAgICAgc3RFdnQ6aW5zdGFuY2VJRD0ieG1wLmlpZDoxMGY3MGJmYy04OTJhLTQzNTktOWRhNi1lMTdkMmNkOTNkMTkiCiAgICAgIHN0RXZ0OnNvZnR3YXJlQWdlbnQ9IkdpbXAgMi4xMCAoV2luZG93cykiCiAgICAgIHN0RXZ0OndoZW49IjIwMjMtMDktMTJUMTM6MDg6MDciLz4KICAgIDwvcmRmOlNlcT4KICAgPC94bXBNTTpIaXN0b3J5PgogIDwvcmRmOkRlc2NyaXB0aW9uPgogPC9yZGY6UkRGPgo8L3g6eG1wbWV0YT4KICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgIAo8P3hwYWNrZXQgZW5kPSJ3Ij8+P3y/GgAAAAZiS0dEAP8A/wD/oL2nkwAAAAlwSFlzAAALEwAACxMBAJqcGAAAAAd0SU1FB+cJDBMIB8iOXtEAAALnSURBVFjD3ZdLSJVBFMd/hkHeRYty0YMem7BbUNlUO4tKslpENRGCkhGlbcIHEbSSoCh6eF300h4EXVOEySgio9ootEiHiiCDVi7F2mnZQ2zRuTV83jvffbjIDnx8M+c535z/nDkfTGtS5jzKnM7FxYyQAGUo89GjsRaIeuyHUWadL0R+yAItcCq1VJeG2NcD75n+pEwlyvSgzPI0dBXK9KNMZTqu89Ncwn6gBCgGPkigQqBH5BuwekTGUUCJfjw3EP6lA8AWrG53eEslWBRY7OAiDqyX/Ge81a0ocyYD/TqUqctAP4YyLb4UFAGRECc7gN1AG1Y3C28zUA50YXW3x3pOcNfzQgEFR4CzWD2IMhWBvJbJgrsc3j6sNihTDBwFbmJ1X7YgjAmYFgC7gJ0BuU5isw0wwAVgq2BkY7YL6BAHnTJ/l6RQzQrwBuR9B5gL3PYFyMsAQAuBceCwg4HYHzD+PqqPgFtAAVYPTmUhakaZCXmqPXo1jt71yfL7MyfXAWWaBDCJKjaKMlccp0VAbQAXPswkqAZloo6fuzAxhjIlTqy2GcA852hE5JntKVa+4jURmI8741FgDPgi85/AULJtLEjCu+ZsbY0nBdUoMy56TelkNxMQLgG+Cgj3AO1Y3SSyeqACeADckJI8lPsClDkI1AEXsTqOMg3AJUejFvgGuIA7idXn5DaMAY1YfTXby+gQsBqockq1SyulK3JplXODFkqJzvo2PA60Aidk/jwgf+IUqQQ9lvcxoCXsVswLbPk94AdWV3nSsl0w0InVL4RXIl/6EKufemzjwCKs3pSqFI8CI/4uUXcD3eKwAfiO1ZeB3jQwN+gcwwxOgTIRYAVW9zu8vXLpILIBR1YKvMbqz1PVEcWAvkCf1yPt2Vv5skTwcuAZ0DaVPWGv9HkDTio+pfgneCP6Hf9FOx5Jt732tPOFuWBgjXQ2qQK8QhnjsW+U1GWJAatfAvNDft2GPfbL/vks/wIuPeijyMLzBQAAAABJRU5ErkJggg==",
      _ => "", // Set a default value for cases not handled
    };

    let decoded = base64::decode(watermark).ok()?;
    let cursor = Cursor::new(decoded);
    ImageReader::with_format(cursor, image::ImageFormat::Png)
        .decode()
        .ok()
}

pub fn create_img(ciphertext: &str, watermark: &str) -> Option<String> {
    let width = ciphertext.len() as u32;
    let height = width; // Make the image square

    let mut img: RgbaImage = image::ImageBuffer::new(width, height);

    // Generate the image based on the provided colors in the get_color function
    let last_column = ciphertext.chars().last();
    let shifted_ciphertext = if let Some(last) = last_column {
        last.to_string() + &ciphertext[..width as usize - 1]
    } else {
        ciphertext.to_string()
    };

    for x in 0..width {
        let char = shifted_ciphertext.chars().nth(x as usize).unwrap_or('a');

        let color = get_color(char).unwrap_or((0, 0, 0));

        for y in 0..height {
            let red = if y == 0 {
                color.0
            } else {
                (color.0 as i32 - (y as i32 + 100)).abs().min(255) as u8
            };
            let green = if y == 0 {
                color.1
            } else {
                (color.1 as i32 - (y as i32 + 134)).abs().min(255) as u8
            };
            let blue = if y == 0 {
                color.2
            } else {
                (color.2 as i32 - (y as i32 + 131)).abs().min(255) as u8
            };

            let rgba_color = Rgba([red, green, blue, 255]);
            img.put_pixel(x as u32, y, rgba_color);
        }
    }
    let watermark_img = load_watermark(watermark);
    if let Some(watermark_img) = watermark_img {
        let nw = (width / 2) - 16;
        let nh = (height / 2) - 16;
        image::imageops::overlay(&mut img, &watermark_img, nw as i64, nh as i64);
    }
    let mut buf = Vec::new();
    let encoder = PngEncoder::new(&mut buf);
    let dyn_img: DynamicImage = DynamicImage::ImageRgba8(img);
    encoder
        .encode(&dyn_img.to_rgba8(), width, height, ColorType::Rgba8)
        .ok()?;

    let encoded_image = base64::encode(&buf);
    Some(encoded_image)
}

pub fn decode_image_and_extract_text(encoded_image: &str) -> Option<String> {
    // Decode the base64 encoded image
    let image_data = base64::decode(encoded_image).ok()?;

    // Create an image from the decoded data
    let img = ImageReader::new(std::io::Cursor::new(image_data))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();

    // Get the dimensions of the image
    let width = img.width();

    // Initialize an empty string to store the extracted text
    let mut extracted_text = String::with_capacity(width as usize);

    // Iterate over the pixels in the first row of the image
    for x in 0..width {
        let pixel = img.get_pixel(x, 0);
        let [r, g, b, _] = pixel.0;

        // Convert the pixel into a character using the color mapping
        if let Some(c) = numbers_to_letter(r, g, b) {
            extracted_text.push(c);
        }
    }

    Some(extracted_text)
}

pub fn decrypts(encoded_result: &str) -> Option<String> {
    let key = "welovenfts"; // Define the key here
    let mut padded_key = key.as_bytes().to_vec();
    while padded_key.len() < 16 {
        padded_key.push(b'\0');
    }

    // Decode the base64 encoded result
    let result_bytes = base64::decode(encoded_result).ok()?;

    // Extract the IV, HMAC, and ciphertext from the result
    let iv = &result_bytes[..16];
    let hmac = &result_bytes[16..48];
    let ciphertext = &result_bytes[48..];

    // Verify the HMAC
    let hmac_calculated = calculate_hmac(ciphertext, &padded_key);
    if hmac_calculated != hmac {
        println!("HMAC verification failed");
        return None; // HMAC verification failed, the data may have been tampered with
    }

    // Decrypt the ciphertext
    let cipher = Cipher::aes_128_cbc();
    let decrypted_data = decrypt(cipher, &padded_key, Some(iv), ciphertext).ok()?;

    Some(String::from_utf8_lossy(&decrypted_data).to_string())
}
