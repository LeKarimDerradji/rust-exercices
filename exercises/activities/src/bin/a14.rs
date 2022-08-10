// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
struct Person {
    age: i32,
    // Owned string to clear up the memory after the program finishes
    name: String,
    favorite_color: String,
}

fn print(data: &str) {
    println!("{:?}", data)
}

fn main() {
    let persons = vec![
        Person {
            age: 8,
            name: "Alice".to_owned(),
            favorite_color: "Pink".to_owned(),
        },
        Person {
            age: 9,
            name: "Bob".to_owned(),
            favorite_color: "Blue".to_owned(),
        },
        Person {
            age: 11,
            name: "Eve".to_owned(),
            favorite_color: "Purple".to_owned(),
        },
    ];

    for person in persons {
        if person.age < 10 {
            println!(
                "The person's name is: {:?} and their favorite color is: {:?}",
                person.name,
                person.favorite_color,
            );
        }
    }
}
