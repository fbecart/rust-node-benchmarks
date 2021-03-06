extern crate crypto;
extern crate toml;

use wasm_bindgen::prelude::*;
use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;

#[wasm_bindgen]
pub fn sum(a: usize, b: usize) -> usize {
    a + b
}

#[wasm_bindgen]
pub fn sha1(data: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(data);
    hasher.result_str()
}

#[wasm_bindgen]
pub fn fibonacci(n: usize) -> i32 {
    let (mut a, mut b) = (1, 1);
    for _i in 0..n {
        let sum = a + b;
        a = b;
        b = sum;
    }
    b
}

#[wasm_bindgen(js_name = "readFieldFromToml")]
pub fn read_field_from_toml(toml_string: &str, field_name: &str) -> String {
    let toml_value = toml_string.parse::<toml::Value>().unwrap();
    toml_value[field_name].as_str()
        .expect(&format!("Field {} not found in provided TOML", field_name))
        .to_owned()
}

#[cfg(test)]
mod tests {
    use crate::sum;

    #[test]
    fn it_works() {
        assert_eq!(sum(2, 2), 4);
    }
}
