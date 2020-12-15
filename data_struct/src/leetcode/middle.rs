/**
 * 查找中位数
 * 
 */
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // 两个数组合并
    let mut nums = [nums1,nums2].concat();
    // 排序从小到大
    nums.sort();
    println!("sort:{:?}",nums);
    // 数组个数
    let len =nums.len();
    if len==0 {
        return 0.0 ; // i32 转f64
    }
    // 只有一个返回本身
    if len==1{
        return nums[0] as f64;
    }
    // 折半，中间
    let x = len%2;

    let index =len/2;
    if x==0{
        let tmp =(nums[index-1]+nums[index]) as f64;
        return tmp/2.0 ;
    }

    nums[index] as f64
}