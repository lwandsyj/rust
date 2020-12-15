pub fn reverse(x: i32) -> i32 {
    //反转向量
    let mut rtn: Vec<char> = x.abs().to_string().chars().collect();
    // 反转向量
    rtn.reverse();
    // 向量重新转成字符串
    let tmp: String = rtn.iter().collect();
    // 转成数字，返回Result 格式
    let i_rtn = tmp.as_str().parse::<i32>();
    // 因为数字有大小限制，有可能转换失败，比如1534236469   反转为9646324351（超出范围）
    if i_rtn.is_err() {
        return 0;
    }
    // result 解析值OK(4)
    let x: i32 = if v { -i_rtn.unwrap() } else { i_rtn.unwrap() };
    x
}

fn test() {
    let y = reverse(1534236469);
    println!("{}", y);
}
