use wasm_bindgen::prelude::*;

fn determine_next_value(history: Vec<isize>) -> isize {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_next_value() {
        assert_eq!(18, determine_next_value(vec![0, 3, 6, 9, 12, 15]));
        assert_eq!(28, determine_next_value(vec![1, 3, 6, 10, 15, 21]));
        assert_eq!(68, determine_next_value(vec![10, 13, 16, 21, 30, 45]));
        // Negative answer
        assert_eq!(-1, determine_next_value(vec![14, 11, 8, 5, 2]));
        // Negative d2xdy2
        assert_eq!(-1, determine_next_value(vec![65, 49, 35, 23, 13, 5]));
    }

    #[test]
    fn test_day_9_sum_of_extrapolated_values() {
        assert_eq!(
            114,
            day_9_sum_of_extrapolated_values(
                r#"0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45"#
            )
        );
    }
}
