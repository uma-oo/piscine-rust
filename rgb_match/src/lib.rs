use std::mem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        match first {
            _ if first == self.r => {
                match second {
                    _ if second == self.g => mem::swap(&mut self.r, &mut self.g),
                    _ if second == self.b => mem::swap(&mut self.r, &mut self.b),
                    _ if second == self.a => mem::swap(&mut self.r,&mut self.a),
                    _ => panic!(),
                };
            }
            _ if first == self.g => {
                match second {
                    _ if second == self.a => mem::swap(&mut self.g,&mut self.a),
                    _ if second == self.r => mem::swap(&mut self.g, &mut self.r),
                    _ if second == self.b => mem::swap(&mut self.g, &mut self.b),
                    _ => panic!(),
                };
            }
            _ if first == self.b => {
                match second {
                    _ if second == self.a => mem::swap(&mut self.b, &mut self.a),
                    _ if second == self.r => mem::swap(&mut self.b,&mut self.r),
                    _ if second == self.g => mem::swap(&mut self.b, &mut self.g),
                    _ => panic!(),
                };
            }
            _ if first == self.a => {
                match second {
                    _ if second == self.g => mem::swap(&mut self.a, &mut self.g),
                    _ if second == self.b => mem::swap(&mut self.a, &mut self.b),
                    _ if second == self.r => mem::swap(&mut self.a,&mut self.r),
                    _ => panic!(),
                };
            }
            _ => panic!(),
        };

        self
    }
}
