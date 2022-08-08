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
        if int < 4 {
            int += 1;
        } else {
            break;
        }
    }

    // Solution 2 with while loop
    let mut another_int = 1;

    while another_int < 5 {
        println!("int is equal to: {:?}", another_int);
        another_int += 1;
    }
}
