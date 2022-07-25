
use std::cmp::min;
use wasm_bindgen::prelude::*;

use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce}; // Or `Aes128GcmSiv`
use aes_gcm_siv::aead::{Aead, NewAead};
// use rand::{thread_rng, Rng};
// use rand::distributions::Alphanumeric;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn encrypto(plaintext: &str, raw_key: &str)->String{
    let real_key = if raw_key.len()!=32 { "0123456789abcdefghjkmnpqrstuvwxy"} else {raw_key};
    let key= Key::from_slice(real_key.as_bytes());
    let cipher = Aes256GcmSiv::new(key);
    // let plaintext = "mmmmmm";
    let nonce = Nonce::from_slice(b"/01/00/00/01"); // 96-bits; unique per message
    
    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes().as_ref())
        .expect("/a0/0f");  // NOTE: handle this error to avoid panics!
    encode(&ciphertext)
    

}

#[wasm_bindgen]
pub fn decrypto(cipheredtext: &str, raw_key: &str)->String{
    let real_key = if raw_key.len()!=32 { "0123456789abcdefghjkmnpqrstuvwxy"} else {raw_key};
    let key= Key::from_slice(real_key.as_bytes());
    let cipher = Aes256GcmSiv::new(key);
    // let plaintext = "mmmmmm";
    
    let nonce = Nonce::from_slice(b"/01/00/00/01"); // 96-bits; unique per message
    let ciphered_bytes = decode(cipheredtext).expect("");
    let plaintext = cipher.decrypt(nonce, ciphered_bytes.as_ref())
    .expect("/a0/0f");  // NOTE: handle this error to avoid panics!
    String::from_utf8(plaintext).expect("")
}

// the following code is borrowed from crate base32
const RFC4648_ALPHABET: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
fn encode(data: &[u8]) -> String {
    let mut ret = Vec::with_capacity((data.len() + 3) / 4 * 5);

    for chunk in data.chunks(5) {
        let buf = {
            let mut buf = [0u8; 5];
            for (i, &b) in chunk.iter().enumerate() {
                buf[i] = b;
            }
            buf
        };
        ret.push(RFC4648_ALPHABET[((buf[0] & 0xF8) >> 3) as usize]);
        ret.push(RFC4648_ALPHABET[(((buf[0] & 0x07) << 2) | ((buf[1] & 0xC0) >> 6)) as usize]);
        ret.push(RFC4648_ALPHABET[((buf[1] & 0x3E) >> 1) as usize]);
        ret.push(RFC4648_ALPHABET[(((buf[1] & 0x01) << 4) | ((buf[2] & 0xF0) >> 4)) as usize]);
        ret.push(RFC4648_ALPHABET[(((buf[2] & 0x0F) << 1) | (buf[3] >> 7)) as usize]);
        ret.push(RFC4648_ALPHABET[((buf[3] & 0x7C) >> 2) as usize]);
        ret.push(RFC4648_ALPHABET[(((buf[3] & 0x03) << 3) | ((buf[4] & 0xE0) >> 5)) as usize]);
        ret.push(RFC4648_ALPHABET[(buf[4] & 0x1F) as usize]);
    }

    if data.len() % 5 != 0 {
        let len = ret.len();
        let num_extra = 8 - (data.len() % 5 * 8 + 4) / 5;
        // if padding {
        //     for i in 1..num_extra + 1 {
        //         ret[len - i] = b'=';
        //     }
        // } else {
        ret.truncate(len - num_extra);
        // }
    }

    String::from_utf8(ret).unwrap()
}

const RFC4648_INV_ALPHABET: [i8; 43] = [
    -1, -1, 26, 27, 28, 29, 30, 31, -1, -1, -1, -1, -1, 0, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8,
    9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
];
fn decode(data: &str) -> Option<Vec<u8>> {
    if !data.is_ascii() {
        return None;
    }
    let data = data.as_bytes();
    let mut unpadded_data_length = data.len();
    for i in 1..min(6, data.len()) + 1 {
        if data[data.len() - i] != b'=' {
            break;
        }
        unpadded_data_length -= 1;
    }
    let output_length = unpadded_data_length * 5 / 8;
    let mut ret = Vec::with_capacity((output_length + 4) / 5 * 5);
    for chunk in data.chunks(8) {
        let buf = {
            let mut buf = [0u8; 8];
            for (i, &c) in chunk.iter().enumerate() {
                match RFC4648_INV_ALPHABET.get(c.to_ascii_uppercase().wrapping_sub(b'0') as usize) {
                    Some(&-1) | None => return None,
                    Some(&value) => buf[i] = value as u8,
                };
            }
            buf
        };
        ret.push((buf[0] << 3) | (buf[1] >> 2));
        ret.push((buf[1] << 6) | (buf[2] << 1) | (buf[3] >> 4));
        ret.push((buf[3] << 4) | (buf[4] >> 1));
        ret.push((buf[4] << 7) | (buf[5] << 2) | (buf[6] >> 3));
        ret.push((buf[6] << 5) | buf[7]);
    }
    ret.truncate(output_length);
    Some(ret)
}