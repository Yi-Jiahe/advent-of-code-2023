use std::env;
use std::fs;

use aoc_restore_snow_operations::{
    day_1, day_10, day_11, day_12, day_13, day_14, day_15, day_16, day_17, day_18, day_19, day_2,
    day_20, day_21, day_3, day_4, day_5, day_6, day_7, day_8, day_9,
};

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
        10 => {
            println!("{}", day_10::day_10_find_furthest_point(&contents));
            println!(
                "{}",
                day_10::day_10_find_number_of_tiles_enclosed(&contents)
            );
        }
        11 => {
            println!("{}", day_11::day_11_sum_lengths_between_galaxies(&contents));
            println!(
                "{}",
                day_11::day_11_sum_lengths_between_galaxies_part_2(&contents)
            );
        }
        12 => {
            println!("{}", day_12::day_12_sum_of_possible_arrangements(&contents));
            println!(
                "{}",
                day_12::day_12_sum_of_possible_arrangements_part_2(&contents)
            );
        }
        13 => {
            println!("{}", day_13::day_13_summarize_notes(&contents));
            println!("{}", day_13::day_13_summarize_notes_part_2(&contents));
        }
        14 => {
            println!(
                "{}",
                day_14::day_14_calcuate_total_load_on_north_support_beams(&contents)
            );
            println!(
                "{}",
                day_14::day_14_calcuate_total_load_on_north_support_beams_part_2(&contents)
            );
        }
        15 => {
            println!("{}", day_15::day_15_determine_verfication_number(&contents));
            println!(
                "{}",
                day_15::day_15_determine_resultant_focusing_power(&contents)
            );
        }
        16 => {
            println!("{}", day_16::day_16_count_energized_tiles(&contents));
            println!("{}", day_16::day_16_count_most_energized_tiles(&contents));
        }
        17 => {
            println!("{}", day_17::day_17_find_lowest_heat_loss(&contents));
            println!(
                "{}",
                day_17::day_17_find_lowest_heat_loss_for_ultra_crucible(&contents)
            );
        }
        18 => {
            println!("{}", day_18::day_18_find_lagoon_capacity(&contents));
        }
        19 => {
            println!("{}", day_19::day_19_sum_accepted_part_ratings(&contents));
            println!(
                "{}",
                day_19::day_19_number_of_combinations_of_accepted_ratings(&contents)
            );
        }
        20 => {
            println!("{}", day_20::day_20_count_pulses(&contents));
            println!("{}", day_20::day_20_count_button_presses(&contents));
        }
        21 => {
            println!("{}", day_21::day_21_part_1(&contents));
            println!("{}", day_21::day_21_part_2(&contents));
        }
        _ => {
            println!("Day {} not implemented", day);
        }
    }
}
