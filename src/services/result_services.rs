extern crate aes;
extern crate block_modes;

use std::string::FromUtf8Error;

use aes::Aes128;
use block_modes::{block_padding::ZeroPadding, BlockMode, Cbc};

// Define Type of AES128/CBC/NoPadding Cipher
// Note: Using ZeroPadding instead of NoPadding is for removing the redundant bytes at the end of the deciphered message
type Aes128CbcZeroPadding = Cbc<Aes128, ZeroPadding>;

// Define Default KEY And IV;
const KEY: &[u8] = b"***REMOVED***";
const IV: &[u8] = b"***REMOVED***";

// General decipher for normal AES/CBC/ZeroPadding
pub fn decipher(key: &[u8], iv: &[u8], data: &[u8]) -> Result<String, FromUtf8Error> {
    // Create buffer
    let mut buffer = Vec::from(data);
    // Initial cipher
    let cipher = Aes128CbcZeroPadding::new_from_slices(key, iv).unwrap();
    let decipher_res = cipher.decrypt(&mut buffer).unwrap();
    String::from_utf8(Vec::from(decipher_res))
}

pub fn decipher_default(data: &[u8]) -> Result<String, FromUtf8Error> {
    decipher(KEY, IV, data)
}

pub fn cipher(key: &[u8], iv: &[u8], data: &[u8]) -> &[u8] {
    // Create Buffer
    let mut buffer = Vec::from(data);
    // Initial Cipher
    let cipher = Aes128CbcZeroPadding::new_from_slices(key, iv).unwrap();
    let cipher_res = cipher.encrypt(&mut buffer).unwrap();
    cipher_res
}

pub fn cipher_default(data: &[u8]) -> &[u8] {
    cipher(KEY, IV, data)
}