fn main(){
  let s =vec![1,2,3,4];
  let x=test_slice(&s);
  println!("{:?}",x);
}

fn test_slice(s:&Vec<i32>)->&[i32]{
    &s[0..2]
}