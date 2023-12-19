use crate::utils::{move_2d, parse_2d_matrix};

use std::cmp::max;
use std::collections::{HashSet, VecDeque};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn count_energized_tiles(
    layout: &Vec<Vec<char>>,
    size: (usize, usize),
    start_state: ((usize, usize), Direction),
) -> usize {
    let (n, m) = size;

    let mut energized_squares: HashSet<(usize, usize)> = HashSet::new();
    let mut explored_states: HashSet<((usize, usize), Direction)> = HashSet::new();
    let mut next = VecDeque::from([start_state]);

    loop {
        if let Some(((i, j), direction)) = next.pop_front() {
            explored_states.insert(((i, j), direction));
            energized_squares.insert((i, j));

            for next_state in match (layout[i][j], direction) {
                ('.', Direction::Up) => vec![(move_2d((i, j), (-1, 0), (n, m)), Direction::Up)],
                ('.', Direction::Down) => vec![(move_2d((i, j), (1, 0), (n, m)), Direction::Down)],
                ('.', Direction::Left) => vec![(move_2d((i, j), (0, -1), (n, m)), Direction::Left)],
                ('.', Direction::Right) => {
                    vec![(move_2d((i, j), (0, 1), (n, m)), Direction::Right)]
                }
                ('/', Direction::Up) => vec![(move_2d((i, j), (0, 1), (n, m)), Direction::Right)],
                ('/', Direction::Down) => vec![(move_2d((i, j), (0, -1), (n, m)), Direction::Left)],
                ('/', Direction::Left) => vec![(move_2d((i, j), (1, 0), (n, m)), Direction::Down)],
                ('/', Direction::Right) => vec![(move_2d((i, j), (-1, 0), (n, m)), Direction::Up)],
                ('\\', Direction::Up) => vec![(move_2d((i, j), (0, -1), (n, m)), Direction::Left)],
                ('\\', Direction::Down) => {
                    vec![(move_2d((i, j), (0, 1), (n, m)), Direction::Right)]
                }
                ('\\', Direction::Left) => vec![(move_2d((i, j), (-1, 0), (n, m)), Direction::Up)],
                ('\\', Direction::Right) => {
                    vec![(move_2d((i, j), (1, 0), (n, m)), Direction::Down)]
                }
                ('|', Direction::Up) => vec![(move_2d((i, j), (-1, 0), (n, m)), Direction::Up)],
                ('|', Direction::Down) => vec![(move_2d((i, j), (1, 0), (n, m)), Direction::Down)],
                ('|', Direction::Left) => vec![
                    (move_2d((i, j), (-1, 0), (n, m)), Direction::Up),
                    (move_2d((i, j), (1, 0), (n, m)), Direction::Down),
                ],
                ('|', Direction::Right) => vec![
                    (move_2d((i, j), (-1, 0), (n, m)), Direction::Up),
                    (move_2d((i, j), (1, 0), (n, m)), Direction::Down),
                ],
                ('-', Direction::Up) => vec![
                    (move_2d((i, j), (0, -1), (n, m)), Direction::Left),
                    (move_2d((i, j), (0, 1), (n, m)), Direction::Right),
                ],
                ('-', Direction::Down) => vec![
                    (move_2d((i, j), (0, -1), (n, m)), Direction::Left),
                    (move_2d((i, j), (0, 1), (n, m)), Direction::Right),
                ],
                ('-', Direction::Left) => vec![(move_2d((i, j), (0, -1), (n, m)), Direction::Left)],
                ('-', Direction::Right) => {
                    vec![(move_2d((i, j), (0, 1), (n, m)), Direction::Right)]
                }
                (_, _) => unreachable!(),
            }
            .iter()
            {
                match next_state {
                    (Ok((i, j)), direction) => {
                        if !explored_states.contains(&((*i, *j), *direction)) {
                            next.push_back(((*i, *j), *direction));
                        }
                    }
                    (Err(_), _) => continue,
                }
            }
        } else {
            break;
        }
    }

    energized_squares.len()
}

pub fn day_16_count_energized_tiles(input: &str) -> usize {
    let layout = parse_2d_matrix(input);

    let n = layout.len();
    let m = layout[0].len();

    count_energized_tiles(&layout, (n, m), ((0, 0), Direction::Right))
}

pub fn day_16_count_most_energized_tiles(input: &str) -> usize {
    let layout = parse_2d_matrix(input);

    let n = layout.len();
    let m = layout[0].len();

    let mut max_energized_tiles = 0;

    for i in 0..n {
        max_energized_tiles = max(
            max_energized_tiles,
            count_energized_tiles(&layout, (n, m), ((i, 0), Direction::Right)),
        );
        max_energized_tiles = max(
            max_energized_tiles,
            count_energized_tiles(&layout, (n, m), ((i, m - 1), Direction::Left)),
        );
    }
    for j in 0..m {
        max_energized_tiles = max(
            max_energized_tiles,
            count_energized_tiles(&layout, (n, m), ((0, j), Direction::Down)),
        );
        max_energized_tiles = max(
            max_energized_tiles,
            count_energized_tiles(&layout, (n, m), ((n - 1, j), Direction::Up)),
        );
    }

    max_energized_tiles
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#".|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|...."#;

    #[test]
    fn test_day_16_count_energized_tiles() {
        assert_eq!(46, day_16_count_energized_tiles(EXAMPLE));
    }

    #[test]
    fn test_day_16_count_most_energized_tiles() {
        assert_eq!(51, day_16_count_most_energized_tiles(EXAMPLE));
    }
}
