#[derive(Debug)]
pub struct Person<'a>{
    pub name: &'a str,
    pub age: u8,
}

impl<'a> Person<'a> {
    pub fn new(name: &'a str) -> Person<'a> {
        Self {
            name: name,
            age: 0,
        }
    }
}
