use std::{io, cmp::Ordering};
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]

struct Employee {
    name: String,
    department: String
}

impl Employee {
    pub fn new (name:String, department: String) -> Self {
        Employee {
            name,
            department
        }
    }
}

fn receive_input_string(message: &str) -> String {
    loop {
        // receive # of integers to generate
        println!("{}", message);

        //check input error
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: String = match input.trim().parse() {
            Ok(input_string) => input_string,
            Err(_) => continue,
        };
        break input;
    }
}

fn adduser(Employees: &mut Vec<Employee>, input_words: &[&str]) {
    let name = input_words[1].to_string();
    let department = input_words[3].to_string();
    Employees.push(Employee::new(name, department));
}

fn display(Employees: &mut Vec<Employee>, input_words: &[&str]) {
    let scale = input_words[1];
    match scale {
        "Department" => {
            Employees.sort();
            let department = input_words[2];
            for _Employee in Employees {
                if _Employee.department == department {
                    println!("{:?}", _Employee);
                }
            }
        },
        "Company" => {
            Employees.sort();
            println!("{:#?}", Employees);
        }
        _ => println!("Invalid input. Please try again.")
    }
}

fn perform(Employees: &mut Vec<Employee>, input_words: &[&str]) -> bool {
    let mut exit = false;
    let action = input_words.first();
    match action {
        None => exit,
        // add user to the db
        Some(&"Add") => {
            adduser(Employees, input_words);
            exit
        },
        // display db based on scale
        Some(&"Display") => {
            display(Employees, input_words);
            exit
        },
        // exit program
        Some(&"Exit") => {
            println!("Exiting program.");
            exit = true;
            exit
        }
        // invalid input
        _ => {
            println!("Invalid input. Please try again.");
            exit
        }
    }
}

fn main() {
    let mut Employees :Vec<Employee> = Vec::new();
    let message = "Please input the action you would like to perform.";
    let mut exit = false;

    while !exit {
        // receive user input
        let user_input = receive_input_string(&message);
        let user_input_split = user_input.split_whitespace();
        let input_words: Vec<&str> = user_input_split.collect();

        // perform action based on user input
        exit = perform(&mut Employees, &input_words);
    }

}
