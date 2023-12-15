use std::convert::TryInto;

fn hash_string(s: &str) -> u8 {
    #[allow(non_snake_case)]
    let ASCII_range = '\u{0}'..='\u{126}';

    let mut acc: usize = 0;

    for c in s.chars() {
        if !ASCII_range.contains(&c) {
            panic!("Non ASCII character in string");
        }

        // Determine the ASCII code for the current character of the string.
        // Increase the current value by the ASCII code you just determined.
        acc = acc + (c as usize);

        // Set the current value to itself multiplied by 17.
        acc = acc * 17;
        // Set the current value to the remainder of dividing itself by 256.
        acc = acc % 256;
    }

    acc.try_into().unwrap()
}

pub fn day_15_determine_verfication_number(initialization_sequence: &str) -> usize {
    let mut acc: usize = 0;

    for step in initialization_sequence.split(",").map(|step| step.trim()) {
        // Ignore newline characters when parsing the initialization sequence.
        if step == "\n" {
            continue;
        }
        acc = acc + (hash_string(step) as usize);
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_string() {
        assert_eq!(30, hash_string("rn=1"));
        assert_eq!(253, hash_string("cm-"));
        assert_eq!(97, hash_string("qp=3"));
        assert_eq!(47, hash_string("cm=2"));
        assert_eq!(14, hash_string("qp-"));
        assert_eq!(180, hash_string("pc=4"));
        assert_eq!(9, hash_string("ot=9"));
        assert_eq!(197, hash_string("ab=5"));
        assert_eq!(48, hash_string("pc-"));
        assert_eq!(214, hash_string("pc=6"));
        assert_eq!(231, hash_string("ot=7"));
    }

    #[test]
    fn test_day_15_determine_verfication_number() {
        assert_eq!(
            1320,
            day_15_determine_verfication_number(
                "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            )
        );
    }
}
