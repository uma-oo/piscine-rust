#[derive(Clone, Debug, Default)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug, Default)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Debug + std::clone::Clone> List<T> {
    pub fn new() -> List<T> {
        Self {
            head: None,
        }
    }

    pub fn push(&mut self, value: T) {
        match self.head.clone() {
            None => {
                self.head = Some(Node::<T> {
                    value: value,
                    next: None,
                });
            }
            Some(next) => {
                self.head = Some(Node::<T> {
                    value: value,
                    next: Some(Box::new(next)),
                });
            }
        }
    }

    pub fn pop(&mut self) {
        self.head = match self.head.take() {
            None => None,
            Some(next_node) => {
                let val = match next_node.next {
                    None => None,
                    Some(val) => Some(*val),
                };
                val
            }
        };
    }

    pub fn len(&self) -> usize {
        let mut current = self.head.clone();
        let mut len = 0;
        loop {
            match current {
                None => {
                    break;
                }
                Some(next_node) => {
                    match next_node.next {
                        Some(val) => {
                            current = Some(*val);
                        }
                        None => {
                            current = None;
                        }
                    }
                    len += 1;
                }
            }
        }
        len as usize
    }
}
