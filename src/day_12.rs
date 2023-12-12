use wasm_bindgen::prelude::*;

fn is_valid(condition: &str, contiguous_groups_of_damaged_springs: &Vec<usize>) -> bool {
    let mut prev = None;
    let mut group_size = 0;

    let contiguous_groups_of_damaged_springs_clone = contiguous_groups_of_damaged_springs.clone();
    let mut group_iterator = contiguous_groups_of_damaged_springs_clone.iter();

    for c in condition.chars() {
        if let Some(prev) = prev {
            match c {
            '#' => {
                group_size = group_size + 1;
            },
            '.' => {
                if prev == '#' {
                    // End of a contiguious group, consume one group for checking
                    match group_iterator.next() {
                        Some(size) => {
                            if group_size != *size {
                                return false;
                            }
                        }
                        None => return false,
                    }

                    group_size = 0;
                }
            },
            _ => unreachable!(),
        }
        }
        
        prev = Some(c);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(true, is_valid(".###.##.#...", &vec![3, 2, 1]));
        // Mismatched group size
        assert_eq!(false, is_valid(".##.##.#...", &vec![3, 2, 1]));
        // Extra group
        assert_eq!(false, is_valid(".###.##.#.#.", &vec![3, 2, 1]));
    }
}
