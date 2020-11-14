fn main(){
    let mut x:Vec<i32> =vec![2];
    let last =x.pop();
    if last !=None {
        println!("{:?}",last);
        return;
    }
    println!("{}","None");

}

fn test_unwrap(){
    println!("{}",Some("hello").unwrap()); // unwrap 返回Some 中的值hello
}