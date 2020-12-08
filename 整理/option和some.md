1. Option 类型

        // std::option::Option;

        enum Option<T>{
            Some(T),
            None
        }

2. 示例

        fn match_option<T:Debug>(o:Option<T>){
            match o{
                Some(i)=>println!("{:?}",i),
                Node=>println("nothing"),
            }
        }

        let a: Option<i32>=Some(3);

3. unwrap() 解析some 里面的值

        let a: Option<i32>=Some(3);

        a.unwrap();

4. Option None

        let d: Option<u32> =None; // 

        if d==None {
            
        }