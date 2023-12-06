use wasm_bindgen::prelude::*;

use std::iter::zip;

fn parse_document(document: &str) -> (Vec<usize>, Vec<usize>) {
    // Document to only contain two lines
    // First is a list of times and the second is a list of distances
    let mut document_iterator = document.split("\n").map(|line| line.trim());
    let times = document_iterator
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();
    let distances = document_iterator
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();
    (times, distances)
}

fn get_number_of_ways_to_win(time: usize, distance: usize) -> usize {
    let mut acc = 0;
    for i in 1..time {
        if (i * (time - i)) > distance {
            acc = acc + 1;
        }
    }
    acc
}

#[wasm_bindgen]
pub fn day_6_product_of_ways_to_beat_each_race(document: &str) -> usize {
    let mut acc = 1;

    let (times, distances) = parse_document(document);

    for (time, distance) in zip(times, distances) {
        acc = acc * get_number_of_ways_to_win(time, distance);
    }

    acc
}

fn parse_document_part_2(document: &str) -> (usize, usize) {
    // Document to only contain two lines
    // First is a time with a lot of spaces and the second is a distance with a lot of spaces
    let mut document_iterator = document.split("\n").map(|line| line.trim());
    let time = document_iterator
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.trim())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    let distance = document_iterator
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.trim())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<usize>()
        .unwrap();
    (time, distance)
}

#[wasm_bindgen]
pub fn day_6_get_number_of_ways_to_win(document: &str) -> usize {
    let (time, distance) = parse_document_part_2(document);

    get_number_of_ways_to_win(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DOCUMENT: &str = r#"Time:      7  15   30
    Distance:  9  40  200"#;

    #[test]
    fn test_parse_document() {
        let (times, distances) = parse_document(EXAMPLE_DOCUMENT);
        assert_eq!(vec![7, 15, 30], times);
        assert_eq!(vec![9, 40, 200], distances);
    }

    macro_rules! generate_get_number_of_ways_to_win_tests {
      ($($name:ident: $value:expr,)*) => {
      $(
          #[test]
          fn $name() {
              let ((time, distance), expected) = $value;
              assert_eq!(expected, get_number_of_ways_to_win(time, distance));
          }
      )*
      }
  }

    generate_get_number_of_ways_to_win_tests! {
        test_get_number_of_ways_to_win_7_9: ((7, 9), 4),
        test_get_number_of_ways_to_win_15_40: ((15, 40), 8),
        test_get_number_of_ways_to_win_30_200: ((30, 200), 9),
        test_get_number_of_ways_to_win_71530_940200: ((71530, 940200), 71503),
    }

    #[test]
    fn test_day_6_product_of_ways_to_beat_each_race() {
        assert_eq!(
            288,
            day_6_product_of_ways_to_beat_each_race(EXAMPLE_DOCUMENT)
        );
    }

    #[test]
    fn test_parse_document_part_2() {
        let (time, distance) = parse_document_part_2(EXAMPLE_DOCUMENT);
        assert_eq!(71530, time);
        assert_eq!(940200, distance);
    }

    #[test]
    fn test_day_6_get_number_of_ways_to_win() {
        assert_eq!(71503, day_6_get_number_of_ways_to_win(EXAMPLE_DOCUMENT));
    }
}
