/*
* 罗马数字转数字
* 返回值为1~3999
*/
// 导入map 引用
use std::collections::BTreeMap;
pub fn roman_to_int(s: String) -> i32 {
    let mut rtn = 0;
    // 要插入元素，因此定义为可变引用
    let mut map = BTreeMap::new();
    map.insert("M", 1000);
    map.insert("CM", 900);
    map.insert("D", 500);
    map.insert("CD", 400);
    map.insert("C", 100);
    map.insert("XC", 90);
    map.insert("L", 50);
    map.insert("XL", 40);
    map.insert("X", 10);
    map.insert("IX", 9);
    map.insert("V", 5);
    map.insert("IV", 4);
    map.insert("I", 1);
    // 获取字符串长度
    let len = s.len();
    // 字符串转换为数组，方便使用索引
    let arr: Vec<char> = s.chars().collect();
    // 当下一个元素和当前元素合并使用时，标记为true, 要跳过循环
    let mut next_use=false;

    println!("arr={:?}",arr);
    for i in 0..len {
        // 当当前元素已经和上一个元素合并使用过了，要跳过当前元素，并恢复标记默认值
        if next_use{
            next_use=false;
            continue;
        }
        // 当为最后一个元素，直接取值
        if i == len - 1 {
            let cur = arr[i].to_string();
            let key=cur.as_str();
            if map.contains_key(key){
                // map 定义的是常量，上面if语句已经判断了存在key,则可定有值，因此不用判断是否None
                let v = map.get(key).unwrap(); // unwrap 解析Result 和 Some 的值
                // v 为引用i32 类型，因此获取原值要解引用
                rtn += *v;
                println!("if:i={},key={},val={}",i,key,v);
            }
        } else {
            // 当前索引
            let cur = arr[i].to_string();
            let cur_b=cur.as_str();
            // 下一个元素
            let last = arr[i + 1].to_string();
            // 联合，因为最多为两位数
            let tmp = format!("{}{}", cur, last);
            let key = tmp.as_str();
            // 是否包含联合索引，包含按两个查询
            if map.contains_key(key) {
                // 标记下一个联合使用，直接跳过，不在判断
                next_use=true;
                let v = map.get(key).unwrap();
                println!("else:if:i={},key={},val={}",i,key,v);
                rtn += *v;
            }else if map.contains_key(cur_b){
                // 没有直接使用当前
                let v = map.get(cur_b).unwrap();
                rtn += *v;
                println!("else:else:i={},key={},val={}",i,cur,v);
            }
        }
    }
    rtn
}
fn test() {
    let x="hello";
    let y="world";
    let s = x.to_string() + y;
    let x = int_to_roman("LVIII".to_string());
    println!("x=\"{}\"", x);
}
