fn main() {
    let mut x:i32 =5;
    x=get_age(5);
    println!("{}",x);
    get_name();
}

fn get_age(age:i32)->i32{
   if age>0 && age<6 {
       return 6;
   }else if age ==8{
       return 8;
   }else{
       return 3;
   }
}

fn get_name(){
    let s = "broadcast";

    println!("{}",s.to_string());
}