mod lib;
use lib::{io_funcs, operations};
use std::collections::HashMap;
use std::{io, io::Write};

fn main() {
    // print the program banner
    println!(
        "\n{:*^51}\n",
        " Welcome to the Employee Record System "
    );

    // create the database
    let mut database: HashMap<String, Vec<String>> = HashMap::new();

    // the main program loop
    'program: loop {
        // show input prompt
        print!("Query: ");
        io::stdout().flush().expect("Failed to flush output");

        // get query from user
        let mut query = String::new();
        io::stdin().read_line(&mut query).expect("Failed to read input");
        query = query.trim().to_string();
    
        // validate the query before converting it
        if query.is_empty() {
            // handle empty query
            println!("Error: Empty query");
        } else {
            // parse query and perform the corresponding operation
            if let Ok(parse_query_result) = io_funcs::parse_query(&query) {
                match [&parse_query_result[0][..], &parse_query_result[1][..], &parse_query_result[2][..]] {
                    ["add", "department", department] => {
                        // add a department
                        if let Ok(_) = operations::add_department(department, &mut database) {
                            println!("Added {} department successfully", department);
                        } else {
                            println!("Failed to add {} department", department);
                        }
                    },
                    ["add", name, department] => {
                        // add employee to department
                        if let Ok(_) = operations::add_to_department(name, department, &mut database) {
                            println!("Added {} to {} successfully", name, department);
                        } else {
                            println!("Failed to add {} to {}", name, department);
                        }
                    },
                    ["remove", "department", department] => {
                        // remove a department
                        if let Ok(_) = operations::remove_department(department, &mut database) {
                            println!("Removed {} department successfully", department);
                        } else {
                            println!("Failed to remove {} department", department);
                        }
                    },
                    ["remove", name, department] => {
                        // add employee to department
                        if let Ok(_) = operations::remove_from_department(name, department, &mut database) {
                            println!("Removed {} from {} successfully", name, department);
                        } else {
                            println!("Failed to remove {} from {}", name, department);
                        }
                    },
                    ["show", "departments", ""] => {
                        // get the list of departments
                        if let Ok(departments) = operations::get_departments(&database) {
                            println!("Departments: {:?}", departments);
                        } else {
                            println!("Failed to retrieve the list of departments");
                        }
                    },
                    ["show", "database", ""] => {
                        // show database
                        println!("Database: {:#?}", &database);
                    },
                    ["show", "employees", department] => {
                        // get employees in a department
                        if let Ok(employees) = operations::get_employees(department, &database) {
                            println!("Employees in {} department: {:#?}", department, employees);
                        } else {
                            println!("Failed to retrieve employees in {} department", department);
                        }
                    },
                    ["search", name, department] => {
                        // add employee to department
                        if let Ok(found) = operations::search_employee(name, department, &database) {
                            if found {
                                println!("Found {} in {} department successfully", name, department);
                            } else {
                                println!("{} is not in {} department", name, department);
                            }
                        } else {
                            println!("Failed to search {} in {}", name, department);
                        }
                    },
                    _ => {}
                }
            }
        }
    
        // make exit decision
        'exit: loop {
            match io_funcs::get_exit_option() {
                Ok(opt) => {
                    if opt == 1 {
                        // rerun program
                        continue 'program;
                    } else if opt == 2 {
                        // rerun program
                        break 'program;
                    } else {
                        // rerun exit block
                        continue 'exit;
                    }
                }
                Err(_) => {
                    // rerun exit block
                    continue 'exit;
                }
            }
        }
    }
}
