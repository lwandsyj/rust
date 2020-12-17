use std::collections::{BTreeMap, HashMap};
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    type RtnType = Vec<i32>;
    let mut arr_rtn: Vec<RtnType> = Vec::new();
    let mut map = BTreeMap::new();
    if nums.is_empty() {
        return arr_rtn;
    }
    let len = nums.len();
    for i in 0..len {
        for j in i..len {
            let num1 = nums[i];
            let num2 = nums[j];
            let total = num1 + num2;
            let arr_1 = vec![num1, num2];
            map.insert(total, arr_1);
        }
    }
    for i in 0..len {
        let num = nums[i];
        let key = -num;
        if map.contains_key(&key) {
            let arr = map.get(&key).unwrap();
            let mut tmp = arr.clone();
            tmp.push(num);
            tmp.sort();
            arr_rtn.push(tmp.to_vec());
        }
    }
    println!("{:?}", map);
    arr_rtn
}
fn main() {
    let mut map = BTreeMap::new();
    map.insert(1, "value");
    map.insert(2, "hello");
    map.insert(3, "world");

    for (k, v) in &map {
        println!("k={},v={}", k, v);
    }
    // iter 引用
    for (k,v) in map.iter(){
        println!("k={},v={}", k, v);
    }
    // iter_mut 可变引用
    for (k,v) in map.iter_mut(){
        println!("k={},v={}", k, v);
    }
    // 返回key 数组
    let mut b = BTreeMap::new();
    b.insert(3, "d");
    b.insert(4, "e");
    b.insert(5, "f");

    map.extend(b.into_iter());
    
    println!("{:?}", map);
}

// fn test(arr:i32){
//     println!("{:?}",arr);
// }
