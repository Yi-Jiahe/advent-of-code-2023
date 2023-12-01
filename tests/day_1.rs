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
}
