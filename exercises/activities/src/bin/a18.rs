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
    age: i32,
}

impl Customer {
    fn is_allowed(&self) -> Result<String, String> {
        if self.age >= 21 {
            Ok("User able to make that purchase".to_owned())
        } else {
            Err("User is unable to make that purchase".to_owned())
        }
    }
}

fn main() {
    let customer_1 = Customer{age: 18};
    let customer_2 = Customer{age: 21};

    let purchase_1 = customer_1.is_allowed();
    let purchase_2 = customer_2.is_allowed();

    println!("{:?} {:?}", purchase_1, purchase_2);

}
