extern crate rsa;
extern crate rand;

use rsa::{
    pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, LineEnding},
    RsaPrivateKey, RsaPublicKey
};

fn main() {
    let mut rng = rand::thread_rng();
    let bits = 2048; // or 4086
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    let private_key_pem = private_key.to_pkcs1_pem(LineEnding::LF).unwrap();
    let public_key_pem = public_key.to_pkcs1_pem(LineEnding::LF).unwrap();
    println!("Private key:\n{}", &*private_key_pem);
    println!("Public key:\n{}", &*public_key_pem);
}