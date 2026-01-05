#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn greet(name: &str) {
    unsafe {
        log(&format!("Hello, {}!", name));
    }
}

#[cfg(not(feature = "wasm"))]
pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}
