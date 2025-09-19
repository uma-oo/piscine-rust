use std::rc::Rc;
use std::alloc::System;
#[derive(Debug)]
pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Self {
        Self { ref_list }
    }

    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        let mut i = 0;
        while i < self.ref_list.len() {
            if Rc::ptr_eq(&element, &self.ref_list[i]) {
                self.ref_list.remove(i);
                i = 0;
            } else {
                i += 1;
            }
        }
    }
}

pub fn how_many_references(ref_list: &Rc<String>) -> usize {
    Rc::strong_count(ref_list)
}
