// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTech,
}

enum Status {
    Active,
    Terminated,
}

struct Employee {
    position: Position,
    status: Status,
}

fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => return Err("Access restricted: employee is terminated!".to_owned()),
        _ => (),
    }

    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("Access restricted: invalid position!".to_owned()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    let _attempt_to_access = try_access(employee)?;
    println!("Employee is authorized!");
    Ok(())
}

fn main() {
    let employees = vec![
        Employee {
            position: Position::KitchenStaff,
            status: Status::Active,
        },
        Employee {
            position: Position::Manager,
            status: Status::Active,
        },
        Employee {
            position: Position::Marketing,
            status: Status::Active,
        },
        Employee {
            position: Position::Maintenance,
            status: Status::Terminated,
        },
        Employee {
            position: Position::LineSupervisor,
            status: Status::Active,
        },
        Employee {
            position: Position::AssemblyTech,
            status: Status::Active,
        },
    ];

    // Err
    // Ok
    // Ok
    // Err

    for employee in employees {
        match print_access(&employee) {
            Err(e) => println!("{}", e),
            _ => (),
        }
    }
}
