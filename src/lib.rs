pub mod day_1;
pub mod day_2;
pub mod day_3;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, advent-of-code-2023!");
}
