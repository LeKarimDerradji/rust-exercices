// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not
use std::io::BufRead;

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    let mut input = String::new();
    println!("What's your name? ");
    std::io::BufReader::new(std::io::stdin()).read_line(&mut input).unwrap();
    let user_one = find_user(&input.to_lowercase().trim()).map(|id| User{user_id:id, name: input});
    println!("{:?}", user_one);
}
