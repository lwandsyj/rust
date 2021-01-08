use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C"{

   #[wasm_bindgen(js_namespace = console)]
   fn log(s:&str);
}


#[wasm_bindgen]
pub fn greet(s:&str){
    console::log_1(&s.into());
}
