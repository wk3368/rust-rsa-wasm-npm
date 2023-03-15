# 1. How To Use

```
import init, { encrypt, decrypt } from '@wk3368/rust-rsa-wasm-npm';

const origin_str = JSON.stringify(
  {
    name: "text",
    type: "input",
    value: 1
  }
);

init().then(() => {
  const encrypted = encrypt(origin_str);
  const decrypted = decrypt(encrypted);
  console.log({origin_str, encrypted, decrypted, "origin_object": JSON.parse(decrypted) })
});

```

# 2. How to generate priv.pem and pub.pem

```
extern crate rsa;
extern crate rand;

use rsa::{
    pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey, LineEnding},
    PublicKey, RsaPrivateKey, RsaPublicKey, Pkcs1v15Encrypt
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
```

# 3. How to Publish NPM

## 3.1 Generate npm pkg

`wasm-pack build --scope wk3368 --release --target web`

## 3.2 npm publish

```
cd pkg
npm publish --access=public
```

# 4. References

- https://developer.mozilla.org/zh-CN/docs/WebAssembly/Rust_to_wasm
- https://github.com/RustCrypto/RSA
- https://docs.rs/rsa/0.8.2/rsa/index.html
