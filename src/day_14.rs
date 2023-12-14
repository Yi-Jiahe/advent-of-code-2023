use crate::utils::print_2d_matrix;

use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone)]
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

fn tilt_platform(platform: &Vec<Vec<char>>, direction: &Direction) -> Vec<Vec<char>> {
    let n = platform.len();
    let m = platform[0].len();

    let mut new_state = platform.clone();

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
                            new_state[i][j] = '.'
                        }
                        '#' => {
                            // Roll rocks
                            for x in last_anchor..(last_anchor + rounded_rocks) {
                                new_state[x][j] = 'O'
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
                        new_state[x][j] = 'O'
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
                            new_state[i][j] = '.'
                        }
                        '#' => {
                            // Roll rocks
                            for x in (last_anchor - rounded_rocks + 1)..=last_anchor {
                                new_state[x][j] = 'O'
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
                        new_state[x][j] = 'O'
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
                            new_state[i][j] = '.'
                        }
                        '#' => {
                            // Roll rocks
                            for y in (last_anchor - rounded_rocks + 1)..=last_anchor {
                                new_state[i][y] = 'O'
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
                        new_state[i][y] = 'O'
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
                            new_state[i][j] = '.'
                        }
                        '#' => {
                            // Roll rocks
                            for y in last_anchor..(last_anchor + rounded_rocks) {
                                new_state[i][y] = 'O'
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
                        new_state[i][y] = 'O'
                    }
                }
            }
        }
    }

    new_state
}

pub fn day_14_calcuate_total_load_on_north_support_beams(input: &str) -> usize {
    let mut platform = parse_input(input);

    let new_state = tilt_platform(&mut platform, &Direction::North);

    let mut acc = 0;
    let n = new_state.len();
    for i in 0..n {
        for c in new_state[i].iter() {
            if *c == 'O' {
                acc = acc + (n - i);
            }
        }
    }

    acc
}

fn run_spin_cycles(platform: &Vec<Vec<char>>, n: usize) -> Vec<Vec<char>> {
    let mut final_state = platform.clone();

    let mut cache_hits = 0;

    let mut memo: HashMap<(Vec<Vec<char>>, Direction), Vec<Vec<char>>> = HashMap::new();

    for _ in 0..n {
        for direction in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            final_state = if let Some(new_state) = memo.get(&(final_state.clone(), direction.clone()))
            {
                cache_hits = cache_hits + 1;
                new_state.to_vec()
            } else {
            let new_state = tilt_platform(&final_state, &direction);
                memo.insert((final_state, direction), new_state.clone());
                new_state
            }
        }
    }

    println!("Cache hit {} times out of {}", cache_hits, n*4);

    final_state
}

pub fn day_14_calcuate_total_load_on_north_support_beams_part_2(input: &str) -> usize {
    let platform = parse_input(input);

    let new_state = run_spin_cycles(&platform, 1000000000);

    let mut acc = 0;
    let n = new_state.len();
    for i in 0..n {
        for c in new_state[i].iter() {
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

        let north_tilt = tilt_platform(&platform, &Direction::North);
        print_2d_matrix(&north_tilt);

        let south_tilt = tilt_platform(&platform, &Direction::South);
        print_2d_matrix(&south_tilt);

        let east_tilt = tilt_platform(&platform, &Direction::East);
        print_2d_matrix(&east_tilt);

        let west_tilt = tilt_platform(&platform, &Direction::West);
        print_2d_matrix(&west_tilt);
    }

    #[test]
    fn test_spin_cycle() {
        let mut platform = parse_input(EXAMPLE);

        platform = run_spin_cycles(&platform, 1);
        print_2d_matrix(&platform);

        platform = run_spin_cycles(&platform, 1);
        print_2d_matrix(&platform);

        platform = run_spin_cycles(&platform, 1);
        print_2d_matrix(&platform);

        platform = run_spin_cycles(&platform, 10000);
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