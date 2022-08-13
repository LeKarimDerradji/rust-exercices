// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase
struct Customer {
    id: i32,
    age: i32,
    country: String,
}

impl Customer {
    fn allowed(&self) -> Result<String> {
        if self.age >= 21 {
            Ok("User able to make that purchase".to_owned()),
        } else {
            Err("User is unable to make that purchase".to_owned())
        }
    }
}

fn main() {
    let customer_1 = Customer{id: 1, age: 18, country: "EN".to_owned()};
    let customer_2 = Customer{id: 2, age: 21, country: "ES".to_owned()};


}
