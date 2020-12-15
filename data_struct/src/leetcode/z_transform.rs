/*
* z 字形字符变形
*  
*/
// BTreeMap 索引按序排列
use std::collections::BTreeMap;

pub fn convert(s: String, num_rows: i32) -> String {
    // 初始化Map
    let mut map = BTreeMap::new();
    // 取最大数 "AB"  1
    let len =num_rows.max(s.len() as i32);
    // 划分行
    for i in 0..len {
        let x = Vec::<char>::new();
        map.insert(i, x);
    }
    //规律 0，1，2，3，2，1，0，1，2，3，2，1，0
    // 向下走或者向上走
    let mut down =false;
    // 
    let mut index =0;
    // 遍历字符
    for i in s.chars(){
        // get_mut 表示要修改值
        map.get_mut(&index).unwrap().push(i);
        // rust 中条件语句不需要用括号括起来
        if index==0 || index==num_rows-1{
            // 到顶部或者底部转换方向
            down=!down;
        }
        index =if down {
            index+1
        }else{
            index-1
        }; // if else 用来实现三目运算，此处分号不可以省略
        println!("index={}",index);
      
    }
    println!("{:?}",map);
    // 初始化字符串
    let mut rtn:String=String::new();
    // 获取map 中搜于值
    for item in map.values(){
        // 字符向量转字符串
       let x:String=item.iter().collect();
       // 字符串连接
       rtn.push_str(x.as_str());
    }
    rtn // 最后语句没有分号表示返回 return rtn; 的简写
}