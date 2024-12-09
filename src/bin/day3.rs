use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::Rule::Do;

fn corrected_mul(filename: &str) {
    let path = Path::new(filename);
    let file = File::open(&path).expect("Failed to open file");

    let reader = io::BufReader::new(file);

    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum: i32 = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            for capture in regex.captures_iter(&line) {
                if let (Ok(x), Ok(y)) = (capture[1].parse::<i32>(), capture[2].parse::<i32>()) {
                    sum += x * y;
                }
            }
        }
    }

    println!("Sum= {}", sum);
}

#[derive(Debug)]
enum Rule {
    Mul(usize, usize),
    Do,
    Dont,
}

fn corrected_mul_part2(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let regex_mul_do_dont = Regex::new(r"do\(\)|mul\((\d+),(\d+)\)|don't\(\)").unwrap();

    let mut sum: usize = 0;
    let mut can_mul = false;

    let mut rules: Vec<Rule> = Vec::new();

    rules.push(Do);

    for line in reader.lines() {
        let line = line?;

        for capture in regex_mul_do_dont.captures_iter(&line) {
            let rule = match capture.get(0).map(|m| m.as_str()) {
                Some("do()") => Rule::Do,
                Some("don't()") => Rule::Dont,
                Some(_) => {
                    let x = capture[1].parse::<usize>().unwrap();
                    let y = capture[2].parse::<usize>().unwrap();
                    Rule::Mul(x, y)
                }
                _ => continue,
            };

            rules.push(rule);
        }
    }

    for rule in &rules {
        match rule {
            Rule::Do => can_mul = true,
            Rule::Dont => can_mul = false,
            Rule::Mul(x, y) if can_mul => sum += x * y,
            _ => {}
        }
    }

    println!("Rules: {:?}", rules);
    println!("Sum= {}", sum);
    Ok(())
}

fn main() {
    corrected_mul("src/inputs/puzzles/input_day-3.txt");
    corrected_mul_part2("src/inputs/puzzles/input_day-3.txt").expect("TODO: panic message");
}
