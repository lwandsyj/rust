fn main() {
    let mut b = vec!["a", "b", "c", "d"];
    b.push("e");
    // for in b 会发生move 移动
   //  for item in b {
   //    println!("{:?}",item);
   //  }

    for i in 0..b.len(){
       let tmp =b[i];
       println!("{}",tmp);
    }
    for i in b.iter(){
      println!("{}",i);
    }
    for i in b.iter_mut(){
       let mut x =i.to_string();
       x.push_str("1");
    }
    println!("{:?}",b);

   
}
