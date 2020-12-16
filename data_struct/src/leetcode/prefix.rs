/*
* 给定一个数组，查找最长公共前缀
*/
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    // 返回公共前缀字符串
    let mut rtn = String::new();
    // 数组为空，则直接返回空字符串
    if strs.is_empty(){
        return rtn;
    }
    /*
    * 查找最短字符串个数，因为最长的公共前缀不会超过最短字符串，即最极端的情况下为整个字符串
    */
    let mut arr: Vec<usize> = strs.iter().map(|item| item.len()).collect();
    // 排序，从小到大
    arr.sort();
    // 获取最小字符串个数
    let len = arr[0];
    println!("len={}", len);
    // 索引会导致move,所以加上clone
    let start = strs[0].clone();
    // split 分割成数组，当为“” 时，收尾会多加两个“” 空元素
    let start_arr: Vec<&str> = start
        .split("")//分割成数组
        .into_iter()//转成iter
        .filter(|x| !x.is_empty())//过滤空元素
        .collect();// 转回数组
    println!("arr={:?}", start_arr);
    println!("strs={:?}", strs);
    // 以最小字符串个数遍历
    for i in 0..len {
        // 获取字符串
        let first = start_arr[i];
        // 合并字符串 String+&str
        let tmp = rtn.clone() + first;
        // 判断数组中是否每一个元素都以前缀开头
        let flag = strs.iter().all(|item| item.starts_with(tmp.as_str()));
        // true 表示每一个元素都返回true
        if flag {
            // 追加
            rtn.push_str(first);
        } else {
            // false 则终止for 循环
            break;
        }
    }
    rtn
}
fn test() {
    let x = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let y = longest_common_prefix(x);
    println!("{}", y);
}
