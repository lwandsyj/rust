fn main() {
   let a =String::from("hello");
   let b=String::from("world");
   for i in a.chars(){
       println!("{}",i);
   }
   let len=a.len();
   for i in 0..len{
       let item=a.chars().into_iter().nth(i);
       println!("{:?}",item.unwrap());
   }

}
