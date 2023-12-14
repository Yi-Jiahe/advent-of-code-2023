use wasm_bindgen::prelude::*;

use std::iter;

fn is_valid(
    condition: &str,
    contiguous_groups_of_damaged_springs: &Vec<usize>,
    complete: bool,
) -> bool {
    let mut prev = None;
    let mut group_size = 0;

    let contiguous_groups_of_damaged_springs_clone = contiguous_groups_of_damaged_springs.clone();
    let mut group_iterator = contiguous_groups_of_damaged_springs_clone.iter();
    let mut next_group_to_match = match group_iterator.next() {
        Some(size) => *size,
        None => 0,
    };

    for c in condition.chars() {
        match c {
            '#' => {
                group_size = group_size + 1;
                if group_size > next_group_to_match {
                    return false;
                }
            }
            '.' => {
                if let Some(prev) = prev {
                    if prev == '#' {
                        if group_size != next_group_to_match {
                            return false;
                        }

                        // End of a contiguous group, consume one group for checking
                        next_group_to_match = match group_iterator.next() {
                            Some(size) => *size,
                            None => 0,
                        };

                        group_size = 0;
                    }
                }
            }
            '?' => {
                // Hitting a question mark will also trigger an early return
                break;
            }
            _ => unreachable!(),
        }

        prev = Some(c);
    }

    // Will return true if the conditions have been fully consumed even if there are groups remaining.
    // This allows it to parse and reject half filled conditions for early return

    // If the whole thing must be correct, check that the remaining conditions are fulfilled
    // There should be no more groups to match, i.e. the iterator should have been fully consumed
    // If it ended on a #, the group_size should match the next group to be matched
    // If it ended on a ., the group_size should be 0 and the group size to match should be 0 (will be set to 0 when the iterator is fully consumed)
    let next_group = group_iterator.next();
    if complete && (!next_group.is_none() || group_size != next_group_to_match) {
        return false;
    }
    true
}

fn replace_char_at_position(s: &str, i: usize, replacement: char) -> String {
    let mut new_string = String::from(s);
    new_string.replace_range(
        new_string
            .char_indices()
            .nth(i)
            .map(|(pos, ch)| (pos..pos + ch.len_utf8()))
            .unwrap(),
        &replacement.to_string(),
    );
    new_string
}

fn find_different_arrangements(
    condition: &str,
    contiguous_groups_of_damaged_springs: &Vec<usize>,
    pos: usize,
) -> usize {
    if !is_valid(condition, contiguous_groups_of_damaged_springs, false) {
        return 0;
    }

    for (i, c) in condition.chars().enumerate() {
        if i < pos {
            continue;
        }
        match c {
            '.' | '#' => continue,
            '?' => {
                return find_different_arrangements(
                    &replace_char_at_position(condition, i, '.'),
                    contiguous_groups_of_damaged_springs,
                    i,
                ) + find_different_arrangements(
                    &replace_char_at_position(condition, i, '#'),
                    contiguous_groups_of_damaged_springs,
                    i,
                );
            }
            _ => unreachable!(),
        }
    }

    if is_valid(condition, contiguous_groups_of_damaged_springs, true) {
        return 1;
    }

    0
}

fn parse_line(line: &str) -> (&str, Vec<usize>) {
    let mut line_iterator = line.split_whitespace();
    let conditions = line_iterator.next().unwrap();
    let groups = line_iterator
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    (conditions, groups)
}

#[wasm_bindgen]
pub fn day_12_sum_of_possible_arrangements(condition_records: &str) -> usize {
    let mut acc = 0;
    for line in condition_records.split("\n").map(|line| line.trim()) {
        let (conditions, groups) = parse_line(line);
        acc = acc + find_different_arrangements(conditions, &groups, 0);
    }

    acc
}

#[wasm_bindgen]
pub fn day_12_sum_of_possible_arrangements_part_2(condition_records: &str) -> usize {
    let mut acc = 0;
    for line in condition_records.split("\n").map(|line| line.trim()) {
        let (conditions, groups) = parse_line(line);
        let new_conditions = iter::repeat(conditions)
            .take(5)
            .collect::<Vec<&str>>()
            .join("?");
        let mut new_groups = groups.clone();
        for _ in 1..5 {
            new_groups.append(&mut groups.clone());
        }
        let a = find_different_arrangements(&new_conditions, &new_groups, 0);
        dbg!(a);
        acc = acc + a;
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! generate_is_valid_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let ((conditions, groups, completed), expected) = $value;
                assert_eq!(expected, is_valid(conditions, groups, completed));
            }
        )*
        }
    }

    generate_is_valid_tests! {
        // Complete and valid
        test_is_valid_complete: ((".###.##.#...", &vec![3, 2, 1], true), true),
        test_is_valid_complete_ends_with_damaged: ((".###.##.#", &vec![3, 2, 1], true), true),
        // Complete and invalid
        test_is_valid_complete_mismatched_group_size: ((".##.##.#...", &vec![3, 2, 1], true), false),
        test_is_valid_complete_extra_group: ((".###.##.#.#.", &vec![3, 2, 1], true), false),
        // Invalid if complete
        test_is_valid_complete_not_all_groups_matched_ending_on_damaged: ((".###......##", &vec![3, 2, 1], true), false),
        test_is_valid_complete_not_all_groups_matched_ending_on_operational: ((".###....##..", &vec![3, 2, 1], true), false),
    }

    #[test]
    fn test_is_valid() {
        // Half filled
        // In the middle of a group
        assert_eq!(true, is_valid(".##", &vec![3, 2, 1], false));
        assert_eq!(true, is_valid(".##?.##.#...", &vec![3, 2, 1], false));
        // Immediately before a group
        assert_eq!(true, is_valid(".###.", &vec![3, 2, 1], false));
        assert_eq!(true, is_valid(".###.?#.#...", &vec![3, 2, 1], false));
        // Immediately after a group
        assert_eq!(true, is_valid(".###.##", &vec![3, 2, 1], false));
        assert_eq!(true, is_valid(".###.##?#...", &vec![3, 2, 1], false));
        // Too many in next group
        assert_eq!(false, is_valid("####.##.#...", &vec![3, 2, 1], false));
    }

    #[test]
    fn test_find_different_arrangements() {
        assert_eq!(1, find_different_arrangements("???.###", &vec![1, 1, 3], 0));
        assert_eq!(
            4,
            find_different_arrangements(".??..??...?##.", &vec![1, 1, 3], 0)
        );
        assert_eq!(
            1,
            find_different_arrangements("?#?#?#?#?#?#?#?", &vec![1, 3, 1, 6], 0)
        );
        assert_eq!(
            1,
            find_different_arrangements("????.#...#...", &vec![4, 1, 1], 0)
        );
        assert_eq!(
            4,
            find_different_arrangements("????.######..#####.", &vec![1, 6, 5], 0)
        );
        assert_eq!(
            10,
            find_different_arrangements("?###????????", &vec![3, 2, 1], 0)
        );
    }

    // Takes a while to run
    #[ignore]
    #[test]
    fn test_find_different_arrangements_part_2() {
        assert_eq!(
            525152,
            day_12_sum_of_possible_arrangements_part_2(
                r#"???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1"#
            )
        );
    }
}
