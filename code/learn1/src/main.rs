
mod test;
use crate::test::array_test::{test};
use crate::util::redis::{connection,dispose};

trait Shap{
    fn area(&self)->i32;
}

struct Test{
    x:i32,
    y:i32
}

impl Shap for Test{
    fn area(&self)->i32{
        self.x+self.y
    }
}

fn main() {
    let host = String::from("123.0.0.1");
   connection(host,3000);
}

