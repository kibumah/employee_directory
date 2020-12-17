use std::io;
use std::collections::HashMap;

fn main() {
    println!("Welcome to the Employee Directory!\n");
    println!("What would you like to do? Type 'Help' for more info.");

    let help = format!("{}\n{}\n{}\n{}\n",
        "Type 'Add <first name> <last name> to <department>' to add an employee",
        "Type 'List <department>' to list employees of a department",
        "Type 'List all' to list all employees",
        "Type 'Quit' to quit",
    );

    let mut dir: HashMap<String, Vec<Name>> = HashMap::new();

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
    
        match Command::from_input(&input) {
            Some(Command::Add { dept, employee }) => dir.entry(dept).or_default().push(employee),
            Some(Command::ListAll) => {
                println!();
                for (dept, employees) in &dir {
                    let mut employees = employees.clone();
                    employees.sort_by(|a, b| a.last.cmp(&b.last));
                    println!("{}: ", dept);
                    for employee in employees {
                        println!("{} {}", employee.first, employee.last);
                    }
                    println!();
                }
            }
            Some(Command::List(dept)) => match dir.get(&dept) {
                Some(employees) => {
                    println!();
                    println!("{}:", dept);
                    for employee in employees {
                        println!("{} {}", employee.first, employee.last);
                    }
                    println!();
                },
                None => println!("I don't recognize that department!\n"),
            },
            Some(Command::Help) => println!("{}", help),
            Some(Command::Quit) => break,
            None => continue,
        }
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Name {
    first: String,
    last: String,
}

enum Command {
    Add { dept: String, employee: Name },
    ListAll,
    List(String),
    Help,
    Quit,
}

impl Command {
    fn from_input(s: &str) -> Option<Self> {
        let words: Vec<&str> = s.trim().split_whitespace().collect();
        match words.as_slice() {
            ["Add", first_name, last_name, "to", dept] => Some(Command::Add {
                dept: dept.to_string(),
                employee: Name {
                    first: first_name.to_string(),
                    last: last_name.to_string(),
                },
            }),
            ["List", "all"] => Some(Command::ListAll),
            ["List", dept] => Some(Command::List(dept.to_string())),
            ["Help"] => Some(Command::Help),
            ["Quit"] => Some(Command::Quit),
            _ => None,
        }
    }
}