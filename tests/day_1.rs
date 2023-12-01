#[cfg(test)]
mod day_1_tests {
    use advent_of_code_2023::day_1::{
        day_1_get_sum_of_calibration_values_in_document,
        day_1_get_sum_of_calibration_values_in_document_part_2,
    };

    #[test]
    fn test_get_sum_of_calibration_values_in_document() {
        assert_eq!(
            142,
            day_1_get_sum_of_calibration_values_in_document(
                r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#
            )
        );
    }

    #[test]
    fn test_get_sum_of_calibration_values_in_document_part_2() {
        assert_eq!(
            281,
            day_1_get_sum_of_calibration_values_in_document_part_2(
                r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#
            )
        );
    }
}
