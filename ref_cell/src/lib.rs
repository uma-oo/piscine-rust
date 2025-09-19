use std::rc::Rc;
use std::cell::RefCell;

mod messenger;
pub use messenger::*;



impl Tracker {
    pub fn new(max: usize) -> Self {
        Self {
            messages: RefCell::new(vec![]),
            max: max,
            value: RefCell::new(0),
        }
    }

    pub fn set_value(&self, value: &Rc<usize>) {
        if Rc::strong_count(value) > self.max {
            self.messages.borrow_mut().push("Error: You can't go over your quota!".to_string());
        } else if
            Rc::strong_count(value) <= self.max &&
            (Rc::strong_count(value) as f64) > 0.7 * (self.max as f64)
        {
            self.messages
                .borrow_mut()
                .push(
                    format!(
                        "Warning: You have used up over {}% of your quota!",
                        (Rc::strong_count(&value) * 100) / self.max
                    )
                );
            *self.value.borrow_mut() = Rc::strong_count(&value);
        } else {
            *self.value.borrow_mut() = Rc::strong_count(&value);
        }
    }

    pub fn peek(&self, value: &Rc<usize>) {
        self.messages
            .borrow_mut()
            .push(
                format!(
                    "This value would use {}% of your quota",
                    (Rc::strong_count(&value) * 100) / self.max
                )
            );
    }
}
