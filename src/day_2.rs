use regex::Regex;

/// Determines if game is valid given the game and the dice loaded into the bag
/// # Arguments
///
/// * `game` String containing the game ID and sets of cubes oulled out in the game
/// * `dice_loaded` Array of values of the dice loaded into the bag in the order [red, green, blue]
///
/// # Returns
/// Array containing the game ID and if the game is valid
fn game_is_valid(game: &str, dice_loaded :&[usize]) -> (usize, bool) {
    let game_re = Regex::new(r"^Game (\d+): (.*)$").unwrap();
    let caps = game_re.captures(game).unwrap();

    let id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();

    let sets = caps.get(2).unwrap().as_str().split(";");
    let cube_res = [
        Regex::new(r"(\d+) red").unwrap(),
        Regex::new(r"(\d+) green").unwrap(),
        Regex::new(r"(\d+) blue").unwrap(),
    ];
    for set in sets {
        for (i, re) in cube_res.iter().enumerate() {
            let caps = re.captures(set);
            if caps.is_none() { continue; }
            let n = caps.unwrap().get(1).unwrap().as_str().parse::<usize>().unwrap();
            if n > dice_loaded[i] {
                return (id, false)
            }
        }
    }

    (id, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! generate_game_is_valid_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let ((game, dice_loaded), (expected_id, expected_validity)) = $value;
                let (id, valid) = game_is_valid(game, &dice_loaded);
                assert_eq!(expected_id, id);
                assert_eq!(expected_validity, valid);
            }
        )*
        }
    }

    generate_game_is_valid_tests! {
        test_game_is_valid_1: (("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", [12, 13, 14]), (1, true)),
        test_game_is_valid_2: (("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", [12, 13, 14]), (2, true)),
        test_game_is_valid_3: (("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", [12, 13, 14]), (3, false)),
        test_game_is_valid_4: (("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", [12, 13, 14]), (4, false)),
        test_game_is_valid_5: (("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", [12, 13, 14]), (5, true)),
    }
}
