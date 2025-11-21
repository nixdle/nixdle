use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn generate_hmac(key: &str, message: &str) -> Vec<u8> {
  let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("this should NEVER fail");
  mac.update(message.as_bytes());
  mac.finalize().into_bytes().to_vec()
}

pub fn verify_hmac(key: &str, message: &str, signature: &[u8]) -> bool {
  let mut mac = HmacSha256::new_from_slice(key.as_bytes()).expect("this should NEVER fail");
  mac.update(message.as_bytes());
  mac.verify_slice(signature).is_ok()
}
