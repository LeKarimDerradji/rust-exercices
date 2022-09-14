// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.
enum Command {
    Add,
    View,
    Remove,
    Update,
    Total,
}

struct Bill {
    name: String,
    total: i32,
}
// how to push bills into vec or arrays
impl Bill {
    fn create_bill(name: String, total: i32) -> Self {
        Bill {
            name: name,
            total: total,
        }

    }
}


fn failed_attempt() {
    let attempt: i32 = 0;
    println!("invalid output");
    let attempt = attempt + 1;
    if attempt == 3 {
        panic!("Too many attempt, existing the program!");
    }
    
}



use std::io::{self, BufReader};
fn main() {
    let mut bills: Vec<Bill> = Vec::new(); 

    loop {
        let mut input = String::new();
        println!(
        "Menu:
        1: Create Bill
        2: View Bill
        4: Edit Bill
        5: Exit Program"
        );

        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.as_str().trim() {
            "1" => println!("1"),
            "2" => println!("2"),
            "3" => println!("3"),
            "4" => println!("4"),
            "5" => println!("5"),
            _ => failed_attempt(),
        }
        //Error handling
        //Matching the keywords
    }
}
