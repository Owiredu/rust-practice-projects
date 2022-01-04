use std::{io, io::Write};

/// Get the list of integers from the user
pub fn get_data() -> Result<Vec<i32>, ()> {
    // show prompt
    print!("Input data (comma separated list of integers): ");
    io::stdout().flush().expect("Failed to flush output");

    // receive data from user input
    let mut data_string = String::new();
    match io::stdin().read_line(&mut data_string) {
        Ok(_) => {
            // process input and store it in a vector
            let mut data: Vec<i32> = Vec::new();
            for num in data_string.split_terminator(",") {
                // ignore empty string data values
                if !num.trim().is_empty() {
                    match num.trim().parse::<i32>() {
                        Ok(val) => data.push(val),
                        Err(err) => {
                            println!("{}", err);
                            return Err(());
                        }
                    }
                }
            }
            // return the data
            Ok(data)
        }
        Err(err) => {
            println!("{}", err);
            Err(())
        }
    }
}

/// Get the exit option from the user
pub fn get_exit_option() -> Result<i32, ()> {
    // show program exit options
    println!(
        "\n{}\n{space:>pad$}{restart}\n{space:>pad$}{exit}",
        "Choose option to restart or exit program:",
        space = "",
        restart = "1. Restart",
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
