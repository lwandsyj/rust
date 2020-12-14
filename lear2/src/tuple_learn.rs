pub fn learn_tuple(){
    // 省略逗号被当成小括号使用，而不是元组
   let x =("hello");
   // 当元组只有一个元素时，后面的逗号不能省略，
   let a =("hello",);
   // 元组获取值只能使用数组下标常量，
   // 不能使用 a.i 或者 a[i] 这种
   println!("{}",a.0);
   let b=("hello",2,'c');
   println!("{}",b.1);
}