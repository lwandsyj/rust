#[derive(Debug)]
struct Student{
    name:String,
    age:i32
}
impl Student {
    fn set_age(&mut self,name:String){
        self.name=name;
    }
}
fn main(){
    let mut stu=Student{
        name:String::from("zhangsan"),
        age:12
    };
    stu.set_age(String::from("lisi"));
    println!("{:?}",stu);
}