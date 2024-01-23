use std::cmp::min;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug)]
struct Block {
    start: Position,
    end: Position,
}

fn parse_input(input: &str) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();

    for line in input.split("\n").map(|line| line.trim()) {
        let positions = line
            .split('~')
            .map(|position| {
                position
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        blocks.push(Block {
            start: Position {
                x: positions[0][0],
                y: positions[0][1],
                z: positions[0][2],
            },
            end: Position {
                x: positions[1][0],
                y: positions[1][1],
                z: positions[1][2],
            },
        });
    }

    blocks
}

fn drop_blocks(mut blocks: Vec<Block>) {
    // Sort blocks by their lowest cube
    blocks.sort_by(|a, b| min(a.start.z, a.end.z).cmp(&min(b.start.z, b.end.z)));
    dbg!(blocks);
}

pub fn day_22_part_1(input: &str) -> usize {
    let mut blocks = parse_input(input);
    drop_blocks(blocks);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_part_1() {
        assert_eq!(
            5,
            day_22_part_1(
                r#"1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9"#
            )
        );
    }
}
