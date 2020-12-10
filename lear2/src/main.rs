
fn test_slice(slice:&[u32])->String{
    slice.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("")
    
}

type state =Vec<i32>;
fn main(){
   let x=&1;
   if *x==1 {
        println!("{}",x);
   }
}