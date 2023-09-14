### install extism
```
sh <(curl https://raw.githubusercontent.com/extism/cli/main/install.sh) /usr/local/bin
extism install latest
```

### build extism pdk
```
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

### call extism sdk
```
extism call ./target/wasm32-unknown-unknown/release/extism_pdk_rust_sample.wasm count_vowels --input "this is a test" --config thing=i_hope_this_flag_become_optional

extism call ./target/wasm32-unknown-unknown/release/extism_pdk_rust_sample.wasm multiply --input '{"a": 2, "b": 3}' --config thing=i_hope_this_flag_become_optional
```