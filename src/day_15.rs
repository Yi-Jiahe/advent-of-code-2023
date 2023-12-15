use std::convert::TryInto;

enum Operation {
    Remove(String),
    Set((String, u8)),
}

fn hash_string(s: &str) -> u8 {
    #[allow(non_snake_case)]
    let ASCII_range = '\u{0}'..='\u{126}';

    let mut hash: u8 = 0;

    for c in s.chars() {
        if !ASCII_range.contains(&c) {
            panic!("Non ASCII character in string");
        }

        hash = update_hash(c, hash);
    }

    hash
}

// c must be an ASCII character
fn update_hash(c: char, hash: u8) -> u8 {
    let mut acc: usize = hash as usize;

    // Determine the ASCII code for the current character of the string.
    // Increase the current value by the ASCII code you just determined.
    acc = acc + (c as usize);

    // Set the current value to itself multiplied by 17.
    acc = acc * 17;
    // Set the current value to the remainder of dividing itself by 256.
    acc = acc % 256;

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

fn parse_step(step: &str) -> (u8, Operation) {
    let mut hash: u8 = 0;
    let mut label: String = String::from("");

    let alpha_lower_range = 'a'..='z';

    let mut step_iterator = step.chars();

    let mut c: char = step_iterator.next().unwrap();

    loop {
        if alpha_lower_range.contains(&c) {
            label.push(c);

            hash = update_hash(c, hash);
        } else {
            break;
        }
        c = step_iterator.next().unwrap();
    }

    match c {
        '-' => {
            return (hash, Operation::Remove(label));
        }
        '=' => {
            let focal_length: u8 = step_iterator
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap()
                .try_into()
                .unwrap();

            return (hash, Operation::Set((label, focal_length)));
        }
        _ => unreachable!(),
    }
}

pub fn day_15_determine_resultant_focusing_power(initialization_sequence: &str) -> usize {
    let mut boxes = std::iter::repeat::<Vec<(String, u8)>>(vec![])
        .take(256)
        .collect::<Vec<_>>();

    for step in initialization_sequence.split(",").map(|step| step.trim()) {
        // Ignore newline characters when parsing the initialization sequence.
        if step == "\n" {
            continue;
        }

        let (hash, operation) = parse_step(step);

        let r#box = &mut boxes[hash as usize];
        let n = r#box.len();

        match operation {
            Operation::Remove(label) => {
                for i in 0..n {
                    if r#box[i].0 == label {
                        r#box.remove(i);
                        break;
                    }
                }
            }
            Operation::Set((label, focal_length)) => {
                let mut present = false;
                for i in 0..n {
                    if r#box[i].0 == label {
                        r#box[i] = (label.clone(), focal_length);
                        present = true;
                    }
                }
                if !present {
                    r#box.push((label.clone(), focal_length));
                }
            }
        }
    }

    let mut acc = 0;

    for (i, r#box) in boxes.iter().enumerate() {
        for (j, (_, focal_length)) in r#box.iter().enumerate() {
            acc = acc + (i + 1) * (j + 1) * (*focal_length as usize);
        }
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

    #[test]
    fn test_day_15_determine_resultant_focusing_power() {
        assert_eq!(
            145,
            day_15_determine_resultant_focusing_power(
                "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
            )
        );
    }
}
