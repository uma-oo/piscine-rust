use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
   let res =  Vec::from_iter(h.values());
   **res.iter().max().unwrap()
   
}