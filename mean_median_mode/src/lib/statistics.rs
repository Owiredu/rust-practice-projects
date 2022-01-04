use std::collections::HashMap;
use std::cmp::Ordering;
use super::auxiliary;

/// Find the mean or average of a vector of integers
pub fn mean(data: &Vec<i32>) -> Result<f64, ()> {
    // return error with unit type if the data is empty
    if data.len() == 0 {
        return Err(());
    }
    
    // calculate the mean
    let mut sum = 0;
    for val in data.iter() {
        sum += val;
    }

    // return the mean
    Ok(auxiliary::round_number(sum as f64 / data.len() as f64, 2))
}

/// Find the median (the value if the middle position when sorted)
/// of a vector of integers
pub fn median(data: &Vec<i32>) -> Result<f64, ()> {
    // return error with unit type if data is empty
    if data.len() == 0 {
        return Err(());
    }
    // handle even and odd data sizes appropriately
    match data.len() % 2 {
        0 => {
            let floored_half = data.len() / 2;
            Ok((data[floored_half - 1] as f64 + data[floored_half] as f64) / 2 as f64)
        }
        1 => Ok(data[data.len() / 2] as f64),
        _ => Err(()),
    }
}

/// Find the mode (the most frequently occurring value) of a vector of integers
pub fn mode(data: &Vec<i32>) -> Result<Vec<i32>, ()> {
    // return error with unit type if data is empty
    if data.len() == 0 {
        return Err(());
    }

    // create a hash map to hold the value counts
    let mut count_map: HashMap<_, _> = HashMap::new();
    // count each value in the data
    for val in data {
        let count = count_map.entry(val).or_insert(0);
        *count += 1;
    }

    // compose a vector containing the modes and their counts
    let mut modes_with_count: Vec<[i32; 2]> = Vec::new();
    // get the mode(s) with their count(s)
    for (value, count) in count_map.iter() {
        if let Some(prev_mode) = modes_with_count.get(0) {
            match prev_mode[1].cmp(count) {
                Ordering::Less => {
                    modes_with_count.clear();
                    modes_with_count.push([**value, *count]);
                }
                Ordering::Equal => {
                    modes_with_count.push([**value, *count]);
                }
                Ordering::Greater => continue,
            }
        } else {
            modes_with_count.push([**value, *count]);
        }
    }

    // get the modes only
    let mut modes: Vec<i32> = Vec::new();
    for mode_count in modes_with_count.iter() {
        modes.push(mode_count[0]);
    }

    // return the mode(s)
    Ok(modes)
}
