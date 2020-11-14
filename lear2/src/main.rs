fn main(){
    let s = String::from("hello");
    let x = s.as_str();
    let d = x.to_string();
    println!("{}",s);
    println!("{}",x);
    println!("{}",d);
}