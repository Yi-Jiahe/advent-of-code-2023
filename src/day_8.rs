use wasm_bindgen::prelude::*;

use std::collections::HashMap;

use regex::Regex;

fn parse_documents(documents: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut document_iterator = documents.split("\n").map(|line| line.trim());

    let left_right_instructions = document_iterator.next().unwrap().trim();
    document_iterator.next();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    let re = Regex::new(r"^([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)$").unwrap();
    for line in document_iterator {
        let caps = re.captures(line).unwrap();
        let node = caps
        .get(1)
        .unwrap()
        .as_str();
        let l = caps
        .get(2)
        .unwrap()
        .as_str();
        let r = caps
        .get(3)
        .unwrap()
        .as_str();
        nodes.insert(node, (l, r));
    }
    
    (left_right_instructions, nodes)
}

#[wasm_bindgen]
pub fn day_8_steps_required_following_instructions(documents: &str) -> usize {
    let (instructions, nodes) = parse_documents(documents);

    let mut steps = 0;
    let mut i = 0;
    let n = instructions.len();

    let mut node = "AAA";
    while node != "ZZZ" {
        steps = steps + 1;
        let instruction = instructions.chars().nth(i).unwrap();
        let paths = nodes.get(node).unwrap();
        node = match instruction {
            'L' => paths.0,
            'R' => paths.1,
            _ => unreachable!(),
        };

        i = i + 1;
        if i >= n {
            i = 0;
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)"#;

    const EXAMPLE_2: &str = r#"LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)"#;

    #[test]
    fn test_day_8_steps_required_following_instructions() {
        assert_eq!(2, day_8_steps_required_following_instructions(EXAMPLE_1));
        assert_eq!(6, day_8_steps_required_following_instructions(EXAMPLE_2));
    }
}
