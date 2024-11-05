use wasm_bindgen::prelude::*;

// JavaScript에서 호출할 수 있도록 #[wasm_bindgen] 어노테이션을 추가합니다.
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
