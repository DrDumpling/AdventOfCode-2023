use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn get_input() -> Lines<BufReader<File>> {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}

fn is_game_valid(game_data: &str) -> bool {
    for reveiled_sets in game_data.split(';') {
        let cube_set: Vec<&str> = reveiled_sets.split(',').map(|x| x.trim()).collect();
        for cube_data in cube_set {
            // each cube data takes the form "number color"
            let (cube_amount_str, cube_color) = cube_data.split_once(' ').unwrap();
            let cube_amount: u32 = cube_amount_str.parse().unwrap();
            
            if cube_amount > match cube_color {
                "red" => 12,
                "green" => 13,
                "blue" => 14,
                x => panic!("Invalid color {}", x),
            } {
                return false;
            }
        }
    }

    true
}

fn puzzle_1() -> u32 {
    let mut sum = 0;

    for (game_index, game) in get_input().enumerate() {
        let game = game.unwrap();
        let game_data = game.split_once(':').unwrap().1;

        if is_game_valid(game_data) {
            sum += (game_index + 1) as u32;
        }
    }

    sum
}

fn get_game_minimums(game_data: &str) -> (u32, u32, u32) {
    let mut red_max = None;
    let mut green_max = None;
    let mut blue_max = None;

    for reveiled_sets in game_data.split(';') {
        let cube_set: Vec<&str> = reveiled_sets.split(',').map(|x| x.trim()).collect();
        for cube_data in cube_set {
            // each cube data takes the form "number color"
            let (cube_amount_str, cube_color) = cube_data.split_once(' ').unwrap();
            let cube_amount: u32 = cube_amount_str.parse().unwrap();
            
            match cube_color {
                "red" => if red_max.is_none() || red_max.unwrap() < cube_amount {red_max = Some(cube_amount)},
                "green" => if green_max.is_none() || green_max.unwrap() < cube_amount {green_max = Some(cube_amount)},
                "blue" => if blue_max.is_none() || blue_max.unwrap() < cube_amount {blue_max = Some(cube_amount)},
                x => panic!("Invalid color {}", x),
            }
        }
    }

    (
        *red_max.get_or_insert(0),
        *green_max.get_or_insert(0),
        *blue_max.get_or_insert(0)
    )
}

fn get_game_power(game_data: &str) -> u32 {
    let (red, green, blue) = get_game_minimums(game_data);
    red * green * blue
}

fn puzzle_2() -> u32 {
    let mut sum = 0;

    for game in get_input() {
        let game = game.unwrap();
        let game_data = game.split_once(':').unwrap().1;

        sum += get_game_power(game_data);
    }

    sum
}

fn main() {
    println!("{}", puzzle_1());
    println!("{}", puzzle_2());
}