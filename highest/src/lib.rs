#[derive(Debug, Clone)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Self {
            numbers: numbers,
        }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        if self.numbers.len() == 0 {
            return None;
        }
        Some(self.numbers[self.numbers.len() - 1])
    }

    pub fn highest(&self) -> Option<u32> {
        let mut highest: Option<u32> = None;
        for number in self.numbers {
            if highest.is_none() || number > &highest.unwrap() {
                highest = Some(*number);
            } else {
                continue;
            }
        }

        highest
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut data: Vec<_> = self.numbers.iter().collect();
        data.sort();
        let mut result = Vec::new();
        let mut i = 0;
        while i < 3 {
            result.push(*data[i]);
            i+=1;
        }

        result
    }
}
