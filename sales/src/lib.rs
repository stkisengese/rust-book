#[derive(Debug, Clone, PartialEq, Default)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart::default()
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(product) = s.products.iter().find(|(name, _)| name == &ele) {
            self.items.push((product.0.clone(), product.1));
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        if self.items.is_empty() {
            self.receipt = Vec::new();
            return Vec::new();
        }

        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Apply promotion: for every 3 items, the cheapest is free
        let promotion_groups = prices.len() / 3;
        let mut total_discount = 0.0;
        
        for i in 0..promotion_groups {
            total_discount += prices[i * 3]; // The cheapest in each group of 3
        }

        let total_price = prices.iter().sum::<f32>() - total_discount;
        let original_total = prices.iter().sum::<f32>();
        let discount_ratio = total_price / original_total;

        // Apply discount proportionally to all items
        self.receipt = prices.iter()
            .map(|price| (price * discount_ratio * 100.0).round() / 100.0)
            .collect();

        self.receipt.clone()
    } 
}


