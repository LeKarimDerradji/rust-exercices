// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct LockersData {
    name: String,
    locker_assigned: Option<i32>,
}

impl LockersData {
    fn print_datas(&self) {
        println!("The student name is: {}", self.name);
        match self.locker_assigned {
            Some(value) => println!("The number of the locker is: {:?}", value),
            None => println!("There's no locker assigned to this student."),
        }
    }
}

fn main() {
    let locker_one = LockersData{name: "Karim".to_owned(), locker_assigned: Some(32)};
    let locker_two = LockersData{name: "Eve".to_owned(), locker_assigned: None};

    locker_one.print_datas();
    locker_two.print_datas();
}
