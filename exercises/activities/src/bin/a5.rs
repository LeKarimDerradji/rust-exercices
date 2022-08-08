// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    // Solution 1
    let mut int = 1;
    loop {
        println!("int is equal to: {:?}", int);
        if int == 4 {
            break;
        }
        int += 1;
    }
}
