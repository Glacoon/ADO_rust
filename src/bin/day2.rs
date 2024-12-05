use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn get_line_in_tab(filename: &str) {
    let path = Path::new(filename);
    let file = File::open(&path).expect("Failed to open file");

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let list: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap_or(0))
            .collect();

        println!("Tab: {:?}", list);
    }
}

fn is_increasing(list: &Vec<i32>) -> bool {
    let mut increasing: bool = true;
    for i in 0..list.len() - 1 {
        if list[i + 1] - list[i] < 1 {
            increasing = false;
            break;
        }
    }
    increasing
}

fn is_safe(list: Vec<i32>) -> bool {
    let mut list = list;

    if !is_increasing(&list) {
        list.reverse();

        return is_increasing(&list);
    }

    true
}

fn main() {
    get_line_in_tab("src/inputs/puzzles/input_day-2.txt");
}
