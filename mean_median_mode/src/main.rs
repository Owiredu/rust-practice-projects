mod lib;
use lib::{io_funcs, statistics};

fn main() {
    // print the program banner
    println!(
        "\n{:*^51}\n",
        " Welcome to the Mean, Mode and Median Finder "
    );
    // the main program loop
    'program: loop {
        // initialize the list of integers
        // let mut data: Vec<i32> = vec![3, 48, 82, 1, 59, 883, 3, 93, 1, 20];
        if let Ok(data) = &mut io_funcs::get_data() {
            // print the dataset
            println!("Dataset: {:?}", data);

            // find average or mean value
            if let Ok(val) = statistics::mean(&data) {
                println!("Mean: {}", val);
            } else {
                println!("Mean: <dataset is empty>");
            }

            // find the median value
            data.sort();
            if let Ok(val) = statistics::median(&data) {
                println!("Median: {}", val);
            } else {
                println!("Median: <dataset is empty>");
            }

            // find the mode value
            if let Ok(val) = statistics::mode(&data) {
                println!("Mode: {:?}", val);
            } else {
                println!("Mode: <dataset is empty>");
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
        } else {
            // rerun program
            continue;
        }
    }
}
