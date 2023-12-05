use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn get_input() -> Lines<BufReader<File>> {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}

fn puzzle_1() -> u32 {
    let mut sum = 0;

    for (game_index, game) in get_input().enumerate() {
        let game = game.unwrap();
        let game_data = game.split_once(':').unwrap().1;

        let mut valid_game = true;

        'outer: for reveiled_sets in game_data.split(";") {
            let cube_set: Vec<&str> = reveiled_sets.split(",").map(|x| x.trim()).collect();
            for cube_data in cube_set {
                let (cube_amount_str, cube_color) = cube_data.split_once(" ").unwrap();
                let cube_amount: u32 = cube_amount_str.parse().unwrap();
                
                if cube_amount > match cube_color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    x => panic!("Invalid color {}", x),
                } {
                    valid_game = false;
                    break 'outer;
                }
            }
        }

        if valid_game {
            sum += (game_index + 1) as u32;
        }
    }

    sum
}

fn main() {
    println!("{}", puzzle_1());
}