extern crate rsa;
extern crate rand;
extern crate hex;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey},
    PublicKey, RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt
};

#[warn(dead_code)]
const PRIVATE_KEY_PEM: &str = include_str!("pem/pkcs1/priv.pem");

#[warn(dead_code)]
const PUBLIC_KEY_PEM: &str = include_str!("pem/pkcs1/pub.pem");

#[wasm_bindgen]
pub fn encrypt(data_str: &str) -> String {
    let public_key = RsaPublicKey::from_pkcs1_pem(PUBLIC_KEY_PEM).unwrap();

    let mut rng = rand::thread_rng();

    // Encrypt
    let data = data_str.as_bytes();

    let enc_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
    let result = hex::encode(&enc_data);
    return result;
}


#[wasm_bindgen]
pub fn decrypt(data: &str) -> String {
    let private_key = RsaPrivateKey::from_pkcs1_pem(PRIVATE_KEY_PEM).unwrap();

    // hex string to Vec<u8>
    let decoded = hex::decode(data).expect("Decoding failed");

    // Decrypt
    let dec_data = private_key.decrypt(Pkcs1v15Encrypt, &decoded).expect("failed to decrypt");

    let result = String::from_utf8(dec_data.clone()).unwrap();
    return result;
}