/// Returns a 2 digit integer given a string containing at least one digit
///
/// # Arguments
///
/// * `line` a line from the newly-improved calibration document
pub fn get_calibration_value(line: &str) -> i32 {
    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;
    for c in line.chars() { 
        if c.is_digit(10) {
            if first_digit.is_none() {
                first_digit = Some(c);
            }
            last_digit = Some(c);
        }
    }

    format!("{}{}", first_digit.unwrap(), last_digit.unwrap()).parse::<i32>().unwrap()
}