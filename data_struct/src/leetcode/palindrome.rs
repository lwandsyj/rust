/**
 *   数字回文
 */
/*
* 不转字符串
*/
pub fn is_palindrome_int(mut x: i32) -> bool {
    // x 为负数   -121   反转为121- 因为有符号，所以不可能相等
    // 除了 0 以外，所有个位是 0 的数字不可能是回文，因为最高位不等于 0
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    // 折半
    let mut num: i32 = 0;
    while x > num {
        num = num * 10 + x % 10;
        x = x / 10;
    }

    x == num / 10 || x == num
}
pub fn is_palindrome_reverse(x: i32) -> bool {
    // 回文： 反转以后和未反转相等
    let mut arr: Vec<char> = x.to_string().chars().collect();
    // 反转
    arr.reverse();
    // 字符向量转字符串
    let new_str: String = arr.iter().collect();
    if new_str == x.to_string() {
        return true;
    }
    return false;
}
pub fn is_palindrome_tow(x: i32) -> bool {
    let mut arr: Vec<char> = x.to_string().chars().collect();
    let len = arr.len();
    let middle = len / 2; // 二分查找
    for i in 0..=middle {
        let start = arr[i];
        let end = arr[len - i - 1];
        println!("start={},end={}", start, end);
        if start != end {
            return false;
        }
    }
    true
}
fn test() {
    let y = is_palindrome_tow(1221);
    println!("{}", y);
}
