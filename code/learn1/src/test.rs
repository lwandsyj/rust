pub mod array_test{
    pub fn test(){
        let mut arr:Vec<i32>=Vec::new();
        // 1. 获取数组长度
        println!("数组长度为:{}",arr.len());
        // 2. 数组是否为空
        println!("数组长度是否为空:{}",arr.is_empty());
        // 3. 获取首个元素
       // arr.push(1);
        let first = arr.first(); // Some(1)
        println!("数组第一个元素:{:?}",arr);
        // 4.在末尾添加元素
        arr.push(1);
        arr.push(2);
        println!("数组添加元素:{:?}",arr);
        // 5. 截取
       // let a = arr[1:2];
       // println!("join:{:?}",a) ;
    }
}