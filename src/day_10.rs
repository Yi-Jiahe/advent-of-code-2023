use wasm_bindgen::prelude::*;

use std::collections::HashMap;
use std::convert::TryInto;

fn parse_sketch(sketch: &str) -> ((isize, isize), HashMap<(isize, isize), [(isize, isize); 2]>) {
    let mut indexable_sketch: Vec<Vec<char>> = Vec::new();
    for line in sketch.split("\n").map(|line| line.trim()) {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        indexable_sketch.push(row);
    }
    let n = indexable_sketch.len();
    let m = indexable_sketch[0].len();

    let mut pipes: HashMap<(isize, isize), [(isize, isize); 2]> = HashMap::new();

    let mut start: (isize, isize) = (0, 0);

    let (mut i, mut j): (isize, isize) = (0, 0);
    for row in &indexable_sketch {
        for c in row {
            match c {
                '|' => pipes.insert((i, j), [(i - 1, j), (i + 1, j)]),
                '-' => pipes.insert((i, j), [(i, j - 1), (i, j + 1)]),
                'L' => pipes.insert((i, j), [(i - 1, j), (i, j + 1)]),
                'J' => pipes.insert((i, j), [(i - 1, j), (i, j - 1)]),
                '7' => pipes.insert((i, j), [(i, j - 1), (i + 1, j)]),
                'F' => pipes.insert((i, j), [(i, j + 1), (i + 1, j)]),
                '.' => None,
                'S' => {
                    start = (i, j);

                    let mut connected_directions = Vec::new();
                    if i > 0 {
                        let (x, y) = (i - 1, j);
                        let up = indexable_sketch[x as usize][y as usize];
                        if up == '|' || up == '7' || up == 'F' {
                            connected_directions.push((x, y));
                        }
                    }
                    if i < (n - 1) as isize {
                        let (x, y) = (i + 1, j);
                        let down = indexable_sketch[x as usize][y as usize];
                        if down == '|' || down == 'L' || down == 'J' {
                            connected_directions.push((x, y));
                        }
                    }
                    if j > 0 {
                        let (x, y) = (i, j - 1);
                        let left = indexable_sketch[x as usize][y as usize];
                        if left == '-' || left == 'L' || left == 'F' {
                            connected_directions.push((x, y));
                        }
                    }
                    if j < (m - 1) as isize {
                        let (x, y) = (i, j + 1);
                        let right = indexable_sketch[x as usize][y as usize];
                        if right == '-' || right == 'J' || right == '7' {
                            connected_directions.push((x, y));
                        }
                    }
                    pipes.insert((i, j), connected_directions.try_into().unwrap())
                }
                _ => unreachable!(),
            };
            j = j + 1;
        }
        i = i + 1;
        j = 0;
    }

    (start, pipes)
}

fn next_pipe(
    curr: (isize, isize),
    prev: (isize, isize),
    pipes: &HashMap<(isize, isize), [(isize, isize); 2]>,
) -> (isize, isize) {
    for pipe in pipes.get(&curr).unwrap() {
        if *pipe != prev {
            return *pipe;
        }
    }
    unreachable!();
}

#[wasm_bindgen]
pub fn day_10_find_furthest_point(sketch: &str) -> usize {
    let (start, pipes) = parse_sketch(sketch);

    let mut loop_length = 1;
    let mut prev: (isize, isize) = start;
    let mut curr: (isize, isize) = pipes.get(&start).unwrap()[0];

    loop {
        let next = next_pipe(curr, prev, &pipes);

        if next == start {
            if loop_length % 2 == 0 {
                return loop_length / 2;
            } else {
                return (loop_length + 1) / 2;
            }
        }

        loop_length = loop_length + 1;
        prev = curr;
        curr = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_10_find_furthest_point() {
        assert_eq!(
            4,
            day_10_find_furthest_point(
                r#".....
    .S-7.
    .|.|.
    .L-J.
    ....."#
            )
        );
        assert_eq!(
            8,
            day_10_find_furthest_point(
                r#"..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ..."#
            )
        );
    }
}
