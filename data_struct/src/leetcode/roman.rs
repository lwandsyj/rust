/*
* 数字转罗马数字
* 值为1~3999
*/
pub fn int_to_roman(num: i32) -> String {
    // 默认返回值为空字符串
    let mut rtn = String::new();
    let mut nums = num;
    // 值范围在1~3999，超出范围返回空
    if num < 1 || num > 3999 {
        return rtn;
    }
    let arr = vec![
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    for item in arr {
        // 判断
        while item.0 <= nums {
            nums -= item.0;
            rtn.push_str(item.1);
        }
    }
    rtn
}
fn test() {
    let x = int_to_roman(1994);
    println!("x=\"{}\"", x);
}
