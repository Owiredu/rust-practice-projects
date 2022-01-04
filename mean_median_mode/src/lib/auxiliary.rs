/// Round a given floating point number to a specified number of decimal places
pub fn round_number(number: f64, decimal_places: usize) -> f64 {
    match format!("{:.decimal_places$}",
        number,
        decimal_places = decimal_places
    )
    .parse::<f64>() {
        Ok(num) => num,
        Err(_) => number,
    }
}