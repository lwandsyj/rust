1. 类似接口，抽象类或者mixin

2. 约束

+ 让impl trait用在函数参数中：

        fn test(f: impl Fn(i32)->i32){}

+ 让impl trait用在类型别名中

        type MyIter = impl Iterator<Item=i32>;

+ 让impl trait用在trait中的方法参数或返回值中

        trait Test {
             fn test() -> impl MyTrait;
        }

+ 让impl Trait用在trait中的关联类型中：

        trait Test {
            type AT = impl MyTrait;
        }