

fn main() {

   let mut b=String::from("hello");
   let   c=&mut b;
   c.push_str("123");
  println!("{}",b);
    
   println!("{}",c);
}

