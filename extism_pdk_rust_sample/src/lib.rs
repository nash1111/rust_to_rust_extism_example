use extism_pdk::*;
use serde::Serialize;

#[derive(serde::Deserialize)]
struct Add {
    a: u32,
    b: u32,
}

#[derive(serde::Serialize)]
struct Sum {
    sum: u32,
}

#[plugin_fn]
pub fn add(Json(add): Json<Add>) -> FnResult<Json<Sum>> {
    let sum = Sum { sum: add.a + add.b };
    Ok(Json(sum))
}

const VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

#[derive(Serialize)]
struct TestOutput {
    pub count: i32,
    pub config: String,
    pub a: String,
}

#[plugin_fn]
pub fn count_vowels(input: String) -> FnResult<Json<TestOutput>> {
    let mut count = 0;
    for ch in input.chars() {
        if VOWELS.contains(&ch) {
            count += 1;
        }
    }

    set_var!("a", "this is var a")?;

    let a = var::get("a")?.expect("variable 'a' set");
    let a = String::from_utf8(a).expect("string from varible value");
    let config = config::get("thing").expect("'thing' key set in config");

    let output = TestOutput { count, config, a };
    Ok(Json(output))
}

#[derive(serde::Deserialize)]
struct Multiply {
    a: u32,
    b: u32,
}

#[derive(serde::Serialize)]
struct Product {
    product: u32,
}

#[plugin_fn]
pub fn multiply(Json(mul): Json<Multiply>) -> FnResult<Json<Product>> {
    let product = Product { product: mul.a * mul.b };
    Ok(Json(product))
}
