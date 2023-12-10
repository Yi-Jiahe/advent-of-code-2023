use wasm_bindgen::prelude::*;

use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

fn parse_sketch(
    sketch: &str,
) -> (
    (isize, isize),
    HashMap<(isize, isize), [(isize, isize); 2]>,
    Vec<Vec<char>>,
) {
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
    for row in indexable_sketch.clone() {
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
                    let mut connected_pipes = Vec::new();
                    if i > 0 {
                        let (x, y) = (i - 1, j);
                        let up = indexable_sketch[x as usize][y as usize];
                        if up == '|' || up == '7' || up == 'F' {
                            connected_directions.push("up");
                            connected_pipes.push((x, y));
                        }
                    }
                    if i < (n - 1) as isize {
                        let (x, y) = (i + 1, j);
                        let down = indexable_sketch[x as usize][y as usize];
                        if down == '|' || down == 'L' || down == 'J' {
                            connected_directions.push("down");
                            connected_pipes.push((x, y));
                        }
                    }
                    if j > 0 {
                        let (x, y) = (i, j - 1);
                        let left = indexable_sketch[x as usize][y as usize];
                        if left == '-' || left == 'L' || left == 'F' {
                            connected_directions.push("left");

                            connected_pipes.push((x, y));
                        }
                    }
                    if j < (m - 1) as isize {
                        let (x, y) = (i, j + 1);
                        let right = indexable_sketch[x as usize][y as usize];
                        if right == '-' || right == 'J' || right == '7' {
                            connected_directions.push("right");

                            connected_pipes.push((x, y));
                        }
                    }
                    match <Vec<&str> as TryInto<[&str; 2]>>::try_into(connected_directions).unwrap()
                    {
                        ["up", "down"] => indexable_sketch[i as usize][j as usize] = '|',
                        ["up", "left"] => indexable_sketch[i as usize][j as usize] = 'J',
                        ["up", "right"] => indexable_sketch[i as usize][j as usize] = 'L',
                        ["down", "left"] => indexable_sketch[i as usize][j as usize] = '7',
                        ["down", "right"] => indexable_sketch[i as usize][j as usize] = 'F',
                        ["left", "right"] => indexable_sketch[i as usize][j as usize] = '-',
                        [&_, _] => unreachable!(),
                    }
                    pipes.insert((i, j), connected_pipes.try_into().unwrap())
                }
                _ => None,
            };
            j = j + 1;
        }
        i = i + 1;
        j = 0;
    }

    (start, pipes, indexable_sketch)
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
    let (start, pipes, _) = parse_sketch(sketch);

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

#[wasm_bindgen]
pub fn day_10_find_number_of_tiles_enclosed(sketch: &str) -> usize {
    let (start, pipes, indexable_sketch) = parse_sketch(sketch);

    let mut row_range: [isize; 2] = [isize::MAX, 0];
    let mut col_range: [isize; 2] = [isize::MAX, 0];
    let mut loop_pipes: HashSet<(isize, isize)> = HashSet::new();
    loop_pipes.insert(start);

    let mut prev: (isize, isize) = start;
    let mut curr: (isize, isize) = pipes.get(&start).unwrap()[0];

    loop {
        loop_pipes.insert(curr);

        if curr.0 < row_range[0] {
            row_range[0] = curr.0
        }
        if curr.0 > row_range[1] {
            row_range[1] = curr.0
        }
        if curr.1 < col_range[0] {
            col_range[0] = curr.1
        }
        if curr.1 > col_range[1] {
            col_range[1] = curr.1
        }

        let next = next_pipe(curr, prev, &pipes);

        if next == start {
            break;
        }

        prev = curr;
        curr = next;
    }

    let mut ans = 0;

    for i in row_range[0]..row_range[1] + 1 {
        let mut prev_pipe: Option<char> = None;
        let mut inside = false;
        for j in col_range[0]..col_range[1] + 1 {
            if loop_pipes.contains(&(i, j)) {
                let pipe = indexable_sketch[i as usize][j as usize];
                if pipe == '-' {
                    continue;
                }

                if pipe == '|' {
                    inside = !inside;
                    continue;
                }
                if let Some(p) = prev_pipe {
                    match pipe {
                        '7' => {
                            if p == 'L' {
                                inside = !inside;
                            }
                        }
                        'J' => {
                            if p == 'F' {
                                inside = !inside;
                            }
                        }
                        _ => unreachable!(),
                    }
                    prev_pipe = None;
                } else {
                    prev_pipe = Some(pipe);
                }
                continue;
            }

            if inside {
                ans = ans + 1;
            }
        }
    }

    ans
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

    #[test]
    fn test_day_10_find_number_of_tiles_enclosed() {
        assert_eq!(
            4,
            day_10_find_number_of_tiles_enclosed(
                r#"...........
                .S-------7.
                .|F-----7|.
                .||.....||.
                .||.....||.
                .|L-7.F-J|.
                .|..|.|..|.
                .L--J.L--J.
                ..........."#
            )
        );
        assert_eq!(
            4,
            day_10_find_number_of_tiles_enclosed(
                r#"...........
                .S-------7.
                .|F-----7|.
                .||OOOOO||.
                .||OOOOO||.
                .|L-7OF-J|.
                .|II|O|II|.
                .L--JOL--J.
                .....O....."#
            )
        );
        assert_eq!(
            8,
            day_10_find_number_of_tiles_enclosed(
                r#"OF----7F7F7F7F-7OOOO
              O|F--7||||||||FJOOOO
              O||OFJ||||||||L7OOOO
              FJL7L7LJLJ||LJIL-7OO
              L--JOL7IIILJS7F-7L7O
              OOOOF-JIIF7FJ|L7L7L7
              OOOOL7IF7||L7|IL7L7|
              OOOOO|FJLJ|FJ|F7|OLJ
              OOOOFJL-7O||O||||OOO
              OOOOL---JOLJOLJLJOOO"#
            )
        );
        assert_eq!(
            10,
            day_10_find_number_of_tiles_enclosed(
                r#"FF7FSF7F7F7F7F7F---7
              L|LJ||||||||||||F--J
              FL-7LJLJ||||||LJL-77
              F--JF--7||LJLJIF7FJ-
              L---JF-JLJIIIIFJLJJ7
              |F|F-JF---7IIIL7L|7|
              |FFJF7L7F-JF7IIL---7
              7-L-JL7||F7|L7F-7F7|
              L.L7LFJ|||||FJL7||LJ
              L7JLJL-JLJLJL--JLJ.L"#
            )
        );
    }
}
