fn main() {
    testFor();
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

    println!("{}",s.len());
}

fn testFor(){
    for i in 1..10 {
        println!(i);
    }
}