use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));

}
#[wasm_bindgen]
pub fn consolelog(name: &str) {
    log(&format!("Hello, {}!", name));

}