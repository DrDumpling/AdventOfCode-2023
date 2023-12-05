use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn get_input() -> Lines<BufReader<File>> {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}

fn puzzle_1() -> u32 {
    let mut sum = 0;
    for line in get_input() {
        let line = line.unwrap();
        let first_digit = line
            .chars()
            .find(|x: &char| x.is_numeric())
            .and_then(|c| c.to_digit(10))
            .unwrap_or(0);

        let last_digit = line
            .chars()
            .rfind(|x: &char| x.is_numeric())
            .and_then(|c| c.to_digit(10))
            .unwrap_or(0);

        sum += first_digit * 10 + last_digit;
    }

    sum
}

fn puzzle_2() -> u32 {
    let mut sum = 0;

    for line in get_input() {
        let (first_digit, last_digit) = get_line_word_values(line.unwrap());
        sum += first_digit * 10 + last_digit;
    }

    sum
}

fn get_possible_value(line: &str, indexed_value: usize) -> Option<u32> {
    let written_numbers = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    if line.chars().nth(indexed_value).unwrap().is_numeric() {
        return line.chars().nth(indexed_value).unwrap().to_digit(10);
    }

    for (word_index, written_number) in written_numbers.iter().enumerate() {
        if line[indexed_value..].starts_with(written_number) {
            return Some(word_index as u32);
        }
    }

    None
}

fn get_line_word_values(line: String) -> (u32, u32) {
    let mut start_value = None;
    let mut end_value = None;
    
    for slice_start in 0..line.len() {
        if let Some(x) = get_possible_value(&line, slice_start) {
            start_value = Some(x);
            break;
        }
    }

    for slice_start in (0..line.len()).rev() {
        if let Some(x) = get_possible_value(&line, slice_start) {
            end_value = Some(x);
            break;
        }
    }
    
    (start_value.unwrap(), end_value.unwrap())
}

fn main() {
    println!("{}", puzzle_1());
    println!("{}", puzzle_2());
}