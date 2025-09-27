#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Self {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let index = s.products
            .iter()
            .position(|(product_name, _)| product_name.to_string() == ele)
            .unwrap();
        self.items.push(s.products[index as usize].clone());

        self.generate_receipt();
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        self.receipt = Vec::new();
        let mut prices: Vec<f32> = self.items
        .iter()
        .map(|&(_, price)| price)
        .collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut cheapests: Vec<f32> = prices[0..self.items.len()/3 as usize].to_vec();
        let total_price: f32 = self.items
            .iter()
            .map(|&(_, price)| price)
            .sum();
      
        let price_after_discount: f32 = total_price - cheapests.iter().sum::<f32>();
        let discount = 1.0 - price_after_discount / total_price;
        for (_, price) in self.items.iter() {
            self.receipt.push(((price - price * discount)*100.0).round()/100.0);
        }
        self.receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt.clone()
    }
}
