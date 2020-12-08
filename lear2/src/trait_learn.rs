trait IFly {
    fn fly(&self);
}

struct Animal {
    name: &'static str,
}

impl IFly for Animal {
    fn fly(&self) {
        let a =1;
        println!("{}", self.name);
    }
}

pub fn trait_test() {
    let x: Animal = Animal { name: "张三" };
    x.fly();
}
