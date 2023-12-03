use regex::Regex;

fn load_schematic(schematic: &str) -> Vec<Vec<char>> {
    schematic
        .split("\n")
        .collect()
}

pub fn day_3_get_sum_of_part_numbers(schematic: &str) -> usize {
    let mut ans = 0;
    let indexable_schematic = load_schematic(schematic);
    let n = indexable_schematic.len();
    let m = indexable_schematic[0].len();
    for (i, line) in schematic.split("\n").enumerate() {
        let re = Regex::new(r"(\d+)").unwrap();
        for (s, e, number) in re
            .find_iter(line)
            .map(|m| (m.start(), m.end(), m.as_str().parse::<usize>().unwrap()))
        {
            let u = if i >= 1 { i - 1 } else { i };
            let d = if i < n - 1 { i + i } else { i };
            let l = if s >= 1 { s - 1 } else { s };
            let r = if e < m - 1 { e } else { e - 1 };

            dbg!(&s, &e, &number);
            dbg!(&l, &r);

            let include = 'block: {
                for y in u..d {
                    dbg!(&y);
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

            if include { ans = ans + number; }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const example_schematic: &str = r#"467..114..
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
        assert_eq!(4361, day_3_get_sum_of_part_numbers(example_schematic));
    }
}
