use std::collections::HashMap;

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
pub fn get_calibration_value_part_2(line: &str) -> usize {
    // Parse digits first, taking note of when they occur
    let (mut first_digit_position, mut first_digit): (Option<usize>, Option<char>) = (None, None);
    let (mut last_digit_position, mut last_digit): (Option<usize>, Option<char>) = (None, None);
    for (i, c) in line.chars().enumerate() { 
        if c.is_digit(10) {
            if first_digit.is_none() {
                first_digit_position = Some(i);
                first_digit = Some(c);
            }
            last_digit_position = Some(i);
            last_digit = Some(c);
        }
    }

    let letters_digits = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ]);

    // Look for word in line
    // If not present, continue to next word
    // If present, check if the first digit has been found yet or occurs after it
    // If so, it is the new first digit
    // Look for word from the right, same logic
    for (letters, digit) in &letters_digits {
        let position = line.find(letters);
        if position.is_none() {
            continue;
        }
        if first_digit_position.is_none() || (position.unwrap() < first_digit_position.unwrap()) {
            first_digit_position = position;
            first_digit = Some(*digit);
        }

        let r_position = line.rfind(letters);
        if last_digit_position.is_none() || r_position > last_digit_position {
            last_digit_position = r_position;
            last_digit = Some(*digit);
        }

    }

    format!("{}{}", first_digit.unwrap(), last_digit.unwrap()).parse::<usize>().unwrap()
}

/// Returns a the sum of calibration values in a valid newly-improved calibration document based on part 2
///
/// # Arguments
///
/// * `document` a valid newly-improved calibration document
pub fn get_sum_of_calibration_values_in_document_part_2(document: &str) -> usize {
    let mut ans = 0;
    for line in document.split("\n") {
        ans = ans + get_calibration_value_part_2(line)
    }
    ans
}