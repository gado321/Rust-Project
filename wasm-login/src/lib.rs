mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-login!");
}

#[wasm_bindgen]
pub fn login(uname: &str, pwd: &str) -> bool {
    if uname == "gado123" && pwd == "gado321" {
        true
    }
    else {
        false
    }
}

#[wasm_bindgen]
pub fn login_failure() {
    alert("Invalid credentials !!! Try again...");
}

