struct Point{
    x:i32,
    y:i32
}
impl Point{
    fn new(x:i32,y:i32)->Point{
        Point{x,y}
    }
    fn get_y(&mut self)->i32{
        self.y=self.y+4;
        return self.y;
    }
}

fn main() {
   let add =|a:i32,b:i32| a+b;
   let val = add(2,3);
   println!("{}",val);
}


