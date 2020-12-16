pub fn max_area(height: Vec<i32>) -> i32 {
    // 左边指针
    let mut l=0;
    // 右边指针
    let mut r=height.len()-1;
    // 最大面积
    let mut max_area=0;
    // 双指针
    while l<r {
        // 左边高
        //* rust 接受usize 类型的索引
        let left=height[l];
        // 右边第一个数
        let right=height[r];
        // 数组中的为高度，去最小的值作为高度，不然会有水溢出，无法构成容器
        let min =left.min(right);
        // 宽 两个数的间隔，即长度
        let w=(r-l) as i32;
        // 计算面积
        let area=min*w;
        // 和上一次的面积相比，去最大的值，这样保持永远去最大的值
        max_area=max_area.max(area);
        // 如果是左边小于右边的高度，则移动左指针
        if left<right{
            l=l+1; // 左边向右一个位置
        }else{ // 否则右边指针向左移动一个位置
            r=r-1;
        }
    }
    //返回最大值
    max_area
}
fn test(){
    let  height =vec![1,2,4,5];
    let mut l=0;
    let x=height[l];
}