use hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

/**
 * 将Option<String>转换为i32
 */
pub fn option_to_number(x: Option<&String>, default_value: i32) -> i32 {
    match x {
        Some(n) => n.parse::<i32>().unwrap_or(default_value),
        None => default_value,
    }
}

/**
 * 将Option<&String>转换为String
 */
pub fn option_to_string(x: Option<&String>) -> String {
    match x {
        Some(n) => n.to_string(),
        None => "".to_string(),
    }
}

/**
 * 签名算法
 */
pub fn hmac_sha256_hex(message: &[u8], key: &[u8]) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(message);
    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    let code_slice = code_bytes.as_slice();
    hex::encode(code_slice)
}
