fn test(mut a: i32) -> i32 {
    a = a + 1;
    a
}
fn test_brow(a: &mut i32) -> i32 {
    *a = *a + 1;
    *a
}
fn main() {
    let mut a = 1;
    let b = test_brow(&mut a);
    println!("{}", a);
    println!("{}", b);
}
