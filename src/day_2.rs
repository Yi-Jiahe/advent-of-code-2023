use wasm_bindgen::prelude::*;

use regex::Regex;

/// Determines if game is valid given the game and the dice loaded into the bag
/// # Arguments
///
/// * `game` String containing the game ID and sets of cubes oulled out in the game
/// * `dice_loaded` Array of values of the dice loaded into the bag in the order [red, green, blue]
///
/// # Returns
/// Array containing the game ID and if the game is valid
fn game_is_valid(game: &str, dice_loaded: &[usize]) -> (usize, bool) {
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
            if caps.is_none() {
                continue;
            }
            let n = caps
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            if n > dice_loaded[i] {
                return (id, false);
            }
        }
    }

    (id, true)
}

/// Determines the sum of valid game ids given information about each game
/// The bag was loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes
/// # Arguments
///
/// * `games` String containing information about each game
///
/// # Returns
/// Sum of possible game ids
#[wasm_bindgen]
pub fn day_2_get_sum_of_possible_game_ids(games: &str) -> usize {
    let mut ans = 0;

    let loaded_dice = [12, 13, 14];
    for game in games.split("\n") {
        let (id, valid) = game_is_valid(game.trim(), &loaded_dice);
        if valid {
            ans = ans + id;
        }
    }
    ans
}

/// Returns the product of the minimum number of each colour of cube required for each game.
/// # Arguments
///
/// * `game` String containing the game ID and sets of cubes oulled out in the game
///
/// # Returns
/// Product of the minimum number of each colour of cube required
fn minimum_power(game: &str) -> usize {
    let game_re = Regex::new(r"^.*:(.*)$").unwrap();
    let caps = game_re.captures(game).unwrap();

    let sets = caps.get(1).unwrap().as_str().split(";");
    let cube_res = [
        Regex::new(r"(\d+) red").unwrap(),
        Regex::new(r"(\d+) green").unwrap(),
        Regex::new(r"(\d+) blue").unwrap(),
    ];
    let mut min_cubes = [0, 0, 0];
    for set in sets {
        for (i, re) in cube_res.iter().enumerate() {
            let caps = re.captures(set);
            if caps.is_none() {
                continue;
            }
            let n = caps
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap();
            if n > min_cubes[i] {
                min_cubes[i] = n
            }
        }
    }

    min_cubes[0] * min_cubes[1] * min_cubes[2]
}

/// Determines the sum of the power of minimum set of cubes that must have been present for each game given information about each game
/// # Arguments
///
/// * `games` String containing information about each game
///
/// # Returns
/// Sum of the power of minimum set of cubes that must have been present for each game
#[wasm_bindgen]
pub fn day_2_get_sum_of_minimum_power(games: &str) -> usize {
    let mut ans = 0;

    for game in games.split("\n") {
        ans = ans + minimum_power(game.trim());
    }
    ans
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

    #[test]
    fn test_day_2_get_sum_of_possible_game_ids() {
        assert_eq!(
            8,
            day_2_get_sum_of_possible_game_ids(
                r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#
            )
        );
    }

    macro_rules! generate_minimum_power_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (game, expected) = $value;
                assert_eq!(expected, minimum_power(game));
            }
        )*
        }
    }

    generate_minimum_power_tests! {
        test_minimum_power_1: ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48),
        test_minimum_power_2: ("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12),
        test_minimum_power_3: ("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560),
        test_minimum_power_4: ("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 630),
        test_minimum_power_5: ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36),
    }

    #[test]
    fn test_day_2_get_sum_of_minimum_power() {
        assert_eq!(
            2286,
            day_2_get_sum_of_minimum_power(
                r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#
            )
        );
    }
}
