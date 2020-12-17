fn main(){
    let arr=vec![1,2,3,4];
    // 切片类型可以不用设置长度
    let y=&arr[0..3];

    let z:Vec<i32>=y.iter().cloned().collect();
    println!("{:?}",z);
}