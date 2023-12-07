pub use part_1::day_7_calculate_total_winnings;
pub use part_2::day_7_calculate_total_winnings_part_2;

// When derived on enums, variants are ordered by their discriminants. By default, the discriminant is smallest for variants at the top, and largest for variants at the bottom.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub mod part_1 {
    use wasm_bindgen::prelude::*;

    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::iter::zip;

    use crate::day_7::HandType;

    const CARD_ORDER: [char; 13] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    #[derive(Debug, PartialEq, Eq)]
    struct Hand<'a> {
        cards: &'a str,
        hand_hype: HandType,
    }

    impl Ord for Hand<'_> {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.hand_hype == other.hand_hype {
                for (l, r) in zip(self.cards.chars(), other.cards.chars()) {
                    if l == r {
                        continue;
                    }
                    return CARD_ORDER
                        .iter()
                        .position(|c| c == &l)
                        .cmp(&CARD_ORDER.iter().position(|c| c == &r));
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

    #[wasm_bindgen]
    pub fn day_7_calculate_total_winnings(hands: &str) -> usize {
        let mut acc = 0;
        let mut hands_and_bids: Vec<(Hand, usize)> = hands
            .split("\n")
            .map(|line| {
                let mut line_iterator = line.trim().split_whitespace().map(|x| x.trim());
                let cards = line_iterator.next().unwrap();
                let bid = line_iterator.next().unwrap().parse::<usize>().unwrap();
                (
                    Hand {
                        cards: cards,
                        hand_hype: determine_type(cards),
                    },
                    bid,
                )
            })
            .collect();
        hands_and_bids.sort_by(|a, b| a.0.cmp(&b.0));

        for (i, (_, bid)) in hands_and_bids.iter().enumerate() {
            acc = acc + ((i + 1) * bid);
        }

        return acc;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const EXAMPLE: &str = r#"32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483"#;

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
            let mut hands = ["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"].map(|cards| Hand {
                cards: cards,
                hand_hype: determine_type(cards),
            });
            hands.sort();
            assert_eq!(
                ["32T3K", "KTJJT", "KK677", "T55J5", "QQQJA"],
                hands.map(|hand| hand.cards)
            );
        }

        #[test]
        fn test_day_7_calculate_total_winnings() {
            assert_eq!(6440, day_7_calculate_total_winnings(EXAMPLE));
        }
    }
}

pub mod part_2 {
    use wasm_bindgen::prelude::*;

    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::iter::zip;

    use crate::day_7::HandType;

    const CARD_ORDER: [char; 13] = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];

    #[derive(Debug, PartialEq, Eq)]
    struct Hand<'a> {
        cards: &'a str,
        hand_hype: HandType,
    }

    impl Ord for Hand<'_> {
        fn cmp(&self, other: &Self) -> Ordering {
            if self.hand_hype == other.hand_hype {
                for (l, r) in zip(self.cards.chars(), other.cards.chars()) {
                    if l == r {
                        continue;
                    }
                    return CARD_ORDER
                        .iter()
                        .position(|c| c == &l)
                        .cmp(&CARD_ORDER.iter().position(|c| c == &r));
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

        let distinct_values = card_counts.values().len();

        // Do the normal tests if no J is involved
        if !card_counts.contains_key(&'J') {
            match distinct_values {
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

        match distinct_values {
            // Joker will become the other
            1 | 2 => return HandType::FiveOfAKind,
            3 => {
                if !card_counts.values().find(|&&x| x == 3).is_none() {
                    // If there's a 3 the Joker will make a 4 of a kind either way
                    return HandType::FourOfAKind;
                }

                // The remaining case is a Two Pair
                let count = card_counts.get(&'J').unwrap();
                if count == &2 {
                    // If the J is the pair, the best case is a Four of a kind
                    return HandType::FourOfAKind;
                }

                // If its not, then the best is a Full house
                return HandType::FullHouse;
            }
            // If the J not the pair, it will combine with the pair because its better than a Two pair
            // If it is the pair, it can only combine with one of the rest to make a Three of a kind
            4 => return HandType::ThreeOfAKind,
            // The worst you can get is a One Pair because the J will pair off with anything
            5 => return HandType::OnePair,
            _ => unreachable!(),
        }
    }

    #[wasm_bindgen]
    pub fn day_7_calculate_total_winnings_part_2(hands: &str) -> usize {
        let mut acc = 0;
        let mut hands_and_bids: Vec<(Hand, usize)> = hands
            .split("\n")
            .map(|line| {
                let mut line_iterator = line.trim().split_whitespace().map(|x| x.trim());
                let cards = line_iterator.next().unwrap();
                let bid = line_iterator.next().unwrap().parse::<usize>().unwrap();
                (
                    Hand {
                        cards: cards,
                        hand_hype: determine_type(cards),
                    },
                    bid,
                )
            })
            .collect();
        hands_and_bids.sort_by(|a, b| a.0.cmp(&b.0));

        for (i, (_, bid)) in hands_and_bids.iter().enumerate() {
            acc = acc + ((i + 1) * bid);
        }

        return acc;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const EXAMPLE: &str = r#"32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"#;

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
            // With Js
            // Example
            test_determine_type_T55J5: ("T55J5", HandType::FourOfAKind),
            test_determine_type_KTJJT: ("KTJJT", HandType::FourOfAKind),
            test_determine_type_QQQJA: ("QQQJA", HandType::FourOfAKind),
            // Full house all Js
            test_determine_type_JJJJJ: ("JJJJJ", HandType::FiveOfAKind),
            // Originally Four of a kind
            test_determine_type_AAJAA: ("AAJAA", HandType::FiveOfAKind),
            test_determine_type_JJJAJ: ("JJJAJ", HandType::FiveOfAKind),
            // Originally Full house
            test_determine_type_AAJAJ: ("AAJAJ", HandType::FiveOfAKind),
            test_determine_type_JAJAJ: ("JAJAJ", HandType::FiveOfAKind),
            // Originally Three of a kind
            test_determine_type_JJAJ2: ("JJAJ2", HandType::FourOfAKind),
            test_determine_type_AJAA3: ("AJAA3", HandType::FourOfAKind),
            // Originally Two Pair
            test_determine_type_JJAA3: ("JJAA3", HandType::FourOfAKind),
            test_determine_type_22AAJ: ("22AAJ", HandType::FullHouse),
        }

        #[test]
        fn test_hand_sort() {
            let mut hands = ["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA"].map(|cards| Hand {
                cards: cards,
                hand_hype: determine_type(cards),
            });
            hands.sort();
            assert_eq!(
                ["32T3K", "KK677", "T55J5", "QQQJA", "KTJJT"],
                hands.map(|hand| hand.cards)
            );
        }

        #[test]
        fn test_day_7_calculate_total_winnings_part_2() {
            assert_eq!(5905, day_7_calculate_total_winnings_part_2(EXAMPLE));
        }
    }
}
