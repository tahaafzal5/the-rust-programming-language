// Exercise: Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company;
// for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all
// people in the company by department, sorted alphabetically.
use std::collections::HashMap;
use std::io;

fn print_welcome() {
    println!("Type 'Add <first_name> to <department>' to add an employee");
    println!("\tE.g. Add Sally to Engineering");
    println!("Type 'List <department>' to list the employees of a department");
    println!("\tE.g. List Sales");
    println!("Type 'All' to list all employees by department");
    println!("Type 'Quit' to quit");
}

fn main() {
    print_welcome();

    let mut department_to_employees: HashMap<String, Vec<String>> = HashMap::new();

    let mut input = String::new();
    loop {
        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match Command::from_input(&input) {
            Some(Command::Add { name, department }) => department_to_employees
                .entry(department)
                .or_default()
                .push(name),
            Some(Command::List { department }) => {
                print_employees_in_department(&department, &department_to_employees);
            }
            Some(Command::All) => {
                for department in department_to_employees.keys() {
                    print_employees_in_department(&department, &department_to_employees);
                }
            }
            Some(Command::Quit) => break,
            None => println!("Input error!"),
        }
    }
}

fn print_employees_in_department(department: &str, map: &HashMap<String, Vec<String>>) {
    if let Some(employees) = map.get(department) {
        println!("{} department has the following Employees:", department);
        for name in employees {
            println!("\t{}", name);
        }
    } else {
        println!("No employees found in {} department.", department);
    }
}

enum Command {
    // Using named fields instead of Add(String, String) because dept and name
    // are the same type and could get mixed up.
    Add { name: String, department: String },
    List { department: String },
    All,
    Quit,
}

impl Command {
    fn from_input(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.trim().split_whitespace().collect();

        match words.as_slice() {
            ["Add", name, "to", department] => Some(Command::Add {
                name: name.to_string(),
                department: department.to_string(),
            }),
            ["List", department] => Some(Command::List {
                department: department.to_string(),
            }),
            ["All"] => Some(Command::All),
            ["Quit"] => Some(Command::Quit),
            _ => None,
        }
    }
}
