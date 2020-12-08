pub trait Add<RHS =Self> {
    type Output;
    fn add(self,rhs:RHS) -> Self::Output;
}

impl Add for i32 {
    type Output = i32;
    fn add(self,rhs:i32)->i32{
        self+rhs
    }
}
fn main(){
    let x:i32 = 1;
    let z=x.add(3);
    println!("{:?}",z);
     
}

