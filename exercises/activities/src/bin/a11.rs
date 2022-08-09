// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter
struct GroceryItem {
    quantity: i32, 
    id: i32,
}

fn display_quantity(item: &GroceryItem) {
    println!("The quantity of the item is: {:?}", item.quantity);
}

fn display_id(item: &GroceryItem) {
    println!("The id of the item is : {:?}", item.id);
}

fn main() {
    let rice = GroceryItem{quantity: 23, id: 1};
    display_quantity(&rice);
    display_id(&rice);
}
