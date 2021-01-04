fn test() {
    let  s=String::from("hello");
   
    let  f=|x| [s,x].concat();
   
      f(String::from("word"));
   
     // println!("{}",s);
   
   
   }
   
   fn test2(){
       let x=12;
       let f=move |y| y+x;
       f(3);
       println!("{}",x)
   }
   
   fn test3(){
       // `Vec` 在语义上是不可复制的。
       let haystack = vec![1, 2, 3];
       let s =String::from("hello");
       let contains = move |needle| s.starts_with(needle);
   
       println!("{}", contains("h"));
       println!("{}", contains("s"));
       let greeting = "hello";
       // 不可复制的类型。
       // `to_owned` 从借用的数据创建有所有权的数据。
       let mut farewell = greeting.to_owned();
       println!("{}",greeting);
       //println!("There're {} elements in vec", s);
   }
   
   fn test3(){
       let mut haystack = vec![1, 2, 3];
       {
           println!("{:?}",haystack);
       }
       println!("{:?}",haystack);
       println!("{:?}",haystack);
   }