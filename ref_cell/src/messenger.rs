
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug)]
pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    value: RefCell<usize>,
    max: usize,
}