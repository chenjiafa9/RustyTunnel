use crate::error::{Error, Result};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::rngs::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};

/// 生成 WireGuard 密钥对
pub fn generate_keypair() -> Result<(String, String)> {
    let mut rng = OsRng;
    let private_key = StaticSecret::random_from_rng(&mut rng);
    let public_key = PublicKey::from(&private_key);

    let private_key_b64 = STANDARD.encode(private_key.as_bytes());
    let public_key_b64 = STANDARD.encode(public_key.as_bytes());

    Ok((private_key_b64, public_key_b64))
}

/// 从 Base64 编码的字符串解码私钥
pub fn decode_private_key(encoded: &str) -> Result<[u8; 32]> {
    let decoded = STANDARD
        .decode(encoded)
        .map_err(|e| Error::CryptoError(format!("Failed to decode private key: {}", e)))?;

    if decoded.len() != 32 {
        return Err(Error::CryptoError(
            "Private key must be 32 bytes".to_string(),
        ));
    }

    let mut key = [0u8; 32];
    key.copy_from_slice(&decoded);
    Ok(key)
}

/// 从 Base64 编码的字符串解码公钥
pub fn decode_public_key(encoded: &str) -> Result<[u8; 32]> {
    let decoded = STANDARD
        .decode(encoded)
        .map_err(|e| Error::CryptoError(format!("Failed to decode public key: {}", e)))?;

    if decoded.len() != 32 {
        return Err(Error::CryptoError(
            "Public key must be 32 bytes".to_string(),
        ));
    }

    let mut key = [0u8; 32];
    key.copy_from_slice(&decoded);
    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_keypair() {
        let (priv_key, pub_key) = generate_keypair().unwrap();
        assert!(!priv_key.is_empty());
        assert!(!pub_key.is_empty());
        assert_ne!(priv_key, pub_key);
    }

    #[test]
    fn test_decode_keys() {
        let (priv_key, pub_key) = generate_keypair().unwrap();
        let priv_decoded = decode_private_key(&priv_key).unwrap();
        let pub_decoded = decode_public_key(&pub_key).unwrap();

        assert_eq!(priv_decoded.len(), 32);
        assert_eq!(pub_decoded.len(), 32);
    }
}
