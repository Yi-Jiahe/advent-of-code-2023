use std::collections::HashSet;

use crate::utils::{move_2d, parse_2d_matrix};

fn find_start(map: &Vec<Vec<char>>, shape: (usize, usize)) -> (usize, usize) {
    for i in 0..shape.0 {
        for j in 0..shape.1 {
            if map[i][j] == 'S' {
                return (i, j);
            }
        }
    }

    // Map is guaranteed to be not empty and contain a start
    unreachable!()
}

fn reachable_plots(input: &str, steps: usize) -> usize {
    let map = parse_2d_matrix(input);

    let (n, m) = (map.len(), map[0].len());

    let start = find_start(&map, (n, m));

    let mut ret = 0;

    let mut visited = HashSet::from([start]);
    let mut stack = vec![start];

    let parity = steps % 2;

    if parity == 0 {
        ret += 1;
    }

    for step in 1..=steps {
        let mut new_stack: Vec<(usize, usize)> = Vec::new();

        for position in stack {
            let adjacent_positions = [
                move_2d(position, (0, -1), (n, m)),
                move_2d(position, (0, 1), (n, m)),
                move_2d(position, (-1, 0), (n, m)),
                move_2d(position, (1, 0), (n, m)),
            ];

            for result in adjacent_positions {
                if let Ok(adjacent_position) = result {
                    if map[adjacent_position.0][adjacent_position.1] == '#' {
                        continue;
                    }

                    if visited.contains(&adjacent_position) {
                        continue;
                    }
                    visited.insert(adjacent_position);
                    new_stack.push(adjacent_position);
                    if step % 2 == parity {
                        ret += 1;
                    }
                }
            }
        }

        stack = new_stack;
    }

    ret
}

pub fn day_21_part_1(input: &str) -> usize {
    reachable_plots(input, 64)
}

fn modulo(a: isize, b: usize) -> usize {
    (((a % b as isize) + b as isize) % b as isize) as usize
}

fn reachable_plots_infinite_grid(input: &str, steps: usize) -> usize {
    let map = parse_2d_matrix(input);

    let (n, m) = (map.len(), map[0].len());

    let start = find_start(&map, (n, m));
    let start_infinite = (start.0 as isize, start.1 as isize);

    let mut ret = 0;

    let mut visited = HashSet::from([start_infinite]);
    let mut stack = vec![start_infinite];

    let parity = steps % 2;

    if parity == 0 {
        ret += 1;
    }

    for step in 1..=steps {
        let mut new_stack: Vec<(isize, isize)> = Vec::new();

        for position in stack {
            let adjacent_positions = [
                (position.0, position.1 - 1),
                (position.0, position.1 + 1),
                (position.0 - 1, position.1),
                (position.0 + 1, position.1),
            ];

            for adjacent_position in adjacent_positions {
                if map[modulo(adjacent_position.0, n)][modulo(adjacent_position.1, m)] == '#' {
                    continue;
                }

                if visited.contains(&adjacent_position) {
                    continue;
                }
                visited.insert(adjacent_position);
                new_stack.push(adjacent_position);
                if step % 2 == parity {
                    ret += 1;
                }
            }
        }
        stack = new_stack;
    }

    ret
}

pub fn day_21_part_2(input: &str) -> usize {
    reachable_plots_infinite_grid(input, 26501365)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ..........."#;

    #[test]
    fn test_reachable_plots() {
        assert_eq!(16, reachable_plots(EXAMPLE, 6));
    }

    #[test]
    // Takes a long time
    #[ignore]
    fn test_reachable_plots_infinite_grid() {
        assert_eq!(16, reachable_plots_infinite_grid(EXAMPLE, 6));
        assert_eq!(50, reachable_plots_infinite_grid(EXAMPLE, 10));
        assert_eq!(1594, reachable_plots_infinite_grid(EXAMPLE, 50));
        assert_eq!(6536, reachable_plots_infinite_grid(EXAMPLE, 100));
        assert_eq!(167004, reachable_plots_infinite_grid(EXAMPLE, 500));
        assert_eq!(668697, reachable_plots_infinite_grid(EXAMPLE, 1000));
        assert_eq!(16733044, reachable_plots_infinite_grid(EXAMPLE, 5000));
    }
}
