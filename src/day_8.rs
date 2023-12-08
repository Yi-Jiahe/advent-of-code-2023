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

    #[test]
    fn test_parse_documents() {
        let (left_right_instructions, nodes) = parse_documents(EXAMPLE_1);
        dbg!(left_right_instructions);
        dbg!(nodes);
        assert_eq!(1, 2);
    }
}
