use std::collections::HashMap;
pub fn two_sum1(nums:Vec<i32>,target:i32)->Vec<i32>{
    let len =nums.len();
    let mut arr:Vec<i32> =vec![];
    for i in 0..len-1{
        for n in (i+1)..len{
            let x =nums[i];
            let y=nums[n];
            if x+y ==target{
                arr.push(i as i32);
                arr.push(n as i32);
            }
        }
    }
    arr
}


pub fn two_sum(nums:Vec<i32>,target:i32)->Vec<i32>{
    let mut obj =HashMap::new();
    let mut arr =vec![];
    for n in 0..nums.len(){
        let tmp = nums[n];
        let m =target-tmp;
        let val = obj.get(&m);
        if val!=None{
            let start=obj.get(&m).unwrap();
           arr.push(*start);
           arr.push(n as i32);
           return arr;
        }
        obj.insert(tmp,n as i32);
    }
    arr
}