use wasm_bindgen::prelude::*;

fn calculate_sequences(history: Vec<isize>) -> Vec<Vec<isize>> {
    let mut sequences: Vec<Vec<isize>> = vec![history];

    loop {
        let mut cont = false;

        let curr = sequences.last().unwrap();
        let mut next: Vec<isize> = Vec::new();
        let n = curr.len();

        for i in 1..n {
            let diff = curr[i] - curr[i - 1];

            // If any of the values are non-zero, we need to repeat the process
            if !cont && diff != 0 {
                cont = true;
            }

            next.push(diff);
        }

        if !cont {
            break;
        }

        sequences.push(next);
    }

    sequences
}

fn determine_next_value(history: Vec<isize>) -> isize {
    let sequences = calculate_sequences(history);

    let mut acc = 0;

    for i in 0..sequences.len() {
        acc = acc + sequences[i].last().unwrap();
    }

    acc
}

#[wasm_bindgen]
pub fn day_9_sum_of_extrapolated_values(report: &str) -> isize {
    let mut acc: isize = 0;

    for history in report.split("\n").map(|line| line.trim()) {
        acc = acc
            + determine_next_value(
                history
                    .split_whitespace()
                    .map(|x| x.trim().parse::<isize>().unwrap())
                    .collect(),
            );
    }

    acc
}

fn determine_previous_value(history: Vec<isize>) -> isize {
    let sequences = calculate_sequences(history);

    let mut acc = 0;

    let n = sequences.len();

    for i in 1..n + 1 {
        acc = sequences[n - i][0] - acc;
    }

    acc
}

#[wasm_bindgen]
pub fn day_9_sum_of_extrapolated_previous_values(report: &str) -> isize {
    let mut acc: isize = 0;

    for history in report.split("\n").map(|line| line.trim()) {
        acc = acc
            + determine_previous_value(
                history
                    .split_whitespace()
                    .map(|x| x.trim().parse::<isize>().unwrap())
                    .collect(),
            );
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45"#;

    #[test]
    fn test_determine_next_value() {
        assert_eq!(18, determine_next_value(vec![0, 3, 6, 9, 12, 15]));
        assert_eq!(28, determine_next_value(vec![1, 3, 6, 10, 15, 21]));
        assert_eq!(68, determine_next_value(vec![10, 13, 16, 21, 30, 45]));
        // Negative answer
        assert_eq!(-1, determine_next_value(vec![14, 11, 8, 5, 2]));
        // Negative d2xdy2
        assert_eq!(-1, determine_next_value(vec![65, 49, 35, 23, 13, 5]));
        // Inflection point as last value
        assert_eq!(11, determine_next_value(vec![17, 11, 8, 8]));
        // Inflection point as first value
        assert_eq!(20, determine_next_value(vec![8, 8, 10, 14]));
    }

    #[test]
    fn test_day_9_sum_of_extrapolated_values() {
        assert_eq!(114, day_9_sum_of_extrapolated_values(EXAMPLE));
    }

    #[test]
    fn test_determine_previous_value() {
        assert_eq!(-3, determine_previous_value(vec![0, 3, 6, 9, 12, 15]));
        assert_eq!(0, determine_previous_value(vec![1, 3, 6, 10, 15, 21]));
        assert_eq!(5, determine_previous_value(vec![10, 13, 16, 21, 30, 45]));
    }

    #[test]
    fn test_day_9_sum_of_extrapolated_previous_values() {
        assert_eq!(2, day_9_sum_of_extrapolated_previous_values(EXAMPLE));
    }
}
