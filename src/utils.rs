use crate::{enums::RPS, errors, constants};
use std::{result::Result, collections::{HashMap, HashSet}};


pub fn sum_lines(lines: Vec<String>, seperator: String) -> Vec<i32> {
    let mut new_value: i32 = 0;
    let mut all_values: Vec<i32> = Vec::new();

    for line in lines {
        if line == seperator {
            all_values.push(new_value);
            new_value = 0;
        } else {
            new_value += line.parse::<i32>().unwrap();
        }
    }
    all_values
}


pub fn convert_string_to_rps(value: &str) -> Result<RPS, errors::StringConversionRPSError> {
    let rps_decision = match value {
        "A" => Ok(RPS::Rock),
        "B" => Ok(RPS::Paper),
        "C" => Ok(RPS::Scissor),
        "X" => Ok(RPS::Rock),
        "Y" => Ok(RPS::Paper),
        "Z" => Ok(RPS::Scissor),
        _ => Err(errors::StringConversionRPSError)
    };

    rps_decision
}


pub fn split_string_in_half(text: String) -> (String, String) {
    let length = text.chars().count()/ 2;
    let split_text = text.split_at(length);
    (String::from(split_text.0), String::from(split_text.1))
}


pub fn get_char_score(character: String) -> i32 {
    let zipped_scores = constants::ALPHABET.iter().zip(1..=52);
    let mut hashed_zipped_scores: HashMap<String, i32> = HashMap::new();
    for score in zipped_scores {
        hashed_zipped_scores.insert(
            score.0.to_string(),
            score.1
        );
    };
    
    let score = hashed_zipped_scores.get(&character).unwrap();
    *score
}


pub fn unique_string(text: String) -> HashSet<char> {
    let mut uniques = HashSet::new();
    for char in text.chars() {
        uniques.insert(char);
    }
    uniques
}


fn find_intersection(mut first_set: HashSet<char>, second_set: &HashSet<char>) -> HashSet<char> {
    first_set.retain(|&k| second_set.contains(&k));
    first_set
}


pub fn find_strings_intersection(vec_set: Vec<HashSet<char>>) -> HashSet<char> {
    let mut result: HashSet<char> = vec_set[0].clone();
    for string in vec_set.iter().skip(1) {
        result = find_intersection(result.clone(), &string.clone());
    }
    result
}


pub fn get_intersection_scores<'a, I>(iterator: I) -> i32
where
    I: Iterator<Item = &'a char>,
{
    let mut result_score: i32 = 0;
    for value in iterator {
        result_score += get_char_score(
            value.to_string()
        );
    };
    result_score
}

pub fn split_string<'a>(text: &'a str, seperator: &str) -> Vec<&'a str> {
    let result: Vec<&str> = text.split(seperator).collect();
    result
}


pub fn range_is_subset(range_one: Vec<i32>, range_two: Vec<i32>) -> bool {
    if range_one[0] == range_two[0] { 
        return true
    } else if range_one[0] > range_two[0] {
        if range_one[1] <= range_two[1] {
            return true
        }
    } else {
        if range_one[1] >= range_two[1] {
            return true
        }
    };
    false
}


pub fn any_overlap_at_all(range_one: Vec<i32>, range_two: Vec<i32>) -> bool {
    if range_one[0] == range_two[0] || range_one[1] == range_two[1] { 
        return true
    } else if range_one[0] < range_two[0] && range_one[1] >= range_two[0] {
        return true
    } else if range_one[0] > range_two[0] && range_one[0] <= range_two[1] {
        return true
    }
    false
}


pub fn add_container_stacks<'a, I>(vec_to_create: usize, iterator: I) -> Vec<Vec<String>>
where
    I: Iterator<Item = &'a String>,
    {
    let mut containers = create_vec_of_vecs(vec_to_create);
    
    for line in iterator {
        let char_line: Vec<char> = line.chars().collect();
        for index in (1..vec_to_create).step_by(4).enumerate() {
            let character = char_line[index.1].to_string();
            if character != " ".to_string() {
                containers[index.0].push(character);
            }
        }
    }

    containers
}


fn create_vec_of_vecs<T>(vec_to_create: usize) -> Vec<Vec<T>>{
    let mut containers: Vec<Vec<T>> = Vec::new();
    
    for _ in 0..((vec_to_create + 1) / 4) {
        containers.push(Vec::new());
    }

    containers
}


pub fn get_first_element_vec_of_vecs(vec_of_vecs: Vec<Vec<String>>) -> String {
    let mut result_string = String::new();
    for vec in &vec_of_vecs[..] {
        result_string.push_str(vec.first().unwrap());
    }
    result_string
}


pub fn find_first_marker(text: &String, message_length: usize) -> usize {
    let mut result: Vec<char> = Vec::new();

    for char in text.chars().enumerate() {
        result.push(char.1);
        if char.0 < message_length {
            continue
        }

        result.reverse();
        result.pop();
        result.reverse();

        let mut unique_chars: HashSet<char> = HashSet::new();
        for value in &result {
            unique_chars.insert(value.clone());
        }

        if unique_chars.len() == message_length {
            return char.0 + 1
        }
    };
    panic!("No marker found!")
}
