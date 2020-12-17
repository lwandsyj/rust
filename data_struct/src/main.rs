use std::collections::BTreeMap;
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    type RtnType=Vec<i32>;
    let mut arr_rtn:Vec<RtnType>=Vec::new();
    let mut map =BTreeMap::new();
    if nums.is_empty(){
        return arr_rtn;
    }
    let len=nums.len();
    for i in 0..len{
        for j in i..len{
            let num1=nums[i];
            let num2=nums[j];
            let total=num1+num2;
            let  arr_1=vec![num1,num2];
            map.insert(total,arr_1 );
        }
    }
    for i in 0..len{
        let num=nums[i];
        let key =-num;
        if map.contains_key(&key){
            let arr=map.get(&key).unwrap();
            let mut tmp=arr.clone();
            tmp.push(num);
            tmp.sort();
            arr_rtn.push(tmp.to_vec());
        }
    }
    println!("{:?}",map);
    arr_rtn
}
fn main(){
    
    let mut v: Vec<i32> = vec![1, 2, 3, 4];
    for item in v.iter_mut(){
        *item=*item+1;
        println!("item={}",item);
    }
    // 输出4
    println!("v 的容量是 {:?}", v);
}