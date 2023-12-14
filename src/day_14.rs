use crate::utils::print_2d_matrix;

enum Direction {
    North,
    South,
    East,
    West,
}

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

fn tilt_platform<'a>(platform: &'a mut Vec<Vec<char>>, direction: Direction) {
    let n = platform.len();
    let m = platform[0].len();

    match direction {
        Direction::North => {
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
        }
        Direction::South => {
            for j in 0..m {
                let mut rounded_rocks = 0;
                let mut last_anchor = n - 1;
                for i in (0..n).rev() {
                    match platform[i][j] {
                        '.' => {}
                        'O' => {
                            rounded_rocks = rounded_rocks + 1;
                            platform[i][j] = '.'
                        }
                        '#' => {
                            // Roll rocks
                            for x in (last_anchor - rounded_rocks + 1)..=last_anchor {
                                platform[x][j] = 'O'
                            }

                            // Reset anchor variabes
                            rounded_rocks = 0;
                            if i > 0 {
                                last_anchor = i - 1;
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                if rounded_rocks != 0 {
                    for x in (last_anchor - rounded_rocks + 1)..=last_anchor {
                        platform[x][j] = 'O'
                    }
                }
            }
        }
        Direction::East => {
            for i in 0..n {
                let mut rounded_rocks = 0;
                let mut last_anchor = m - 1;
                for j in (0..m).rev() {
                    match platform[i][j] {
                        '.' => {}
                        'O' => {
                            rounded_rocks = rounded_rocks + 1;
                            platform[i][j] = '.'
                        }
                        '#' => {
                            // Roll rocks
                            for y in (last_anchor - rounded_rocks + 1)..=last_anchor {
                                platform[i][y] = 'O'
                            }

                            // Reset anchor variabes
                            rounded_rocks = 0;
                            if j > 0 {
                                last_anchor = j - 1;
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                if rounded_rocks != 0 {
                    for y in (last_anchor - rounded_rocks + 1)..=last_anchor {
                        platform[i][y] = 'O'
                    }
                }
            }
        }
        Direction::West => {
            for i in 0..n {
                let mut rounded_rocks = 0;
                let mut last_anchor = 0;
                for j in 0..m {
                    match platform[i][j] {
                        '.' => {}
                        'O' => {
                            rounded_rocks = rounded_rocks + 1;
                            platform[i][j] = '.'
                        }
                        '#' => {
                            // Roll rocks
                            for y in last_anchor..(last_anchor + rounded_rocks) {
                                platform[i][y] = 'O'
                            }

                            // Reset anchor variabes
                            rounded_rocks = 0;
                            last_anchor = j + 1;
                        }
                        _ => unreachable!(),
                    }
                }

                if rounded_rocks != 0 {
                    for y in last_anchor..(last_anchor + rounded_rocks) {
                        platform[i][y] = 'O'
                    }
                }
            }
        }
    }
}

pub fn day_14_calcuate_total_load_on_north_support_beams(input: &str) -> usize {
    let mut platform = parse_input(input);

    tilt_platform(&mut platform, Direction::North);

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

fn run_spin_cycle<'a>(platform: &'a mut Vec<Vec<char>>) {
    tilt_platform(platform, Direction::North);
    tilt_platform(platform, Direction::West);
    tilt_platform(platform, Direction::South);
    tilt_platform(platform, Direction::East);
}

pub fn day_14_calcuate_total_load_on_north_support_beams_part_2(input: &str) -> usize {
    let mut platform = parse_input(input);

    for _ in 0..1000000000 {
        run_spin_cycle(&mut platform);
    }

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

    #[test]
    fn test_tilt() {
        let platform = parse_input(EXAMPLE);
        print_2d_matrix(&platform);

        let mut north_tilt = platform.clone();
        tilt_platform(&mut north_tilt, Direction::North);
        print_2d_matrix(&north_tilt);

        let mut south_tilt = platform.clone();
        tilt_platform(&mut south_tilt, Direction::South);
        print_2d_matrix(&south_tilt);

        let mut west_tilt = platform.clone();
        tilt_platform(&mut west_tilt, Direction::West);
        print_2d_matrix(&west_tilt);

        let mut east_tilt = platform.clone();
        tilt_platform(&mut east_tilt, Direction::East);
        print_2d_matrix(&east_tilt);
    }

    #[test]
    fn test_spin_cycle() {
        let mut platform = parse_input(EXAMPLE);

        run_spin_cycle(&mut platform);
        print_2d_matrix(&platform);
        
        run_spin_cycle(&mut platform);
        print_2d_matrix(&platform);        
        
        run_spin_cycle(&mut platform);
        print_2d_matrix(&platform);
    }

    #[test]
    fn test_day_14_calcuate_total_load_on_north_support_beams_part_2() {
        assert_eq!(
            64,
            day_14_calcuate_total_load_on_north_support_beams_part_2(EXAMPLE)
        );
    }
}
