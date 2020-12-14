pub fn test(mut a: i32) -> i32 {
    a = a + 1;
    a
}
pub fn test_brow(a: &mut i32) -> i32 {
    *a = *a + 1;
    *a
}

/**
 *  调用
 *  let mut a = 1;
    let b = test_brow(&mut a);
    println!("{}", a);
    println!("{}", b);
 */