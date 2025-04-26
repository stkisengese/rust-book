use sales::*;

fn main() {
    let store = Store::new(vec![
        (String::from("product A"), 1.23),
        (String::from("product B"), 23.1),
        (String::from("product C"), 3.12)]);

    println!("{:?}", store);

    let mut cart = Cart::new();
    cart.insert_item(&store, String::from("product A"));
    cart.insert_item(&store, String::from("product B"));
    cart.insert_item(&store, String::from("product C"));

    println!("{:?}", cart.generate_receipt());

    println!("{:?}", cart);
}

// And its output:

// $ cargo run
// Store { products: [("product A", 1.23), ("product B", 23.1), ("product C", 3.12)] }
// [1.17, 2.98, 22.06]
// Cart { items: [("product A", 1.23), ("product B", 23.1), ("product C", 3.12)], receipt: [1.17, 2.98, 22.06] }
// $
