use crate::utils::print_2d_matrix;

use std::collections::HashSet;
use std::cmp::{min, max};

fn parse_dig_plan(input: &str) -> Vec<(char, usize, String)> {
    let mut dig_plan = Vec::new();

    for line in input.split("\n").map(|line| line.trim()) {
        if line == "" {continue;}

        let mut line_iterator = line.split_whitespace();
        let direction = line_iterator.next().unwrap().parse::<char>().unwrap();
        let distance = line_iterator.next().unwrap().parse::<usize>().unwrap();
        // Remove parenthesis from colour
        let mut colour_chars = line_iterator.next().unwrap().chars();
        colour_chars.next();
        colour_chars.next_back();
        let colour = colour_chars.into_iter().collect();

        dig_plan.push((direction, distance, colour));
    }

    dig_plan
}

pub fn day_18_find_lagoon_capacity(input: &str) -> usize {
    let dig_plan = parse_dig_plan(input);

    // [Top, Bottom, Left, Right]
    let mut bounds = [0; 4];
    let mut current = [0; 2];

    let mut trench = Vec::new();
    let mut trench_set: HashSet<[isize; 2]> = HashSet::from([current]);

    for (direction, distance, colour) in dig_plan {
        for _ in 0..distance {
            match direction {
                'U' => current[0] = current[0] - 1,
                'D' =>current[0] = current[0] + 1,
                'L' => current[1] = current[1] - 1,
                'R' => current[1] = current[1] + 1,
                _ => unreachable!()
            }

            trench.push((current, colour.clone()));
            trench_set.insert(current);

            bounds[0] = min(bounds[0], current[0]);
            bounds[1] = max(bounds[1], current[0]);
            bounds[2] = min(bounds[2], current[1]);
            bounds[3] = max(bounds[3], current[1]);
        }
    }

    let (n, m) = ((bounds[1]-bounds[0]+1) as usize, (bounds[3]-bounds[2]+1) as usize);
    let mut visualization: Vec<Vec<char>> = std::iter::repeat(
        std::iter::repeat('.')
            .take(m)
            .collect(),
    )
    .take(n)
    .collect();

    for [x, y] in trench_set {
        visualization[(x-bounds[0]) as usize][(y-bounds[2]) as usize] = '#';
        if x == 0 && y == 0 {
            visualization[(x-bounds[0]) as usize][(y-bounds[2]) as usize] = '*';
        }
    } 

    print_2d_matrix(&visualization);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_18_find_lagoon_capacity() {
        assert_eq!(62, day_18_find_lagoon_capacity(r#"R 6 (#70c710)
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
        U 2 (#7a21e3)"#));
    }
}

