#![feature(use_extern_macros)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
//    fn appendNumberToBody(x: u32);
    fn appendStringToBody(s: &str);
//    fn alert(x: u32);
}

#[wasm_bindgen]
pub extern fn run() {

//        appendNumberToBody(42);
//        alert(36);
    appendStringToBody("Hi from Rust");
}