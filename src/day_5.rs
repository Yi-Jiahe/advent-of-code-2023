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
    let mut almanac_iterator = almanac.split("\n").map(|line| line.trim());

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

    // Skip new line separator
    almanac_iterator.next();

    let mut number_map: HashMap<&str, Vec<(usize, usize, usize)>> = HashMap::new();
    let mut category_map: HashMap<&str, &str> = HashMap::new();

    // The rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category.
    let re = Regex::new(r"^(\w+)-to-(\w+) map:$").unwrap();
    let mut l = "left";
    let mut mappings: Vec<(usize, usize, usize)> = Vec::new();
    for line in almanac_iterator {
        if let Some(caps) = re.captures(line) {
            l = caps.get(1).unwrap().as_str();
            let r = caps.get(2).unwrap().as_str();
            category_map.insert(l, r);

            mappings = Vec::new();

            continue;
        }

        if line == "" {
            // Sort mappings by source_range_start
            mappings.sort_by(|a, b| a.1.cmp(&b.1));
            number_map.insert(l, mappings.clone());

            continue;
        }

        let mut numbers = line
            .split_whitespace()
            .map(|x| x.trim().parse::<usize>().unwrap());
        let destination_range_start = numbers.next().unwrap();
        let source_range_start = numbers.next().unwrap();
        let range_length = numbers.next().unwrap();
        mappings.push((destination_range_start, source_range_start, range_length));
    }
    mappings.sort_by(|a, b| a.1.cmp(&b.1));
    number_map.insert(l, mappings.clone());

    (seeds, number_map, category_map)
}

#[wasm_bindgen]
pub fn day_5_get_lowest_location(almanac: &str) -> usize {
    let (seeds, number_map, category_map) = parse_almanac(almanac);

    let mut ans = usize::MAX;
    for seed in seeds {
        let mut category = "seed";
        let mut number = seed;
        while category != "location" {
            for (destination_range_start, source_range_start, range_length) in
                number_map.get(category).unwrap()
            {
                if number < *source_range_start {
                    // Number is outside of the mappings and is mapped directly to the same number
                    break;
                }
                if number < source_range_start + range_length {
                    number = destination_range_start + (number - source_range_start);
                    break;
                }
            }
            // If the loop completes, it also means that the number is outside any of the mappings and is unchanged
            category = category_map.get(category).unwrap();
        }

        if number < ans {
            ans = number;
        }
    }

    ans
}

#[wasm_bindgen]
pub fn day_5_get_lowest_location_part_2(almanac: &str) -> usize {
    let (seeds, number_map, category_map) = parse_almanac(almanac);

    let mut seed_ranges = Vec::new();
    let mut i = 0;
    while i < seeds.len() - 1 {
        seed_ranges.push((seeds[i], seeds[i] + seeds[i + 1]));
        i = i + 2;
    }

    // Remove overlaps
    seed_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut condensed_seed_ranges = Vec::new();
    let mut prev_range = seed_ranges[0];
    for range in &seed_ranges[1..] {
        if range.0 > prev_range.1 {
            // Non-overlapping: Save the last range and continue with this one
            condensed_seed_ranges.push(prev_range);
            prev_range = *range;
            continue;
        }

        if range.1 < prev_range.1 {
            // Fully contained
            continue;
        }

        if range.0 < prev_range.1 && range.1 > prev_range.1 {
            // Overlapping: combine the two
            prev_range.1 = range.1;
            continue;
        }
    }
    condensed_seed_ranges.push(prev_range);

    let mut ans = usize::MAX;
    for seed_range in condensed_seed_ranges {
        // Source ranges. Will be updated for each category
        let mut ranges = vec![seed_range];
        let mut category = "seed";
        while category != "location" {
            // Destination ranges
            let mut new_ranges = Vec::new();
            let mappings = number_map.get(category).unwrap();
            for range in &ranges {
                let mut start = range.0;
                let end = range.1;
                for (i, (destination_range_start, source_range_start, range_length)) in mappings.iter().enumerate() {
                    // Out of the mapping, check the next mapping
                    if start >= source_range_start + range_length {
                        if i == mappings.len() - 1{
                            // Catch all if the range doesn't sit in any of the mappings
                            new_ranges.push((start, end));
                        }
                        continue;
                    }

                    if start < *source_range_start {
                        // Range starts outside the mapping
                        if end <= *source_range_start {
                            // Range is completely outside any mappings
                            new_ranges.push((start, end));
                            break;
                        }

                        // Add the bit outside the mapping
                        new_ranges.push((start, *source_range_start));

                        // Add the bit inside the mapping
                        if end <= source_range_start + range_length {
                            new_ranges.push((
                                *destination_range_start,
                                *destination_range_start + (end - source_range_start),
                            ));
                            break;
                        }

                        // Add the bit inside the mapping and continue with the rest that doesn't fit.
                        new_ranges.push((
                            *destination_range_start,
                            *destination_range_start + range_length,
                        ));

                        start = source_range_start + range_length;
                        continue;
                    }

                    // Start is inside the range
                    if end <= source_range_start + range_length {
                        new_ranges.push((
                            *destination_range_start + (start - source_range_start),
                            *destination_range_start + (end - source_range_start),
                        ));
                        break;
                    }

                    // Add the bit inside the mapping and continue with the rest that doesn't fit.
                    new_ranges.push((
                        *destination_range_start + (start - source_range_start),
                        *destination_range_start + range_length,
                    ));

                    start = source_range_start + range_length;
                    continue;
                }
   
            }
            ranges = new_ranges;
            category = category_map.get(category).unwrap();
        }

        for (start, _) in ranges {
            if start < ans {
                ans = start
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ALMANAC: &str = r#"seeds: 79 14 55 13

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
    56 93 4"#;

    #[test]
    fn test_parse_almanac() {
        let (seeds, number_map, category_map) = parse_almanac(EXAMPLE_ALMANAC);

        assert_eq!(vec![79, 14, 55, 13], seeds);
        // Tuples are ordered by the 2nd element
        assert_eq!(
            HashMap::from([
                ("seed", vec![(52, 50, 48), (50, 98, 2),]),
                ("water", vec![(88, 18, 7), (18, 25, 70)]),
                ("soil", vec![(39, 0, 15), (0, 15, 37,), (37, 52, 2,)]),
                (
                    "fertilizer",
                    vec![(42, 0, 7), (57, 7, 4), (0, 11, 42), (49, 53, 8)]
                ),
                ("light", vec![(81, 45, 19), (68, 64, 13), (45, 77, 23)]),
                ("temperature", vec![(1, 0, 69), (0, 69, 1)]),
                ("humidity", vec![(60, 56, 37), (56, 93, 4)])
            ]),
            number_map
        );
        assert_eq!(
            HashMap::from([
                ("temperature", "humidity"),
                ("seed", "soil"),
                ("fertilizer", "water"),
                ("humidity", "location"),
                ("light", "temperature"),
                ("water", "light"),
                ("soil", "fertilizer"),
            ]),
            category_map
        );
    }

    #[test]
    fn test_day_5_get_lowest_location() {
        assert_eq!(35, day_5_get_lowest_location(EXAMPLE_ALMANAC));
    }

    #[test]
    fn test_day_5_get_lowest_location_part_2() {
        assert_eq!(46, day_5_get_lowest_location_part_2(EXAMPLE_ALMANAC));
    }
}
