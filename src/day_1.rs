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

/// Returns a the sum of calibration values in a valid newly-improved calibration document
///
/// # Arguments
///
/// * `document` a valid newly-improved calibration document
pub fn get_sum_of_calibration_values_in_document(document: &str) -> i32 {
    let mut ans = 0;
    for line in document.split("\n") {
        ans = ans + get_calibration_value(line)
    }
    ans
}

/// Returns a 2 digit integer given a string containing at least one digit or digit spelt out with letters 
///
/// # Arguments
///
/// * `line` a line from the newly-improved calibration document
pub fn get_calibration_value_part_2(line: &str) -> i32 {
    0
}