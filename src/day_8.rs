use wasm_bindgen::prelude::*;

use std::collections::HashMap;

use regex::Regex;

fn parse_documents(documents: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut document_iterator = documents.split("\n").map(|line| line.trim());

    let left_right_instructions = document_iterator.next().unwrap().trim();
    document_iterator.next();

    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    let re = Regex::new(r"^([1-9A-Z]+) = \(([1-9A-Z]+), ([1-9A-Z]+)\)$").unwrap();
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

fn identify_starting_nodes<'a>(nodes: &HashMap<&'a str, (&str, &str)>) -> Vec<&'a str> {
    let mut starting_nodes: Vec<&str> = Vec::new();
    for node in nodes.keys() {
        if node.chars().nth(2).unwrap() == 'A' {
            starting_nodes.push(node);
        }
    }

    starting_nodes
}

fn is_end_node(node: &str) -> bool {
    node.chars().nth(2).unwrap() == 'Z'
}

fn to_next_end<'a>(current: (&'a str, usize), instructions: &str, n: usize, nodes: &HashMap<&'a str, (&'a str, &'a str)>) -> ((&'a str, usize), usize) {
    let (mut node, mut i) = current;

    let mut steps = 0;

    loop {
        steps = steps + 1;

        let paths = nodes.get(node).unwrap();
        let instruction = instructions.chars().nth(i).unwrap();
        node = match instruction {
            'L' => paths.0,
            'R' => paths.1,
            _ => unreachable!(),
        };

        i = i + 1;
        if i >= n {
            i = 0;
        }
        
        if is_end_node(node) {
            return ((node, i), steps);
        }


    }
}

#[wasm_bindgen]
pub fn day_8_steps_required_following_instructions_part_2(documents: &str) -> usize {
    let (instructions, nodes) = parse_documents(documents);

    let mut memo: HashMap<(&str, usize), ((&str, usize), usize)> = HashMap::new();

    let n = instructions.len();

    let mut active_nodes: Vec<(&str, usize, usize)> = identify_starting_nodes(&nodes).iter().map(|node| (*node, 0, 0)).collect();
    let m = active_nodes.len();

    let mut done = true;
    let mut max_steps = 0;
    loop {
        for i in 0..m {
            let (node, j, mut steps_taken) = active_nodes[i];

            // Let the others catch up first 
            if steps_taken >= max_steps && max_steps != 0 {
                continue;
            }

            if steps_taken != max_steps || !is_end_node(node) {
                done = false;
            }

            let ((next_node, next_j), steps_required) = match memo.get(&(node, j)) {
                Some(((next_node, next_j), steps_required)) => ((*next_node, *next_j), *steps_required),
                None => {
                    let ((next_node, next_j), steps_required) = to_next_end((node, j), instructions, n, &nodes);
                    memo.insert((node, j), ((next_node, next_j), steps_required));
                    dbg!(&memo);
                    ((next_node, next_j), steps_required)
                }
            };

            steps_taken = steps_taken + steps_required;
            active_nodes[i] = (next_node, next_j, steps_taken);

            if steps_taken > max_steps {
                max_steps = steps_taken;
            }
        }

        if done {
            return max_steps;
        }

        done = true;
    }
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

    const EXAMPLE_3: &str = r#"LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)"#;

    #[test]
    fn test_day_8_steps_required_following_instructions() {
        assert_eq!(2, day_8_steps_required_following_instructions(EXAMPLE_1));
        assert_eq!(6, day_8_steps_required_following_instructions(EXAMPLE_2));
    }

    #[test]
    fn test_identify_starting_nodes() {
        let (_, nodes) = parse_documents(EXAMPLE_3);

        let mut starting_nodes = identify_starting_nodes(&nodes);
        let mut expected = vec!["11A", "22A"];

        starting_nodes.sort();
        expected.sort();

        assert_eq!(expected, starting_nodes);
    }

    #[test]
    fn test_day_8_steps_required_following_instructions_part_2() {
        assert_eq!(6, day_8_steps_required_following_instructions_part_2(EXAMPLE_3));
    }
}
