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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reachable_plots() {
        assert_eq!(
            16,
            reachable_plots(
                r#"...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ..........."#,
                6
            )
        );
    }
}
