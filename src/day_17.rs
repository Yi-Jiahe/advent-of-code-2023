use crate::utils::{move_2d, parse_2d_matrix, print_2d_matrix};

use std::cmp::min;
use std::collections::VecDeque;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

pub fn day_17_find_lowest_heat_loss(input: &str) -> usize {
    let map = parse_2d_matrix(input);

    let n = map.len();
    let m = map[0].len();

    let mut dp: Vec<Vec<Vec<[usize; 3]>>> = std::iter::repeat(
        std::iter::repeat(std::iter::repeat([usize::MAX; 3]).take(4).collect())
            .take(m)
            .collect(),
    )
    .take(n)
    .collect();

    let mut stack = VecDeque::from([
        ((1, 0), (Direction::Down, 1), 0),
        ((0, 1), (Direction::Right, 1), 0),
    ]);

    loop {
        if let Some(((i, j), (direction, consecutive_blocks), prev_heat_loss)) = stack.pop_front() {
            let curr_heat_loss = prev_heat_loss + map[i][j].to_digit(10).unwrap() as usize;

            // If the best state on the current block is better than the current state, don't bother processing the current state
            if curr_heat_loss >= dp[i][j][direction as usize][consecutive_blocks - 1] {
                continue;
            }
            dp[i][j][direction as usize][consecutive_blocks - 1] = curr_heat_loss;

            let mut next = Vec::new();
            match direction {
                Direction::Up => {
                    next.push((
                        move_2d((i, j), (0, -1), (n, m)),
                        (Direction::Left, 1),
                        curr_heat_loss,
                    ));
                    next.push((
                        move_2d((i, j), (0, 1), (n, m)),
                        (Direction::Right, 1),
                        curr_heat_loss,
                    ));
                    if consecutive_blocks < 3 {
                        next.push((
                            move_2d((i, j), (-1, 0), (n, m)),
                            (Direction::Up, consecutive_blocks + 1),
                            curr_heat_loss,
                        ));
                    }
                }
                Direction::Down => {
                    next.push((
                        move_2d((i, j), (0, -1), (n, m)),
                        (Direction::Left, 1),
                        curr_heat_loss,
                    ));
                    next.push((
                        move_2d((i, j), (0, 1), (n, m)),
                        (Direction::Right, 1),
                        curr_heat_loss,
                    ));
                    if consecutive_blocks < 3 {
                        next.push((
                            move_2d((i, j), (1, 0), (n, m)),
                            (Direction::Down, consecutive_blocks + 1),
                            curr_heat_loss,
                        ));
                    }
                }
                Direction::Left => {
                    next.push((
                        move_2d((i, j), (-1, 0), (n, m)),
                        (Direction::Up, 1),
                        curr_heat_loss,
                    ));
                    next.push((
                        move_2d((i, j), (1, 0), (n, m)),
                        (Direction::Down, 1),
                        curr_heat_loss,
                    ));
                    if consecutive_blocks < 3 {
                        next.push((
                            move_2d((i, j), (0, -1), (n, m)),
                            (Direction::Left, consecutive_blocks + 1),
                            curr_heat_loss,
                        ));
                    }
                }
                Direction::Right => {
                    next.push((
                        move_2d((i, j), (-1, 0), (n, m)),
                        (Direction::Up, 1),
                        curr_heat_loss,
                    ));
                    next.push((
                        move_2d((i, j), (1, 0), (n, m)),
                        (Direction::Down, 1),
                        curr_heat_loss,
                    ));
                    if consecutive_blocks < 3 {
                        next.push((
                            move_2d((i, j), (0, 1), (n, m)),
                            (Direction::Right, consecutive_blocks + 1),
                            curr_heat_loss,
                        ));
                    }
                }
            }
            for (result, (direction, consecutive_blocks), heat_loss) in next.iter() {
                match result {
                    Ok((next_i, next_j)) => stack.push_back((
                        (*next_i, *next_j),
                        (*direction, *consecutive_blocks),
                        *heat_loss,
                    )),
                    Err(_) => continue,
                }
            }
        } else {
            break;
        }
    }

    let mut ans = usize::MAX;
    for heat_losses in &dp[n - 1][m - 1] {
        for heat_loss in heat_losses {
            ans = min(ans, *heat_loss)
        }
    }
    ans
}

