extern crate base32;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ec(cleartext: &str) -> String {
  let encoded = base32::encode(cleartext.as_bytes());
  return encoded;
}
pub fn dc(cleartext: &str) -> String {
  let decoded = base32::decode(cleartext.as_bytes());
  return decoded;
}
