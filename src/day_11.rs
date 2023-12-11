use wasm_bindgen::prelude::*;

use std::cmp::{max, min};
use std::collections::HashSet;

fn parse_image(
    image: &str,
) -> (
    (usize, usize),
    HashSet<usize>,
    HashSet<usize>,
    HashSet<(usize, usize)>,
) {
    let (mut empty_rows, mut empty_cols) = (HashSet::<usize>::new(), HashSet::<usize>::new());
    let mut galaxies = HashSet::<(usize, usize)>::new();

    let (mut n, mut m) = (0, 0);

    for (i, line) in image.split("\n").map(|line| line.trim()).enumerate() {
        if i == 0 {
            for (j, _) in line.chars().enumerate() {
                empty_cols.insert(j);
                m = j + 1;
            }
        }
        empty_rows.insert(i);
        n = i + 1;
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.insert((i, j));
                empty_rows.remove(&i);
                empty_cols.remove(&j);
            }
        }
    }

    ((n, m), empty_rows, empty_cols, galaxies)
}

fn expand_universe(
    size: (usize, usize),
    empty_rows: &HashSet<usize>,
    empty_cols: &HashSet<usize>,
    galaxies: &HashSet<(usize, usize)>,
    times: usize,
) -> HashSet<(usize, usize)> {
    let (n, m) = size;

    let mut expanded_rows = vec![0; n];
    let mut skipped_rows = 0;
    for i in 0..n {
        if empty_rows.contains(&i) {
            skipped_rows = skipped_rows + (times - 1);
        }
        expanded_rows[i] = i + skipped_rows;
    }

    let mut expanded_cols = vec![0; m];
    let mut skipped_cols = 0;
    for i in 0..m {
        if empty_cols.contains(&i) {
            skipped_cols = skipped_cols + (times - 1);
        }
        expanded_cols[i] = i + skipped_cols;
    }

    let mut expanded_galaxies = HashSet::<(usize, usize)>::new();

    for galaxy in galaxies {
        let (i, j) = *galaxy;
        expanded_galaxies.insert((expanded_rows[i], expanded_cols[j]));
    }

    expanded_galaxies
}

#[wasm_bindgen]
pub fn day_11_sum_lengths_between_galaxies(image: &str) -> usize {
    let (size, empty_rows, empty_cols, galaxies) = parse_image(image);
    let expanded_galaxies = expand_universe(size, &empty_rows, &empty_cols, &galaxies, 2)
        .iter()
        .map(|x| *x)
        .collect::<Vec<(usize, usize)>>();

    let n = expanded_galaxies.len();

    let mut acc = 0;
    for i in 0..n {
        let (x0, y0) = expanded_galaxies[i];
        for j in i + 1..n {
            let (x1, y1) = expanded_galaxies[j];

            acc = acc + (max(x1, x0) - min(x1, x0)) + (max(y1, y0) - min(y1, y0));
        }
    }

    acc
}

#[wasm_bindgen]
pub fn day_11_sum_lengths_between_galaxies_part_2(image: &str) -> usize {
    let (size, empty_rows, empty_cols, galaxies) = parse_image(image);
    let expanded_galaxies = expand_universe(size, &empty_rows, &empty_cols, &galaxies, 1000000)
        .iter()
        .map(|x| *x)
        .collect::<Vec<(usize, usize)>>();

    let n = expanded_galaxies.len();

    let mut acc = 0;
    for i in 0..n {
        let (x0, y0) = expanded_galaxies[i];
        for j in i + 1..n {
            let (x1, y1) = expanded_galaxies[j];

            acc = acc + (max(x1, x0) - min(x1, x0)) + (max(y1, y0) - min(y1, y0));
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."#;

    #[test]
    fn test_parse_image() {
        let (size, empty_rows, empty_cols, galaxies) = parse_image(EXAMPLE);

        assert_eq!((10, 10), size);
        assert_eq!(HashSet::from([3, 7]), empty_rows);
        assert_eq!(HashSet::from([2, 5, 8]), empty_cols);

        let expanded_galaxies = expand_universe(size, &empty_rows, &empty_cols, &galaxies, 2);
        let (_, _, _, parsed_expanded_galaxies) = parse_image(
            r#"....#........
        .........#...
        #............
        .............
        .............
        ........#....
        .#...........
        ............#
        .............
        .............
        .........#...
        #....#......."#,
        );
        assert_eq!(parsed_expanded_galaxies, expanded_galaxies);
    }

    #[test]
    fn test_day_11_sum_lengths_between_galaxies() {
        assert_eq!(374, day_11_sum_lengths_between_galaxies(EXAMPLE))
    }
}
