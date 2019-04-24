use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum(a: usize, b: usize) -> usize {
  a + b
}

#[cfg(test)]
mod tests {
    use crate::sum;

    #[test]
    fn it_works() {
        assert_eq!(sum(2, 2), 4);
    }
}
