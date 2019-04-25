extern crate crypto;

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

#[cfg(test)]
mod tests {
    use crate::sum;

    #[test]
    fn it_works() {
        assert_eq!(sum(2, 2), 4);
    }
}
