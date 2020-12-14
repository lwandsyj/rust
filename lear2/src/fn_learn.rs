pub fn test(mut a: i32) -> i32 {
    a = a + 1;
    a
}
pub fn test_brow(a: &mut i32) -> i32 {
    *a = *a + 1;
    *a
}

/**
 *  调用
 *  let mut a = 1;
    let b = test_brow(&mut a);
    println!("{}", a);
    println!("{}", b);
 */

 // 引用类型数组可以是不固定长度
fn test1(a:&[i32]){
    println!("{:?}",a);
 }
 fn main1() {
   let a:[i32;5]=[1,2,3,4,5];
   test1(&a);
   println!("{:?}",a);
   let b:[i32;3]=[3,4,5];
   test1(&b);
 }
 