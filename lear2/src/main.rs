fn main() {
    let  a=[1,2,3,4,5];
    let mut x = Vec::from(a);
    let y:Vec<i32> = Vec::new();
    // 元素个数
    println!("{:?}",a.len());
    // 是否为空
    println!("{:?}",y.is_empty());
    // 数组切片
    println!("{:?}",&x[0..3]);
    // 追加元素
    x.push(6);
    println!("{:?}",x);
    
    x.insert(1, 10);
    println!("{:?}",x);
    let end =x.pop();
    println!("end is :{}",end.unwrap());
    println!("{:?}",x);
    // 翻转
    x.reverse();
    println!("{:?}",x);
    // 排序
    x.sort();
    println!("{:?}",x);
    // 移除
    let d =x.remove(2);
    println!("{}",d);
    println!("{:?}",x);


}
