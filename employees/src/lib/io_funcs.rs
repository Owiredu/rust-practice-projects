use std::{io, io::Write};

#[allow(dead_code)]

/// Parse query
/// 
/// # QUERY GUIDELINES
/// 
/// #### To add an employee to department:
/// ```Add <single word employee name> to <single word department name>```
/// 
/// #### To remove an employee from a department:
/// ```Remove <single word employee name> from <single word department name>```
/// 
/// #### To search for an employee in a department
/// ```Search <single word employee name> in <single word department name>```
/// 
/// #### To show the employees in particular department
/// ```Show employees in <single word department name>```
/// 
/// #### To show departments
/// ```Show departments```
/// 
/// #### To show the database:
/// ```Show database```
pub fn parse_query(query: &str) -> Result<[String; 3], ()> {
    // parse query
    let mut query_vec: Vec<String> = query.split_terminator(" ").map(|chunk| chunk.to_string()).collect();
    // remove empty string
    query_vec.retain(|chumk| chumk.trim() != "");
    // check if there are 4 chunks in the query to validate it
    if query_vec.len() != 4 && query_vec.len() != 2 {
        println!("Error: Invalid query");
        println!("{:?}", query_vec);
        return Err(());
    }
    // interpret the command
    match &query_vec[0].to_ascii_lowercase()[..] {
        "add" => {
            if &query_vec[2].to_ascii_lowercase() == "to" {
                Ok(["add".to_string(), query_vec[1].clone(), query_vec[3].clone()])
            } else {
                println!("Error: Invalid command. Consider changing \"{}\" to \"to\"", &query_vec[2]);
                Err(())
            }
        },
        "remove" => {
            if &query_vec[2].to_ascii_lowercase() == "from" {
                Ok(["remove".to_string(), query_vec[1].clone(), query_vec[3].clone()])
            } else {
                println!("Error: Invalid command. Consider changing \"{}\" to \"from\"", &query_vec[2]);
                Err(())
            }
        },
        "show" => {
            match query_vec.len() {
                4 => {
                    if &query_vec[2].to_ascii_lowercase() == "in" {
                        Ok(["show".to_string(), "employees".to_string(), query_vec[3].clone()])
                    } else {
                        println!("Error: Invalid command. Consider changing \"{}\" to \"in\"", &query_vec[2]);
                        Err(())
                    }
                },
                2 => {
                    match &query_vec[1][..] {
                        // handle show departments query
                        "departments" => {
                            Ok(["show".to_string(), "departments".to_string(), "".to_string()])
                        },
                        // handle show database query
                        "database" => {
                            Ok(["show".to_string(), "database".to_string(), "".to_string()])
                        },
                        // handle erroneously composed query
                        _ => {
                            println!("Invalid command. Consider changing \"{}\" to \"departments\" or \"database\"", &query_vec[1]);
                            Err(())
                        }
                    }
                },
                _ => {
                    println!("Invalid command");
                    Err(())
                }
            }
        },
        "search" => {
            if &query_vec[2].to_ascii_lowercase() == "in" {
                Ok(["search".to_string(), query_vec[1].clone(), query_vec[3].clone()])
            } else {
                println!("Error: Invalid command. Consider changing \"{}\" to \"in\"", &query_vec[2]);
                Err(())
            }
        },
        _ => {
            println!("Error: Invalid command \"{}\"", &query_vec[0].to_ascii_lowercase());
            Err(())
        }
    }
}

/// Get the exit option from the user
pub fn get_exit_option() -> Result<i32, ()> {
    // show program exit options
    println!(
        "\n{}\n{space:>pad$}{query}\n{space:>pad$}{exit}",
        "Choose option to query or exit program:",
        space = "",
        query = "1. Run query",
        exit = "2. Exit",
        pad = 5
    );

    // show prompt
    print!("{}", "Option: ");
    io::stdout().flush().expect("Failed to flush output");

    // receive option from the user
    let mut option = String::new();
    match io::stdin().read_line(&mut option) {
        Ok(_) => {
            match option.trim().parse::<i32>() {
                Ok(opt) => {
                    if opt != 1 && opt != 2 {
                        println!("Invalid option");
                    }
                    println!();
                    Ok(opt)
                },
                Err(err) => {
                    println!("{}", err);
                    Err(())
                }
            }
        },
        Err(err) => {
            println!("{}", err);
            Err(())
        }
    }
}