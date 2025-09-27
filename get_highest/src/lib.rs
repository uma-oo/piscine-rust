#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl Numbers {
    pub fn new(numbers: &[u32]) -> Self {}

    pub fn list(&self) -> &[u32] {}

    pub fn latest(&self) -> Option<u32> {}

    pub fn highest(&self) -> Option<u32> {}

    pub fn highest_three(&self) -> Vec<u32> {}
}
