

fn main() {
   let  a="hello";
   let mut b=a;
   b="world";
   println!("{}",a);
    
   println!("{}",b);
}

fn show(b:&str){
    println!("{}",b);
}

