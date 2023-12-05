use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn get_input() -> Lines<BufReader<File>> {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}

fn get_card_numbers(numbers: &str) -> Vec<u32> {
    numbers.split_whitespace().map(|x| x.parse().unwrap()).collect()
}

fn get_similar_number_count(line: &str) -> u32 {
    let card_data: &str = line.split_once(':').unwrap().1;
    let (winning_numbers, found_numbers) = card_data.split_once('|').unwrap();
    
    let winning_numbers = get_card_numbers(winning_numbers);
    let found_numbers = get_card_numbers(found_numbers);

    found_numbers.iter().filter(|x| winning_numbers.contains(x)).count() as u32
}

fn puzzle_1() -> u32 {
    let mut sum = 0;

    for line in get_input() {
        let similar_number_count = get_similar_number_count(&line.unwrap());
        if similar_number_count > 0 {
            sum += 1 << (similar_number_count - 1);
        }
    }

    sum
}

fn get_added_scratch_card_amount(index: usize, lines: &[String], cache: &mut HashMap<usize, u32>) -> u32 {
    let mut sum = 1; //count self

    if let Some(found_value) = cache.get(&index) {
        return *found_value
    }

    let added_card_amount = get_similar_number_count(&lines[index]);
    
    if added_card_amount > 0 {
        for added_card_id in index + 1..=index + added_card_amount as usize {
            sum += get_added_scratch_card_amount(added_card_id, lines, cache);
        }
    }

    cache.insert(index, sum);
    sum
}

fn puzzle_2() -> u32 {
    let mut sum = 0;

    let lines: Vec<String> = get_input().collect::<Result<_, _>>().unwrap();
    let mut processed_cards = Vec::new();

    for original_card in 0..lines.len() {
        processed_cards.push(original_card);
    }

    let mut card_cache:HashMap<usize, u32> = HashMap::new();
    while let Some(card_index) = processed_cards.pop() {
        sum += get_added_scratch_card_amount(card_index, &lines, &mut card_cache);
    }

    println!("{:?}", card_cache);

    sum
}

fn main() {
    println!("{}", puzzle_1());
    println!("{}", puzzle_2());
}