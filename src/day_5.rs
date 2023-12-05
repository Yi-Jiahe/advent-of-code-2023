use wasm_bindgen::prelude::*;

use std::collections::HashMap;

use regex::Regex;

fn parse_almanac(
    almanac: &str,
) -> (
    Vec<usize>,
    HashMap<&str, Vec<(usize, usize, usize)>>,
    HashMap<&str, &str>,
) {
    let almanac_iterator = almanac.split("\n\n").map(|section| section.trim());

    // Consume first line which contains a listing of which seeds need to be planted
    let seeds_line = almanac_iterator.next().unwrap();
    let re = Regex::new(r"^seeds:(.*)$").unwrap();
    let caps = re.captures(seeds_line).unwrap();
    let seeds = caps
        .get(1)
        .unwrap()
        .as_str()
        .trim()
        .split_whitespace()
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();

    let mut number_map: HashMap<&str, Vec<(usize, usize, usize)>> = HashMap::new();
    let mut category_map: HashMap<&str, &str> = HashMap::new();

    // The rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category.
    let re = Regex::new(r"^(\w+)-to-(\w+) map:$").unwrap();
    for map in almanac_iterator {
        let map_iterator = map.split("\n").map(|line| line.trim());

        let category_line = map_iterator.next().unwrap();

        let caps = re.captures(category_line).unwrap();

        let l = caps.get(1).unwrap().as_str();
        let r = caps.get(2).unwrap().as_str();
        category_map.insert(l, r);

        let mappings: Vec<(usize, usize, usize)> = Vec::new();
        for mapping in map_iterator {
            let numbers = mapping
                .split_whitespace()
                .map(|x| x.trim().parse::<usize>().unwrap());
            let destination_range_start = numbers.next().unwrap();
            let source_range_start = numbers.next().unwrap();
            let range_length = numbers.next().unwrap();

            mappings.push((destination_range_start, source_range_start, range_length));
        }
        mappings.sort_by(|a, b| a.1.cmp(&b.1));
        number_map.insert(l, mappings);
    }

    (seeds, number_map, category_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_almanac() {
        let (seeds, number_map, category_map) = parse_almanac(
            r#"seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4"#,
        );

        assert_eq!(vec!([79, 14, 55, 13]), seeds);
    }
}
