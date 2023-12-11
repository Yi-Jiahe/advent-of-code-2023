use std::collections::HashSet;

fn parse_galaxy(
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
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
    galaxies: HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let (n, m) = size;

    let mut expanded_rows = vec![0; n];
    let mut skipped_rows = 0;
    for i in 0..n {
        if empty_rows.contains(&i) {
            skipped_rows = skipped_rows + 1;
        }
        expanded_rows[i] = i + skipped_rows;
    }

    dbg!(&expanded_rows);

    let mut expanded_cols = vec![0; m];
    let mut skipped_cols = 0;
    for i in 0..m {
        if empty_cols.contains(&i) {
            skipped_cols = skipped_cols + 1;
        }
        expanded_cols[i] = i + skipped_cols;
    }

    dbg!(&expanded_cols);

    let mut expanded_galaxies = HashSet::<(usize, usize)>::new();

    for galaxy in galaxies {
        let (i, j) = galaxy;
        expanded_galaxies.insert((expanded_rows[i], expanded_cols[j]));
    }

    expanded_galaxies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_galaxy() {
        let (size, empty_rows, empty_cols, galaxies) = parse_galaxy(
            r#"...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."#,
        );

        assert_eq!((10, 10), size);
        assert_eq!(HashSet::from([3, 7]), empty_rows);
        assert_eq!(HashSet::from([2, 5, 8]), empty_cols);

        let expanded_galaxies = expand_universe(size, empty_rows, empty_cols, galaxies);
        let (_, _, _, parsed_expanded_galaxies) = parse_galaxy(
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
}
