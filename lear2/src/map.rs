use std::collections::BTreeMap;

pub fn map_learn() {
    let mut v = BTreeMap::new();
    v.insert(2, "a");
    v.insert(3, "b");
    v.insert(1, "helo");
    let box_v=Box::new(v);
    println!("{:?}",  *box_v);
}