pub fn day_17_find_lowest_heat_loss_for_ultra_crucible(input: &str) -> usize {
    let map = parse_2d_matrix(input);

    let n = map.len();
    let m = map[0].len();

    let mut dp: Vec<Vec<Vec<[usize; 10]>>> = std::iter::repeat(
        std::iter::repeat(std::iter::repeat([usize::MAX; 10]).take(4).collect())
            .take(m)
            .collect(),
    )
    .take(n)
    .collect();

    let mut stack = VecDeque::from([
        ((1, 0), (Direction::Down, 1), 0),
        ((0, 1), (Direction::Right, 1), 0),
    ]);

    loop {
        if let Some(((i, j), (direction, consecutive_blocks), prev_heat_loss)) = stack.pop_front() {
            let curr_heat_loss = prev_heat_loss + map[i][j].to_digit(10).unwrap() as usize;

            // If the best state on the current block is better than the current state, don't bother processing the current state
            if curr_heat_loss >= dp[i][j][direction as usize][consecutive_blocks - 1] {
                continue;
            }
            dp[i][j][direction as usize][consecutive_blocks - 1] = curr_heat_loss;

            let mut next = Vec::new();
            match direction {
                Direction::Up => {
                    if consecutive_blocks >= 4 {
                        next.push((
                            move_2d((i, j), (0, -1), (n, m)),
                            (Direction::Left, 1),
                            curr_heat_loss,
                        ));
                        next.push((
                            move_2d((i, j), (0, 1), (n, m)),
                            (Direction::Right, 1),
                            curr_heat_loss,
                        ));
                    }
                    if consecutive_blocks < 10 {
                        next.push((
                            move_2d((i, j), (-1, 0), (n, m)),
                            (Direction::Up, consecutive_blocks + 1),
                            curr_heat_loss,
                        ));
                    }
                }
                Direction::Down => {
                    if consecutive_blocks >= 4 {
                                  next.push((
                        move_2d((i, j), (0, -1), (n, m)),
                        (Direction::Left, 1),
                        curr_heat_loss,
                    ));
                    next.push((
                        move_2d((i, j), (0, 1), (n, m)),
                        (Direction::Right, 1),
                        curr_heat_loss,
                    ));
                    }
      
                    if consecutive_blocks < 10 {
                        next.push((
                            move_2d((i, j), (1, 0), (n, m)),
                            (Direction::Down, consecutive_blocks + 1),
                            curr_heat_loss,
                        ));
                    }
                }
                Direction::Left => {
                    if consecutive_blocks >= 4 {
                                        next.push((
                        move_2d((i, j), (-1, 0), (n, m)),
                        (Direction::Up, 1),
                        curr_heat_loss,
                    ));
                    next.push((
                        move_2d((i, j), (1, 0), (n, m)),
                        (Direction::Down, 1),
                        curr_heat_loss,
                    ));
                    }

                    if consecutive_blocks < 10 {
                        next.push((
                            move_2d((i, j), (0, -1), (n, m)),
                            (Direction::Left, consecutive_blocks + 1),
                            curr_heat_loss,
                        ));
                    }
                }
                Direction::Right => {
                    if consecutive_blocks >= 4 {
                                           next.push((
                        move_2d((i, j), (-1, 0), (n, m)),
                        (Direction::Up, 1),
                        curr_heat_loss,
                    ));
                    next.push((
                        move_2d((i, j), (1, 0), (n, m)),
                        (Direction::Down, 1),
                        curr_heat_loss,
                    ));
                    }
 
                    if consecutive_blocks < 10 {
                        next.push((
                            move_2d((i, j), (0, 1), (n, m)),
                            (Direction::Right, consecutive_blocks + 1),
                            curr_heat_loss,
                        ));
                    }
                }
            }
            for (result, (direction, consecutive_blocks), heat_loss) in next.iter() {
                match result {
                    Ok((next_i, next_j)) => stack.push_back((
                        (*next_i, *next_j),
                        (*direction, *consecutive_blocks),
                        *heat_loss,
                    )),
                    Err(_) => continue,
                }
            }
        } else {
            break;
        }
    }

    let mut ans = usize::MAX;
    for heat_losses in &dp[n - 1][m - 1] {
        for (i, heat_loss) in heat_losses.iter().enumerate() {
            if i + 1 >= 4 {
                ans = min(ans, *heat_loss);
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"2413432311323
  3215453535623
  3255245654254
  3446585845452
  4546657867536
  1438598798454
  4457876987766
  3637877979653
  4654967986887
  4564679986453
  1224686865563
  2546548887735
  4322674655533"#;

    #[test]
    fn test_day_17_find_lowest_heat_loss() {
        assert_eq!(102, day_17_find_lowest_heat_loss(EXAMPLE));
    }

    #[test]
    fn test_day_17_find_lowest_heat_loss_for_ultra_crucible() {
        assert_eq!(94, day_17_find_lowest_heat_loss_for_ultra_crucible(EXAMPLE));
        assert_eq!(71, day_17_find_lowest_heat_loss_for_ultra_crucible(r#"111111111111
        999999999991
        999999999991
        999999999991
        999999999991"#));
    }
}
