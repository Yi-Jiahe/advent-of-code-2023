use crate::utils::print_2d_matrix;

use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};

fn parse_dig_plan_part_1(input: &str) -> Vec<(char, usize)> {
    let mut dig_plan = Vec::new();

    for line in input.split("\n").map(|line| line.trim()) {
        if line == "" {
            continue;
        }

        let mut line_iterator = line.split_whitespace();
        let direction = line_iterator.next().unwrap().parse::<char>().unwrap();
        let distance = line_iterator.next().unwrap().parse::<usize>().unwrap();

        dig_plan.push((direction, distance));
    }

    dig_plan
}

fn find_flood_fill_seed(bounds: [isize; 4], trench_set: &HashSet<[isize; 2]>) -> [isize; 2] {
    'row_loop: for x in bounds[0]..=bounds[1] {
        for y in bounds[2]..=bounds[3] {
            // The first instance we hit the trench, check if its empty on the otherside. If so, its inside the lagoon
            if trench_set.contains(&[x, y]) && !trench_set.contains(&[x, y + 1]) {
                return [x, y + 1];
            } else {
                continue 'row_loop;
            }
        }
    }

    unreachable!();
}

fn count_interior(seed: [isize; 2], trench_set: &HashSet<[isize; 2]>) -> usize {
    let mut filled: HashSet<[isize; 2]> = HashSet::new();
    filled.extend(trench_set);

    let mut stack = VecDeque::from([seed]);

    loop {
        if let Some([x, y]) = stack.pop_front() {
            if filled.contains(&[x, y]) {
                continue;
            }
            filled.insert([x, y]);
            stack.push_back([x - 1, y]);
            stack.push_back([x + 1, y]);
            stack.push_back([x, y - 1]);
            stack.push_back([x, y + 1]);
        } else {
            break;
        }
    }

    filled.len()
}

pub fn day_18_find_lagoon_capacity(input: &str) -> usize {
    let dig_plan = parse_dig_plan_part_1(input);

    // [Top, Bottom, Left, Right]
    let mut bounds = [0; 4];
    let mut current = [0; 2];

    let mut trench_set: HashSet<[isize; 2]> = HashSet::from([current]);

    for (direction, distance) in dig_plan {
        for _ in 0..distance {
            match direction {
                'U' => current[0] = current[0] - 1,
                'D' => current[0] = current[0] + 1,
                'L' => current[1] = current[1] - 1,
                'R' => current[1] = current[1] + 1,
                _ => unreachable!(),
            }

            trench_set.insert(current);

            bounds[0] = min(bounds[0], current[0]);
            bounds[1] = max(bounds[1], current[0]);
            bounds[2] = min(bounds[2], current[1]);
            bounds[3] = max(bounds[3], current[1]);
        }
    }

    let (n, m) = (
        (bounds[1] - bounds[0] + 1) as usize,
        (bounds[3] - bounds[2] + 1) as usize,
    );
    let mut visualization: Vec<Vec<char>> =
        std::iter::repeat(std::iter::repeat('.').take(m).collect())
            .take(n)
            .collect();

    for [x, y] in &trench_set {
        visualization[(x - bounds[0]) as usize][(y - bounds[2]) as usize] = '#';
        if *x == 0 && *y == 0 {
            visualization[(x - bounds[0]) as usize][(y - bounds[2]) as usize] = '*';
        }
    }

    print_2d_matrix(&visualization);

    let seed = find_flood_fill_seed(bounds, &trench_set);

    count_interior(seed, &trench_set)
}

fn parse_dig_plan_part_2(input: &str) -> Vec<(char, usize)> {
    let mut dig_plan = Vec::new();

    for line in input.split("\n").map(|line| line.trim()) {
        if line == "" {
            continue;
        }

        let mut line_iterator = line.split_whitespace();
        let colour = line_iterator.nth(2).unwrap();

        let direction = match colour[7..=7].parse::<char>().unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => unreachable!(),
        };
        let distance_hex = &colour[2..=6];
        let mut distance = 0;

        for (i, c) in distance_hex.chars().rev().enumerate() {
            distance = distance + (c.to_digit(16).unwrap() as usize * 16_usize.pow(i as u32))
        }

        dig_plan.push((direction, distance));
    }

    dig_plan
}

pub fn day_18_find_lagoon_capacity_part_2(input: &str) -> usize {
    let dig_plan = parse_dig_plan_part_2(input);

    // [Top, Bottom, Left, Right]
    let mut bounds = [0; 4];
    let mut current = [0; 2];

    let mut trench_set: HashSet<[isize; 2]> = HashSet::from([current]);

    for (direction, distance) in dig_plan {
        for _ in 0..distance {
            match direction {
                'U' => current[0] = current[0] - 1,
                'D' => current[0] = current[0] + 1,
                'L' => current[1] = current[1] - 1,
                'R' => current[1] = current[1] + 1,
                _ => unreachable!(),
            }

            trench_set.insert(current);

            bounds[0] = min(bounds[0], current[0]);
            bounds[1] = max(bounds[1], current[0]);
            bounds[2] = min(bounds[2], current[1]);
            bounds[3] = max(bounds[3], current[1]);
        }
    }

    println!("Dug trench");

    let seed = find_flood_fill_seed(bounds, &trench_set);

    count_interior(seed, &trench_set)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_18_find_lagoon_capacity() {
        assert_eq!(
            62,
            day_18_find_lagoon_capacity(
                r#"R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"#
            )
        );
    }

    // Still figuring out a solution that can run to completion
    #[ignore]
    #[test]
    fn test_day_18_find_lagoon_capacity_part_2() {
        assert_eq!(
            952408144115,
            day_18_find_lagoon_capacity_part_2(
                r#"R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"#
            )
        );
    }
}
