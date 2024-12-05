use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_input_in_lists(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let path = Path::new(filename);
    let file = File::open(&path).expect("Failed to open file");

    let reader = io::BufReader::new(file);

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap_or(0))
            .collect();

        if numbers.len() == 2 {
            list1.push(numbers[0]);
            list2.push(numbers[1]);
        }
    }

    list1.sort();
    list2.sort();

    (list1, list2)
}

fn calculate_similarity_score(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut right_list_counts = HashMap::new();

    for &num in list2 {
        *right_list_counts.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = list1
        .iter()
        .map(|&num| num * right_list_counts.get(&num).unwrap_or(&0))
        .sum();

    similarity_score
}

fn main() {
    println!("Current directory: {:?}", std::env::current_dir());
    let (list1, list2) = get_input_in_lists("src/inputs/puzzles/input_day-1.txt");
    let diff: i32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (b - a).abs())
        .sum();

    println!("Difference score: {}", diff);

    let similarity_score = calculate_similarity_score(&list1, &list2);
    println!("Similarity score: {}", similarity_score);
}