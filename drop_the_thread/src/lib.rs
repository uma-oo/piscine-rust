use std::cell::{ Cell, RefCell };

#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl ThreadPool {
    pub fn new() -> Self {
        Self {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_thread(&self, c: String) -> (usize, Thread) {
        let id = self.states.borrow().len();
        self.states.borrow_mut().push(false);
        (id, Thread::new(id, c, &self))
    }

    pub fn thread_len(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id] == true
    }

    pub fn drop_thread(&self, id: usize) {
        match true {
            _ if self.states.borrow_mut()[id] == false => {
                self.states.borrow_mut()[id] = true;
                self.drops.replace(self.drops.get() + 1);
            }
            _ => {
                panic!("{} is already dropped", id);
            }
        }
    }
}

#[derive(Debug)]
pub struct Thread<'a> {
    // expected public fields
    pub pid: i32,
    pub cmd: String,
    pub parent: &'a ThreadPool,
}

impl<'a> Thread<'a> {
    pub fn new(p: usize, c: String, t: &'a ThreadPool) -> Self {
        Self {
            pid: p as i32,
            cmd: c,
            parent: t,
        }
    }

    pub fn skill(self) {
        drop(self);
    }
}

impl Drop for Thread<'_> {
    fn drop(&mut self) {
        self.parent.drop_thread(self.pid as usize);
    }
}
