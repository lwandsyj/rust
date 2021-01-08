extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C"{
    fn alert(s:&str);
}

#[wasm_bindgen]
pub fn greet(name:&str){
    alert(&format!("hello,{}!",name));
}
#[wasm_bindgen]
pub struct Student{
   
     name:String,
     age:i32
}

#[wasm_bindgen]
impl Student {
    #[wasm_bindgen(constructor)]
   pub fn new(name:&str,age:i32)->Self {
       //let n:&'static str=Box::leak(name.to_string().into_boxed_str());
        Self{
            name:name.to_string(),
            age
        }
    }
}