use std::env;
use std::fs;

use aoc_restore_snow_operations::{day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8, day_9};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1].parse::<usize>().unwrap();
    let input = &args[2];
    println!("Running day {} reading input from {}", day, input);

    let contents = fs::read_to_string(input).expect("Should have been able to read the file");

    match day {
        1 => {
            println!(
                "{}",
                day_1::day_1_get_sum_of_calibration_values_in_document(&contents)
            );
            println!(
                "{}",
                day_1::day_1_get_sum_of_calibration_values_in_document_part_2(&contents)
            );
        }
        2 => {
            println!("{}", day_2::day_2_get_sum_of_possible_game_ids(&contents));
            println!("{}", day_2::day_2_get_sum_of_minimum_power(&contents));
        }
        3 => {
            println!("{}", day_3::day_3_get_sum_of_part_numbers(&contents));
            println!("{}", day_3::day_3_get_sum_of_gear_ratios(&contents));
        }
        4 => {
            println!("{}", day_4::day_4_total_scratchcard_points(&contents));
            println!("{}", day_4::day_4_get_final_number_of_cards(&contents));
        }
        5 => {
            println!("{}", day_5::day_5_get_lowest_location(&contents));
            println!("{}", day_5::day_5_get_lowest_location_part_2(&contents));
        }
        6 => {
            println!(
                "{}",
                day_6::day_6_product_of_ways_to_beat_each_race(&contents)
            );
            println!("{}", day_6::day_6_get_number_of_ways_to_win(&contents));
        }
        7 => {
            println!("{}", day_7::day_7_calculate_total_winnings(&contents));
            println!(
                "{}",
                day_7::day_7_calculate_total_winnings_part_2(&contents)
            );
        }
        8 => {
            println!(
                "{}",
                day_8::day_8_steps_required_following_instructions(&contents)
            );
            println!(
                "{}",
                day_8::day_8_steps_required_following_instructions_part_2(&contents)
            );
        }
        9 => {
            println!("{}", day_9::day_9_sum_of_extrapolated_values(&contents));
            println!(
                "{}",
                day_9::day_9_sum_of_extrapolated_previous_values(&contents)
            );
        }
        _ => {
            println!("Day {} not implemented", day);
        }
    }
}
