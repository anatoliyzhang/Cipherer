
use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce}; // Or `Aes128GcmSiv`
use aes_gcm_siv::aead::{Aead, NewAead};

fn main(){

let key= Key::from_slice("an example very very secret key.".as_bytes());
let cipher = Aes256GcmSiv::new(key);
let plaintext = "mmmmmm";
let nonce = Nonce::from_slice("unique nonce".as_bytes()); // 96-bits; unique per message

let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes().as_ref())
    .expect("encryption failure!");  // NOTE: handle this error to avoid panics!
// let mut cipher_vec = Vec::new();
// cipher_vec = ciphertext.to_vec();
// println!("{:?}", encode(ciphertext));
// println!("{:?}", ciphertext.iter().map(|&a| a as char).collect::<Vec<_>>());
// println!("{:?}",ciphertext.iter().map(|&b| {
//     char::from(b)
// }).collect());
// let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
    // .expect("decryption failure!");  // NOTE: handle this error to avoid panics!

// assert_eq!(&plaintext, b"plaintext message");
// String::from("s: &str")
}