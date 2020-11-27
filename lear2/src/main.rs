fn main(){
    let mut arr =vec![1,3,100];
    maximum_gap(arr);
}

 fn maximum_gap(mut nums: Vec<i32>) -> i32 {
    let len=nums.len();
    if len < 2 {
        return 0;
    }
    nums.sort();
    let mut max =0;
    for i in 0..len-1 {
        let last = nums[(i+1)];
        let current =nums[i];
        let tmp = last-current;
        if max < tmp {
            max=tmp;
        }
    }
    println!("{:?}",max);
    return max;
}