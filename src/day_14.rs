fn parse_input(input: &str) -> Vec<Vec<char>> {
    let mut platform = Vec::new();

    for line in input.split("\n").map(|line| line.trim()) {
        if line == "" {
            continue;
        }

        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        platform.push(row);
    }

    platform
}

fn tilt_platform<'a>(platform: &'a mut Vec<Vec<char>>) -> &'a Vec<Vec<char>> {
    let n = platform.len();
    let m = platform[0].len();

    for j in 0..m {
        let mut rounded_rocks = 0;
        let mut last_anchor = 0;
        for i in 0..n {
            match platform[i][j] {
                '.' => {}
                'O' => {
                    rounded_rocks = rounded_rocks + 1;
                    platform[i][j] = '.'
                }
                '#' => {
                    // Roll rocks
                    for x in last_anchor..(last_anchor + rounded_rocks) {
                        platform[x][j] = 'O'
                    }

                    // Reset anchor variabes
                    rounded_rocks = 0;
                    last_anchor = i + 1;
                }
                _ => unreachable!(),
            }
        }

        if rounded_rocks != 0 {
            for x in last_anchor..(last_anchor + rounded_rocks) {
                platform[x][j] = 'O'
            }
        }
    }

    platform
}

pub fn day_14_calcuate_total_load_on_north_support_beams(input: &str) -> usize {
    let mut platform = parse_input(input);

    tilt_platform(&mut platform);

    let mut acc = 0;
    let n = platform.len();
    for i in 0..n {
        for c in platform[i].iter() {
            if *c == 'O' {
                acc = acc + (n - i);
            }
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"O....#....
    O.OO#....#
    .....##...
    OO.#O....O
    .O.....O#.
    O.#..O.#.#
    ..O..#O..O
    .......O..
    #....###..
    #OO..#...."#;

    #[test]
    fn test_day_14_calcuate_total_load_on_north_support_beams() {
        assert_eq!(
            136,
            day_14_calcuate_total_load_on_north_support_beams(EXAMPLE)
        );
    }
}
