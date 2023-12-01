#[cfg(test)]
mod day_1_tests {
    use advent_of_code_2023::day_1;

    macro_rules! generate_parse_calibration_value_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, day_1::get_calibration_value(input));
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

    #[test]
    fn test_get_sum_of_calibration_values_in_document() {
        assert_eq!(142, day_1::get_sum_of_calibration_values_in_document(r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#));
    }

    macro_rules! generate_parse_calibration_value_part_2_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, day_1::get_calibration_value_part_2(input));
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

    #[test]
    fn test_get_sum_of_calibration_values_in_document_part_2() {
        assert_eq!(281, day_1::get_sum_of_calibration_values_in_document_part_2(r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#));
    }
}
