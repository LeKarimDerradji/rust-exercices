// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn return_tuple(x: i32, y: i32) -> (i32, i32) {
    (x, y)
}

fn main() {
    let coord = return_tuple(8, 5);
    if coord.1 > 5 {
        println!("greater than 5");
    } else if coord.1 < 5 {
        println!("lesser than 5");
    } else {
        println!("equal to 5");
    }
}
