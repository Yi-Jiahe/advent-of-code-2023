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

fn find_line_of_reflection(pattern: &Vec<Vec<char>>) -> Reflection {
    let n = pattern.len();
    'iterate_rows: for i in 1..n {
        let mut pairs = Vec::new();
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
        for (l, r) in pairs {
            if pattern[l] != pattern[r] {
                continue 'iterate_rows;
            }
        }

        return Reflection::Horizontal(i);
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

        return Reflection::Vertical(i);
    }

    unreachable!()
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
        match find_line_of_reflection(&pattern) {
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

    #[test]
    fn test_parse_notes() {
        let patterns = parse_notes(EXAMPLE);

        assert_eq!(
            Reflection::Vertical(5),
            find_line_of_reflection(&patterns[0])
        );
        assert_eq!(
            Reflection::Horizontal(4),
            find_line_of_reflection(&patterns[1])
        );
    }

    #[test]
    fn test_day_13_summarize_notes() {
        assert_eq!(405, day_13_summarize_notes(EXAMPLE));
    }
}
