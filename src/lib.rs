use std::collections::HashSet;

use utils::*;
use reader::*;

mod reader;
mod utils;
mod enums;
mod errors;
mod constants;


pub fn day_1_puzzle_1() -> String {
    let lines = lines_from_file(format!("{}day_1.txt", constants::DATA_FOLDER));
    let mut sum_lines: Vec<i32> = sum_lines(lines, "".to_string());
    sum_lines.sort();
    (*sum_lines.last().unwrap()).to_string()
}


pub fn day_1_puzzle_2() -> String {
    let lines = lines_from_file(format!("{}day_1.txt", constants::DATA_FOLDER));
    let mut sum_lines: Vec<i32> = sum_lines(lines, "".to_string());
    sum_lines.sort();
    let sum: i32 = sum_lines.iter().rev().take(3).sum();
    
    sum.to_string()
}


pub fn day_2_puzzle_1() -> String {
    let lines = lines_from_file(format!("{}day_2.txt", constants::DATA_FOLDER));
    let mut total_score: i32 = 0;

    for line in lines {
        let values: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
        let opponent_pick = convert_string_to_rps(values[0]).expect("Invalid string passed");
        let my_pick = convert_string_to_rps(values[1]).expect("Invalid string passed");
        total_score += my_pick.get_total_score(&opponent_pick);
    }
    
    total_score.to_string()
}


pub fn day_2_puzzle_2() -> String {
    let lines = lines_from_file(format!("{}day_2.txt", constants::DATA_FOLDER));
    let mut total_score: i32 = 0;

    for line in lines {
        let values: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
        let opponent_pick = convert_string_to_rps(values[0]).expect("Invalid string passed");
        let my_pick = opponent_pick.get_result_pick(values[1]).expect("Invalid string passed");
        total_score += my_pick.get_total_score(&opponent_pick);
    }

    total_score.to_string()
}


pub fn day_3_puzzle_1() -> String {
    let lines = lines_from_file(format!("{}day_3.txt", constants::DATA_FOLDER));
    let mut total_score: i32 = 0;

    for line in lines {

        let split_string = split_string_in_half(line);
        let vec_split_string = Vec::from([
            unique_string(split_string.0), 
            unique_string(split_string.1)
        ]);

        total_score += get_intersection_scores(find_strings_intersection(vec_split_string).iter())
    }

    total_score.to_string()
}


pub fn day_3_puzzle_2() -> String {
    let group_lines = lines_from_file_in_groups(
        format!("{}day_3.txt", constants::DATA_FOLDER), 3
    );
    let mut total_score: i32 = 0;

    for group in group_lines {
        let mut vec_split_string: Vec<HashSet<char>> = Vec::new();
        for text in group {
            vec_split_string.push(unique_string(text));
        }

        total_score += get_intersection_scores(find_strings_intersection(vec_split_string).iter());   
    }
    
    total_score.to_string()
}


pub fn day_4_puzzle_1() -> String {
    let lines = lines_from_file(format!("{}day_4.txt", constants::DATA_FOLDER));
    let mut total_score: i32 = 0;

    for line in lines {
        let vec_string = split_string(&line, ",");
        let first_range = split_string(&vec_string[0], "-");
        let second_range = split_string(&vec_string[1], "-");
        if range_is_subset(
            first_range.iter().map(|&e| e.parse::<i32>().unwrap()).collect(),
            second_range.iter().map(|&e| e.parse::<i32>().unwrap()).collect()
        ) {
            total_score += 1
        }
    };
    total_score.to_string()
}


pub fn day_4_puzzle_2() -> String {
    let lines = lines_from_file(format!("{}day_4.txt", constants::DATA_FOLDER));
    let mut total_score: i32 = 0;

    for line in lines {
        let vec_string = split_string(&line, ",");
        let first_range = split_string(&vec_string[0], "-");
        let second_range = split_string(&vec_string[1], "-");
        if any_overlap_at_all(
            first_range.iter().map(|&e| e.parse::<i32>().unwrap()).collect(),
            second_range.iter().map(|&e| e.parse::<i32>().unwrap()).collect()
        ) {
            total_score += 1
        }
    };
    total_score.to_string()
}
