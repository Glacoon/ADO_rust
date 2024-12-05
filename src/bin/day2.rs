use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn get_line_in_tab(filename: &str) {
    let path = Path::new(filename);
    let file = File::open(&path).expect("Failed to open file");

    let reader = io::BufReader::new(file);

    let safe_reports_count = reader
        .lines()
        .filter_map(|line_result| {
            line_result.ok().map(|line| {
                let levels: Vec<i32> = line
                    .split_whitespace()
                    .map(|level_str| level_str.parse().unwrap_or(0))
                    .collect();
                println!("Tab: {:?}", levels);
                is_safe(levels)
            })
        })
        .filter(|&is_safe_report| is_safe_report)
        .count();

    println!("Count of safe lists: {}", safe_reports_count);
}

fn is_increasing(levels: &Vec<i32>) -> bool {
    let mut increasing: bool = true;
    for i in 0..levels.len() - 1 {
        if levels[i + 1] - levels[i] < 1 {
            increasing = false;
            break;
        }
    }
    increasing
}

fn check_diff(levels: &Vec<i32>) -> bool {
    for i in 0..levels.len() - 1 {
        let diff = (levels[i + 1] - levels[i]).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}

fn is_safe(list: Vec<i32>) -> bool {
    if is_increasing(&list) {
        check_diff(&list)
    } else {
        let mut reversed_list = list.clone();
        reversed_list.reverse();
        if is_increasing(&reversed_list) {
            check_diff(&reversed_list)
        } else {
            false
        }
    }
}

fn main() {
    get_line_in_tab("src/inputs/puzzles/input_day-2.txt");
}