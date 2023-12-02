#[cfg(test)]
mod day_2_tests {
    use aoc_restore_snow_operations::day_2::{
        day_2_get_sum_of_possible_game_ids,
    };

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
}