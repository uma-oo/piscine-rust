#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.v % 2 == 0 {
            self.v = self.v / 2;
        } else {
            self.v = 3 * self.v + 1;
        }
        Some(self.v)
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Self { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    let mut num = Collatz::new(n); 
    let mut steps = 0;
    while num!=(Collatz{v:1}) && num !=  (Collatz{v:0}){
        num.next();
        steps += 1;
    }
    steps
}
