use std::collections::{HashMap, HashSet};

fn load_schematic(schematic: &str) -> Vec<Vec<char>> {
    schematic
        .split("\n")
        .map(|line| line.trim().chars().collect())
        .collect()
}

pub fn day_3_get_sum_of_part_numbers(schematic: &str) -> usize {
    let mut ans = 0;
    let indexable_schematic = load_schematic(schematic);
    let n = indexable_schematic.len();
    let m = indexable_schematic[0].len();
    for (i, line) in indexable_schematic.iter().enumerate() {
        let mut s = 0;
        let mut number_str: String = "".to_string();
        for (j, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                if number_str == "" {
                    s = j;
                }
                number_str = format!("{}{}", number_str, c);

                if (j == m - 1) || !indexable_schematic[i][j + 1].is_digit(10) {
                    let number = number_str.parse::<usize>().unwrap();
                    let e = j;

                    let u = if i >= 1 { i - 1 } else { i };
                    let d = if i < n - 1 { i + 1 } else { i };
                    let l = if s >= 1 { s - 1 } else { s };
                    let r = if e < m - 1 { e + 1 } else { e };

                    let include = 'block: {
                        for y in u..=d {
                            if y == i {
                                for x in [l, r] {
                                    let c = indexable_schematic[y][x];
                                    if !c.is_digit(10) && c != '.' {
                                        break 'block true;
                                    }
                                }
                            } else {
                                for x in l..=r {
                                    let c = indexable_schematic[y][x];
                                    if !c.is_digit(10) && c != '.' {
                                        break 'block true;
                                    }
                                }
                            }
                        }
                        false
                    };

                    if include {
                        ans = ans + number;
                    }

                    number_str = "".to_string();
                }
            }
        }
    }

    ans
}

#[derive(Debug)]
struct Part {
    start: usize,
    end: usize,
    number: usize,
}

fn extract_parts(schematic: &str) -> HashMap<usize, Vec<Part>> {
    let mut parts: HashMap<usize, Vec<Part>> = HashMap::new();
    for (i, line) in schematic.split("\n").map(|line| line.trim()).enumerate() {
        let mut parts_on_line = Vec::new();
        let mut s = 0;
        let mut number_str: String = "".to_string();
        let chars: Vec<char> = line.chars().collect();
        let m = chars.len();
        for j in 0..m {
            let c = chars[j];
            if c.is_digit(10) {
                if number_str == "" {
                    s = j;
                }
                number_str = format!("{}{}", number_str, c);

                if (j == m - 1) || !chars[j + 1].is_digit(10) {
                    parts_on_line.push(Part {
                        start: s,
                        end: j,
                        number: number_str.parse::<usize>().unwrap(),
                    });
                    number_str = "".to_string();
                }
            }
        }
        if parts_on_line.len() != 0 {
            parts.insert(i, parts_on_line);
        }
    }

    parts
}

pub fn day_3_get_sum_of_gear_ratios(schematic: &str) -> usize {
    let mut ans = 0;

    let parts = extract_parts(schematic);

    for (i, line) in schematic.split("\n").map(|line| line.trim()).enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '*' {
                let mut adjacent_parts = HashSet::new();

                for y in i - 1..=i + 1 {
                    for x in j - 1..=j + 1 {
                        if y == i && x == j {
                            continue;
                        }
                        let parts_on_line = parts.get(&y);
                        if parts_on_line.is_none() {
                            continue;
                        }
                        for part in parts_on_line.unwrap() {
                            if x >= part.start && x <= part.end {
                                adjacent_parts.insert(part.number);
                            }
                        }
                    }
                }
                if adjacent_parts.len() == 2 {
                    let mut a = 1;
                    for part in adjacent_parts {
                        a = a * part;
                    }
                    ans = ans + a;
                }
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SCHEMATIC: &str = r#"467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598.."#;

    #[test]
    fn test_day_3_get_sum_of_part_numbers() {
        assert_eq!(4361, day_3_get_sum_of_part_numbers(EXAMPLE_SCHEMATIC));
    }

    #[test]
    fn test_day_3_get_sum_of_gear_ratios() {
        assert_eq!(467835, day_3_get_sum_of_gear_ratios(EXAMPLE_SCHEMATIC));
    }
}
