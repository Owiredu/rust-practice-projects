use std::{io, io::Write};

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