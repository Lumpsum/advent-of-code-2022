use std::collections::HashSet;

use day_1::DayOne;
use day_2::DayTwo;
use day_3::DayThree;
use day_4::DayFour;
use day_5::DayFive;
use day_6::DaySix;
use solution::{Solution, Outcome};
use utils::*;
use reader::*;
use constants::*;

mod reader;
mod utils;
mod enums;
mod errors;
mod constants;
mod solution;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;


pub enum PuzzleResult {
    Number(Option<Outcome<usize>>),
    Text(Option<Outcome<String>>)
}


pub fn run_day_usize(day: usize) -> PuzzleResult {
    let result = match day {
        1 => DayOne::run(1),
        2 => DayTwo::run(2),
        3 => DayThree::run(3),
        4 => DayFour::run(4),
        6 => DaySix::run(6),
        _ => panic!("Day not implemented")
    };
    PuzzleResult::Number(result)
}

pub fn run_day_string(day: usize) -> PuzzleResult {
    let result = match day {
        5 => DayFive::run(5),
        _ => panic!("Day not implemented")
    };
    PuzzleResult::Text(result)
}


// pub fn day_1_puzzle_1() -> String {
//     let lines = lines_from_file(format!("{}day_1.txt", DATA_FOLDER));
//     let mut sum_lines: Vec<i32> = sum_lines(lines, "".to_string());
//     sum_lines.sort();
//     (*sum_lines.last().unwrap()).to_string()
// }


// pub fn day_1_puzzle_2() -> String {
//     let lines = lines_from_file(format!("{}day_1.txt", DATA_FOLDER));
//     let mut sum_lines: Vec<i32> = sum_lines(lines, "".to_string());
//     sum_lines.sort();
//     let sum: i32 = sum_lines.iter().rev().take(3).sum();
    
//     sum.to_string()
// }


// pub fn day_2_puzzle_1() -> String {
//     let lines = lines_from_file(format!("{}day_2.txt", DATA_FOLDER));
//     let mut total_score: i32 = 0;

//     for line in lines {
//         let values: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
//         let opponent_pick = convert_string_to_rps(values[0]).expect("Invalid string passed");
//         let my_pick = convert_string_to_rps(values[1]).expect("Invalid string passed");
//         total_score += my_pick.get_total_score(&opponent_pick);
//     }
    
//     total_score.to_string()
// }


// pub fn day_2_puzzle_2() -> String {
//     let lines = lines_from_file(format!("{}day_2.txt", DATA_FOLDER));
//     let mut total_score: i32 = 0;

//     for line in lines {
//         let values: Vec<&str> = line.split(" ").collect::<Vec<&str>>();
//         let opponent_pick = convert_string_to_rps(values[0]).expect("Invalid string passed");
//         let my_pick = opponent_pick.get_result_pick(values[1]).expect("Invalid string passed");
//         total_score += my_pick.get_total_score(&opponent_pick);
//     }

//     total_score.to_string()
// }


pub fn day_3_puzzle_1() -> String {
    let lines = lines_from_file(format!("{}day_3.txt", DATA_FOLDER));
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
        format!("{}day_3.txt", DATA_FOLDER), 3
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
    let lines = lines_from_file(format!("{}day_4.txt", DATA_FOLDER));
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
    let lines = lines_from_file(format!("{}day_4.txt", DATA_FOLDER));
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


pub fn day_5_puzzle_1() -> String {
    let lines = lines_from_file(format!("{}day_5.txt", DATA_FOLDER));
    let split_index: usize = lines.iter().position(|r| r == "").unwrap();
    let mut containers: Vec<Vec<String>> = add_container_stacks(
        lines[0].len(),
        lines[..(split_index - 1)].iter()
    );
    
    for line in &lines[(split_index + 1)..] {
        let movements = line.split_whitespace().collect::<Vec<&str>>();
        let containers_to_move = movements[1].parse::<i32>().unwrap();
        let stack_to_move_from = movements[3].parse::<usize>().unwrap() - 1;
        let stack_to_move_to = movements[5].parse::<usize>().unwrap() - 1;
        
        containers[stack_to_move_from].reverse();
        containers[stack_to_move_to].reverse();
        for _ in 0..containers_to_move {
            let value = containers[stack_to_move_from].pop().unwrap().to_string();
            containers[stack_to_move_to].push(
                value
            );
        }
        containers[stack_to_move_from].reverse();
        containers[stack_to_move_to].reverse();
    }

    get_first_element_vec_of_vecs(containers)
}


pub fn day_5_puzzle_2() -> String {
    let lines = lines_from_file(format!("{}day_5.txt", DATA_FOLDER));
    let split_index: usize = lines.iter().position(|r| r == "").unwrap();
    let mut containers: Vec<Vec<String>> = add_container_stacks(
        lines[0].len(),
        lines[..(split_index - 1)].iter()
    );
    
    for line in &lines[(split_index + 1)..] {
        let movements = line.split_whitespace().collect::<Vec<&str>>();
        let containers_to_move = movements[1].parse::<i32>().unwrap();
        let stack_to_move_from = movements[3].parse::<usize>().unwrap() - 1;
        let stack_to_move_to = movements[5].parse::<usize>().unwrap() - 1;
        
        containers[stack_to_move_from].reverse();
        let mut tmp_vec: Vec<String> = Vec::new();
        for _ in 0..containers_to_move {
            let value = containers[stack_to_move_from].pop().unwrap().to_string();
            tmp_vec.push(value);
        }
        containers[stack_to_move_from].reverse();

        tmp_vec.append(&mut containers[stack_to_move_to].clone());
        containers[stack_to_move_to] = tmp_vec;

    }

    get_first_element_vec_of_vecs(containers)
}


pub fn day_6_puzzle_1() -> String {
    let lines = lines_from_file(format!("{}day_6.txt", DATA_FOLDER));
    let marker: usize = find_first_marker(&lines[0], 4);
    marker.to_string()
}


pub fn day_6_puzzle_2() -> String {
    let lines = lines_from_file(format!("{}day_6.txt", DATA_FOLDER));
    let marker: usize = find_first_marker(&lines[0], 14);
    marker.to_string()
}
