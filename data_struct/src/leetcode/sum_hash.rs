
#[derive(Debug,Eq, PartialEq)]
pub struct ListNode{
    pub val:i32,
    pub next:Option<Box<ListNode>>
}
impl ListNode{
    fn new(val:i32)->Self{
        Self{
            next:None,
            val
        }
    }
}
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if l1==None || l2 ==None {
       return None;
    }
    println!("{:?}",l1.unwrap().val);
    None
}

fn main1() {
    let l1 =ListNode::new(243);
    let l2=ListNode::new(564);
    let lb1=Box::new(l1);
    let lb2=Box::new(l2);
    let x=add_two_numbers(Some(lb1),Some(lb2));
    
}
