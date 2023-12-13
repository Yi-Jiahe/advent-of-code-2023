#[derive(Debug, PartialEq)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

fn parse_notes(notes: &str) -> Vec<Vec<Vec<char>>> {
    let mut patterns: Vec<Vec<Vec<char>>> = Vec::new();

    let mut pattern: Vec<Vec<char>> = Vec::new();
    for line in notes.split("\n").map(|line| line.trim()) {
        if line == "" {
            patterns.push(pattern);
            pattern = Vec::new();
            continue;
        }
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        pattern.push(row);
    }
    if pattern.len() != 0 {
        patterns.push(pattern);
    }

    patterns
}

fn find_lines_of_reflection(pattern: &Vec<Vec<char>>) -> Vec<Reflection> {
    let mut lines = Vec::new();

    let n = pattern.len();
    'iterate_rows: for i in 1..n {
        let mut pairs = Vec::new();
        // Generates pairs of indices around the split
        if i < n / 2 + 1 {
            for j in 0..i {
                pairs.push((i - (j + 1), i + (j)));
            }
        } else {
            for j in i..n {
                let diff = j - i;
                pairs.push((i - (diff + 1), i + diff));
            }
        }
        // All pairs must be mirrored
        for (l, r) in pairs {
            if pattern[l] != pattern[r] {
                continue 'iterate_rows;
            }
        }

        lines.push(Reflection::Horizontal(i));
    }

    let rotated_pattern = rotate_pattern(pattern);
    let m = rotated_pattern.len();
    'iterate_cols: for i in 1..m {
        let mut pairs = Vec::new();
        if i < m / 2 + 1 {
            for j in 0..i {
                pairs.push((i - (j + 1), i + (j)));
            }
        } else {
            for j in i..m {
                let diff = j - i;
                pairs.push((i - (diff + 1), i + diff));
            }
        }
        for (l, r) in pairs {
            if rotated_pattern[l] != rotated_pattern[r] {
                continue 'iterate_cols;
            }
        }

        lines.push(Reflection::Vertical(i));
    }

    lines
}

fn rotate_pattern(pattern: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let m = pattern[0].len();

    let mut rotated_pattern = vec![Vec::new(); m];
    for row in pattern {
        for (i, c) in row.iter().enumerate() {
            rotated_pattern[i].push(*c);
        }
    }

    rotated_pattern
}

pub fn day_13_summarize_notes(notes: &str) -> usize {
    let mut acc = 0;

    let patterns = parse_notes(notes);

    for pattern in patterns {
        // There is guaranteed to be only one line for the original pattern
        // From my experience
        match find_lines_of_reflection(&pattern)[0] {
            Reflection::Vertical(i) => {
                acc = acc + i;
            }
            Reflection::Horizontal(i) => {
                acc = acc + (i * 100);
            }
        }
    }

    acc
}

fn print_pattern(pattern: &Vec<Vec<char>>) {
    for row in pattern {
        println!("{}", row.iter().collect::<String>());
    }
}

pub fn day_13_summarize_notes_part_2(notes: &str) -> usize {
    let mut acc = 0;

    let patterns = parse_notes(notes);

    'outer: for pattern in patterns {
        let original_line_of_reflection = &find_lines_of_reflection(&pattern)[0];

        let n = pattern.len();
        let m = pattern[0].len();

        for i in 0..n {
            for j in 0..m {
                let mut new_pattern = pattern.clone();
                new_pattern[i][j] = match new_pattern[i][j] {
                    '#' => '.',
                    '.' => '#',
                    _ => unreachable!(),
                };

                let lines_of_reflection = find_lines_of_reflection(&new_pattern);

                // There can be multiple lines of reflection in the new pattern
                for line in lines_of_reflection {
                    // A different reflection line must be valid
                    if line == *original_line_of_reflection {
                        continue;
                    }

                    match line {
                        Reflection::Vertical(x) => {
                            acc = acc + x;
                            continue 'outer;
                        }
                        Reflection::Horizontal(x) => {
                            acc = acc + (x * 100);
                            continue 'outer;
                        }
                    }
                }
            }
        }
        // All corrected patterns should have a different valid reflection line
        unreachable!();
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"#.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.
    
    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#"#;

    // From https://www.reddit.com/r/adventofcode/comments/18hitog/2023_day_13_easy_additional_examples/
    const ADDITIONAL_EXAMPLE: &str = r#"#.##..##.
    ..#.##.#.
    ##......#
    ##......#
    ..#.##.#.
    ..##..##.
    #.#.##.#.
    
    #...##..#
    #....#..#
    ..##..###
    #####.##.
    #####.##.
    ..##..###
    #....#..#
    
    .#.##.#.#
    .##..##..
    .#.##.#..
    #......##
    #......##
    .#.##.#..
    .##..##.#
    
    #..#....#
    ###..##..
    .##.#####
    .##.#####
    ###..##..
    #..#....#
    #..##...#"#;

    #[test]
    fn test_parse_notes() {
        let patterns = parse_notes(EXAMPLE);

        assert_eq!(
            Reflection::Vertical(5),
            find_lines_of_reflection(&patterns[0])[0]
        );
        assert_eq!(
            Reflection::Horizontal(4),
            find_lines_of_reflection(&patterns[1])[0]
        );
    }

    #[test]
    fn test_find_line_of_reflection() {
        let patterns = parse_notes(
            r#"..##..##...
        ..#.##.#.##
        ...#.#.#...
        ##.#.####..
        ...###.#.##
        ######.#.##
        ######..###"#,
        );

        assert_eq!(
            Reflection::Vertical(1),
            find_lines_of_reflection(&patterns[0])[0]
        );
        assert_eq!(
            Reflection::Vertical(10),
            find_lines_of_reflection(&patterns[0])[1]
        );
    }

    #[test]
    fn test_day_13_summarize_notes() {
        assert_eq!(405, day_13_summarize_notes(EXAMPLE));
        assert_eq!(709, day_13_summarize_notes(ADDITIONAL_EXAMPLE));
    }

    #[test]
    fn test_day_13_summarize_notes_part_2() {
        assert_eq!(400, day_13_summarize_notes_part_2(EXAMPLE));
        assert_eq!(1400, day_13_summarize_notes_part_2(ADDITIONAL_EXAMPLE));
    }
}
