use wasm_bindgen::prelude::*;

use std::collections::HashMap;

/// Returns a 2 digit integer given a string containing at least one digit
///
/// # Arguments
///
/// * `line` a line from the newly-improved calibration document
fn get_calibration_value(line: &str) -> i32 {
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

    format!("{}{}", first_digit.unwrap(), last_digit.unwrap())
        .parse::<i32>()
        .unwrap()
}

/// Returns a the sum of calibration values in a valid newly-improved calibration document
///
/// # Arguments
///
/// * `document` a valid newly-improved calibration document
#[wasm_bindgen]
pub fn day_1_get_sum_of_calibration_values_in_document(document: &str) -> i32 {
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
fn get_calibration_value_part_2(line: &str) -> usize {
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
        ("nine", '9'),
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

    format!("{}{}", first_digit.unwrap(), last_digit.unwrap())
        .parse::<usize>()
        .unwrap()
}

/// Returns a the sum of calibration values in a valid newly-improved calibration document based on part 2
///
/// # Arguments
///
/// * `document` a valid newly-improved calibration document
#[wasm_bindgen]
pub fn day_1_get_sum_of_calibration_values_in_document_part_2(document: &str) -> usize {
    let mut ans = 0;
    for line in document.split("\n") {
        ans = ans + get_calibration_value_part_2(line)
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! generate_parse_calibration_value_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, get_calibration_value(input));
            }
        )*
        }
    }

    generate_parse_calibration_value_tests! {
        test_get_calibration_value_1abc2: ("1abc2", 12),
        test_get_calibration_value_pqr3stu8vwx: ("pqr3stu8vwx", 38),
        test_get_calibration_value_a1b2c3d4e5f: ("a1b2c3d4e5f", 15),
        test_get_calibration_value_treb7uchet: ("treb7uchet", 77),
    }

    macro_rules! generate_parse_calibration_value_part_2_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, get_calibration_value_part_2(input));
            }
        )*
        }
    }

    generate_parse_calibration_value_part_2_tests! {
        test_get_calibration_value_part_2_two1nine: ("two1nine", 29),
        test_get_calibration_value_part_2_pqr3stu8vwx: ("eightwothree", 83),
        test_get_calibration_value_part_2_abcone2threexyz: ("abcone2threexyz", 13),
        test_get_calibration_value_part_2_xtwone3four: ("xtwone3four", 24),
        test_get_calibration_value_part_2_4nineeightseven2: ("4nineeightseven2", 42),
        test_get_calibration_value_part_2_zoneight234: ("zoneight234", 14),
        test_get_calibration_value_part_2_7pqrstsixteen: ("7pqrstsixteen", 76),
        // Shared characters
        test_get_calibration_value_part_2_eighthree: ("eighthree", 83),
        test_get_calibration_value_part_2_sevenine: ("sevenine", 79),
        // Word right after first digit
        test_get_calibration_value_part_2_1nine: ("1nine", 19),
        // Only one word
        test_get_calibration_value_part_2_seven: ("seven", 77),
        // Repeated word on right
        test_get_calibration_value_part_2_one3one: ("one3one", 11),
    }
}
