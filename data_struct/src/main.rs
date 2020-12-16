pub fn max_area(height: Vec<i32>) -> i32 {
    // 左边指针
    let mut l=0;
    // 右边指针
    let mut r=height.len()-1;
    // 最大面积
    let mut max_area=0;
    // 双指针
    while l<r {
        let left=height[l];
        let right=height[r];
        let min =left.min(right);
        let w=(r-l) as i32;
        let area=min*w;
        max_area=max_area.max(area);
        if left<right{
            l=l+1;
        }else{
            r=r-1;
        }
    }
    max_area
}
fn main(){
    let  height =vec![1,2,4,5];
    let mut l=0;
    let x=height[l];
}