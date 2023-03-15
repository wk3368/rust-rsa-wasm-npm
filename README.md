# 1. generate npm pkg

`wasm-pack build --scope wk3368 --release --target web`

# 2. npm publish

```
cd pkg
npm publish --access=public
```

# 3. How To Use

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

# References

- https://developer.mozilla.org/zh-CN/docs/WebAssembly/Rust_to_wasm
- https://docs.rs/rsa/0.8.2/rsa/index.html
