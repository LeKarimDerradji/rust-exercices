// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

// Notes : 
// We declare the functions outside main()
fn display_first_name() {
    println!("Karim");
}
fn display_last_name() {
    println!("Derradji");
}

fn main() {
    display_first_name();
    display_last_name();

    println!("Is learning Rust");
}
