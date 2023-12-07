use wasm_bindgen::prelude::*;

use std::cmp::Ordering;
use std::iter::zip;
use std::collections::HashMap;

static card_order: HashMap<char, usize> = Lazy::new(|| HashMap::from([
    ('2', 0),
    ('3', 1),
    ('4', 2),
    ('5', 3),
    ('6', 4),
    ('7', 5),
    ('8', 6),
    ('9', 7),
    ('T', 8),
    ('J', 9),
    ('Q', 10),
    ('K', 11),
    ('A', 12),
]));

// When derived on enums, variants are ordered by their discriminants. By default, the discriminant is smallest for variants at the top, and largest for variants at the bottom.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(PartialEq, Eq)]
struct Hand<'a> {
    cards: &'a str,
    hand_hype: HandType,
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_hype == other.hand_hype {
            for (l, r) in zip(self.cards.chars(), other.cards.chars()) {
                // TODO: Implement proper sort order for cards
                let l_value = card_order.get(&l).unwrap();
                let r_value = card_order.get(&r).unwrap();
                if l_value == r_value { continue; }
                return l_value.cmp(&r_value);
            }
        }
        self.hand_hype.cmp(&other.hand_hype)
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn determine_type(hand: &str) -> HandType {
    let mut card_counts: HashMap<char, usize> = HashMap::new();

    for char in hand.chars() {
        if let Some(count) = card_counts.get(&char) {
            card_counts.insert(char, count + 1);
        } else {
            card_counts.insert(char, 1);
        }
    }

    match card_counts.values().len() {
        1 => return HandType::FiveOfAKind,
        2 => {
            for count in card_counts.values() {
                if count == &4 || count == &1 {
                    return HandType::FourOfAKind;
                } else if count == &3 || count == &2 {
                    return HandType::FullHouse;
                }
            }
            unreachable!();
        }
        3 => {
            for count in card_counts.values() {
                if count == &3 {
                    return HandType::ThreeOfAKind;
                } else if count == &2 {
                    return HandType::TwoPair;
                }
            }
            unreachable!();
        }
        4 => return HandType::OnePair,
        5 => return HandType::HighCard,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! generate_determine_type_test {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[allow(non_snake_case)]
            fn $name() {
                let (cards, expected) = $value;
                assert_eq!(expected, determine_type(cards));
            }
        )*
        }
    }

    generate_determine_type_test! {
        test_determine_type_AAAAA: ("AAAAA", HandType::FiveOfAKind),
        test_determine_type_AA8AA: ("AA8AA", HandType::FourOfAKind),
        test_determine_type_23332: ("23332", HandType::FullHouse),
        test_determine_type_TTT98: ("TTT98", HandType::ThreeOfAKind),
        test_determine_type_23432: ("23432", HandType::TwoPair),
        test_determine_type_A23A4: ("A23A4", HandType::OnePair),
        test_determine_type_23456: ("23456", HandType::HighCard),
    }

    #[test]
    fn test_hand_sort() {
        let mut hands = ["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"].map(|cards| Hand{cards: cards, hand_hype: determine_type(cards)});
        hands.sort();
        assert_eq!(["32T3K", "KTJJT", "KK677", "T55J5", "QQQJA"], hands.map(|hand| hand.cards));
    }
}
