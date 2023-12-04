use wasm_bindgen::prelude::*;

use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::iter::FromIterator;

use regex::Regex;

fn identify_card_point_value(card: &str) -> usize {
    let (_, winning_numbers, card_numbers) = parse_card(card);

    let n_matches = get_number_of_matches(winning_numbers, card_numbers);

    match n_matches {
        0 => 0,
        n => 2_usize.pow((n - 1).try_into().unwrap()),
    }
}

#[wasm_bindgen]
pub fn day_4_total_scratchcard_points(cards: &str) -> usize {
    let mut ans = 0;
    for card in cards.split("\n").map(|line| line.trim()) {
        ans = ans + identify_card_point_value(card);
    }
    ans
}

fn parse_card(card: &str) -> (usize, HashSet<&str>, HashSet<&str>) {
    let re = Regex::new(r"^Card\s+(\d+):(.*)\|(.*)$").unwrap();
    let caps = re.captures(card).unwrap();

    let n = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let l = caps.get(2).unwrap().as_str().trim();
    let r = caps.get(3).unwrap().as_str().trim();

    let winning_numbers = HashSet::<&str>::from_iter(
        l.split_whitespace()
            .map(|x| x.trim())
            .collect::<Vec<&str>>(),
    );
    let card_numbers = HashSet::<&str>::from_iter(
        r.split_whitespace()
            .map(|x| x.trim())
            .collect::<Vec<&str>>(),
    );

    (n, winning_numbers, card_numbers)
}

fn get_number_of_matches(winning_numbers: HashSet<&str>, card_numbers: HashSet<&str>) -> usize {
    card_numbers
        .intersection(&winning_numbers)
        .collect::<Vec<_>>()
        .len()
}

#[wasm_bindgen]
pub fn day_4_get_final_number_of_cards(cards: &str) -> usize {
    let mut ans = 0;

    let mut n_cards: HashMap<usize, usize> = HashMap::new();

    for card in cards.split("\n").map(|line| line.trim()) {
        let (i, winning_numbers, card_numbers) = parse_card(card);
        let n_matches = get_number_of_matches(winning_numbers, card_numbers);

        let mut n = 1;

        if let Some(extra_cards) = n_cards.remove(&i) {
            n = n + extra_cards;
        }

        for j in 1..=n_matches {
            dbg!(&j);
            if let Some(extra_cards) = n_cards.get(&(i + j)) {
                n_cards.insert(i + j, extra_cards + n);
            } else {
                n_cards.insert(i + j, n);
            }
        }

        ans = ans + n;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    macro_rules! generate_identify_card_point_value_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (card, expected) = $value;
                assert_eq!(expected, identify_card_point_value(card));
            }
        )*
        }
    }

    generate_identify_card_point_value_tests! {
        test_identify_card_point_value_1: ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8),
        test_identify_card_point_value_2: ("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2),
        test_identify_card_point_value_3: ("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2),
        test_identify_card_point_value_4: ("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1),
        test_identify_card_point_value_5: ("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0),
        test_identify_card_point_value_6: ("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0),
    }

    #[test]
    fn test_day_4_total_scratchcard_points() {
        assert_eq!(13, day_4_total_scratchcard_points(EXAMPLE));
    }

    #[test]
    fn test_day_4_get_final_number_of_cards() {
        assert_eq!(30, day_4_get_final_number_of_cards(EXAMPLE));
    }
}
