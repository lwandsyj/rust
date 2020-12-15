
pub fn my_atoi(s: String) -> i32 {
    // 判断字符串是否为空
    if s.is_empty() {
        return 0;
    }
    // 出去两端空格
    let mut str = s.trim();
    let mut f = true; // + 号为true,-为负
    // 存储数字字符
    let mut arr: Vec<char> = vec![];
    // 遍历
    for i in 0..str.len() {
        // 根据下标获取单个字符
        let item = str.chars().nth(i).unwrap();
        //item.is_digit(radix)
        // 第一个字符是否正负号
        // 第一个字符不为数字或者正负号就直接表示无法转换，返回默认值0
        if i == 0 {
            if item == '+' {
                f = true;
            } else if item == '-' {
                f = false;
            } else {
                f = true;
                let x = item.to_string().parse::<i32>();
                if x.is_err() {
                    // 终止函数
                    return 0;
                }
                arr.push(item);
            }
        } else {
            // 遇到不能转换为数组的字符直接退出循环
            let x = item.to_string().parse::<i32>();
            if x.is_err() {
                // 终止循环
                break;
            }
            arr.push(item);
        }
    }
    // 数组为空直接返回默认值
    if arr.is_empty(){
        return 0;
    }
    // 可以转换成数字的集合转换为字符串
    let rtn: String = arr.iter().collect();
    // 最大值
    let max = i32::MAX;
    // 最小值
    let min = i32::MIN;
    let i_rtn = rtn.parse::<i32>();
    if i_rtn.is_err() {
        if rtn > max.to_string() || rtn > min.to_string() {
            if f {
                return max;
            } else {
                return min;
            }
        }
        return 0;
    }
    if f {
        i_rtn.unwrap()
    } else {
        -i_rtn.unwrap()
    }
}

fn test() {
    let x = my_atoi("   -42".to_string());
    println!("x={}", x);
    let x = my_atoi("words and 987".to_string());
    println!("x={}", x);
    let x = my_atoi("4193 with words".to_string());
    println!("x={}", x);
    let max = i32::MAX;
    let min = i32::MIN;
    let y = "91283472332";
    println!("max={}", max);
    println!("min={}", min);
    println!("x==y:{}", y.to_string() > max.to_string());
}
