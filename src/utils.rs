use std::fmt;

#[derive(Debug)]
pub struct Move2DError {}

impl fmt::Display for Move2DError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Moving out of matrix bounds!")
    }
}

#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[allow(dead_code)]
pub fn print_2d_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

pub fn parse_2d_matrix(s: &str) -> Vec<Vec<char>> {
    let mut ret = Vec::new();

    for line in s.split("\n").map(|line| line.trim()) {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        ret.push(row);
    }
    ret
}

pub fn move_2d(
    start: (usize, usize),
    delta: (isize, isize),
    size: (usize, usize),
) -> Result<(usize, usize), Move2DError> {
    let ((i, j), (dx, dy), (n, m)) = (start, delta, size);
    let (new_x, new_y) = ((i as isize + dx), (j as isize + dy));
    if new_x < 0 || new_x as usize >= n || new_y < 0 || new_y as usize >= m {
        return Err(Move2DError {});
    }
    return Ok((new_x as usize, new_y as usize));
}
