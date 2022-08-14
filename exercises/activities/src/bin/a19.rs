// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;

fn main() {
    let mut furnitures = HashMap::new();
    furnitures.insert("Chairs", 5);
    furnitures.insert("Beds", 3);
    furnitures.insert("Tables", 2);
    furnitures.insert("Couches", 0);

    for (item, stocks) in furnitures {
        match stocks {
            0 => println!("The item {} is out of stocks.", item),
            _ => println!("There is {} {} in stocks.", stocks, item),
        }
    }
}
