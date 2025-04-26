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

        // Calculate how many free items we get (1 free per 3 items)
        let free_items = prices.len() / 3;
        let total_discount: f32 = prices.iter().take(free_items).sum();
        let total_after_discount = prices.iter().sum::<f32>() - total_discount;

        // Calculate adjustment ratio
        let original_total: f32 = prices.iter().sum();
        let ratio = total_after_discount / original_total;

        // Apply ratio to each item and round to 2 decimal places
        self.receipt = prices.iter()
            .map(|&price| {
                let adjusted = price * ratio;
                (adjusted * 100.0).round() / 100.0
            })
            .collect();

        self.receipt.clone()
    }
}


