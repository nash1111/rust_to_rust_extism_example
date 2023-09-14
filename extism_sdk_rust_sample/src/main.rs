use extism::{Context, Plugin};
use serde_json::Value;

// code.wasm
// curl https://raw.githubusercontent.com/extism/extism/main/wasm/code.wasm > code.wasm

fn downloaded_wasm_sample() {
    let wasm = include_bytes!("code.wasm");
    let context = Context::new();

    // ignore official document
    let mut plugin = Plugin::new(&context, wasm, [], false).unwrap();
    let data = plugin.call("count_vowels", "this is a test").unwrap();
    assert_eq!(data, b"{\"count\": 4}");
    // bite to string
    let data_str = std::str::from_utf8(&data).expect("Failed to convert to string");

    // string to json
    let parsed: Value = serde_json::from_str(data_str).expect("Failed to parse JSON");

    // json to int
    if let Some(count) = parsed["count"].as_i64() {
        println!("Count: {}", count);
    } else {
        println!("Failed to retrieve 'count' value");
    }
}

fn sdk_sample() {
    let wasm = include_bytes!("../../extism_pdk_rust_sample/target/wasm32-unknown-unknown/release/extism_pdk_rust_sample.wasm");
    let context = Context::new();

    // ignore official document
    let mut plugin = Plugin::new(&context, wasm, [], false).unwrap();
    let data = plugin.call("multiply", "{\"a\": 2, \"b\": 3}").unwrap();

    // bite to string
    let data_str = std::str::from_utf8(&data).expect("Failed to convert to string");

    // string to json
    let parsed: Value = serde_json::from_str(data_str).expect("Failed to parse JSON");

    // json to int
    if let Some(count) = parsed["product"].as_i64() {
        println!("product: {}", count);
    } else {
        println!("Failed to retrieve 'product' value");
    }
}

fn main() {
    downloaded_wasm_sample();
    sdk_sample();
}
