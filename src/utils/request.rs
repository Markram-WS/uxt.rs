use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::SigningKey;
use ed25519_dalek::Signer;
use pkcs8::DecodePrivateKey; // <— ต้องเพิ่มบรรทัดนี้
use urlencoding::encode;

use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn sign(secret: &str, payload: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(payload.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

pub fn sign_ed25519(private_key_base64: &str, payload: &str) -> String {
    let key_bytes = general_purpose::STANDARD
        .decode(private_key_base64)
        .expect("Failed to decode base64 private key");
    let signing_key = SigningKey::from_pkcs8_der(&key_bytes)
        .expect("Invalid Ed25519 PKCS#8 Key");
    let signature = signing_key.sign(payload.as_bytes());
    general_purpose::STANDARD.encode(signature.to_bytes())
}

pub fn create_signature( payload: &Vec<(&str, String)>,private_key:&str) ->  Result<String, Box<dyn std::error::Error>> {
    let query_string  = serde_urlencoded::to_string(&payload).expect("url params encode failed");
    let key_bytes = general_purpose::STANDARD.decode(private_key)?;
    let signing_key = SigningKey::from_pkcs8_der(&key_bytes)
        .map_err(|e| format!("failed to parse pkcs8 ed25519 key: {}", e))?;
    let signature = signing_key.sign(query_string.as_bytes());
    let signature_b64 = general_purpose::STANDARD.encode(signature.to_bytes());
    let signature_encoded = encode(&signature_b64).into_owned();
    //println!("{}", signature_encoded);
    Ok(signature_encoded)
}


use serde_json::Value;

pub fn create_payload_signature(
    payload: &Value, 
    private_key: &str
) -> Result<String, Box<dyn std::error::Error>> {
    // 1. แปลง JSON Value เป็น Query String (เช่น "symbol=BTC&amount=1")
    // Note: payload ต้องเป็น JSON Object เท่านั้น
    let query_string = if payload.is_object() {
        serde_urlencoded::to_string(payload)
            .map_err(|e| format!("URL params encode failed: {}", e))?
    } else {
        return Err("Payload must be a JSON Object".into());
    };

    // 2. Decode Private Key จาก Base64 (ถ้า private_key ของคุณเป็น Base64)
    let key_bytes = general_purpose::STANDARD.decode(private_key)?;

    // 3. สร้าง SigningKey (ในที่นี้ใช้ Ed25519 ตามโค้ดที่คุณให้มา)
    // Note: ตรวจสอบว่า Key เป็น PKCS8 หรือ Raw bytes
    let signing_key = SigningKey::from_pkcs8_der(&key_bytes)
        .map_err(|e| format!("Failed to parse pkcs8 ed25519 key: {}", e))?;

    // 4. ทำการ Sign ข้อมูล
    let signature = signing_key.sign(query_string.as_bytes());

    // 5. แปลง Signature เป็น Base64 และทำ URL Encode
    let signature_b64 = general_purpose::STANDARD.encode(signature.to_bytes());
    let signature_encoded = encode(&signature_b64).into_owned();

    Ok(signature_encoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_utils_create_signature() {
        let params: Vec<(&str, String)> = vec![
            ("timestamp", "1760244698897".to_string()),
        ];
        let binance_secret = "MC4CAQAwBQYDK2VwBCIEIHvC/UOrqjH8NrhgY4gJFkgyfB359eC9Mofmj/qFvuzB";
        assert_eq!(create_signature( &params,&binance_secret).unwrap(), "qhA4ffi029s6iwNqaO9ex5qqjH7fZYVyBngZldEGm1rbXeexgwK2BK1%2F8NdviRmyq9kB1vXKCBVF4pk2C7SPBA%3D%3D");
    }

}