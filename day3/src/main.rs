use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use lazy_static::lazy_static;

lazy_static! {
    static ref LINE_LENGTH: usize = get_input().next().unwrap().unwrap().len();
    static ref LINE_COUNT: usize = get_input().count();
}

fn get_input() -> Lines<BufReader<File>> {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}

#[derive(Debug)]
struct Peice {
    peice_number: u32,
    position: SchematicPosition,
}

impl Peice {
    fn get_peiece_length(&self) -> usize {
        (self.peice_number.checked_ilog10().unwrap_or(0) + 1) as usize
    }

    fn is_near_symbol(&self, lines: &[String]) -> bool {
        let symbols = ['@', '#', '$', '%', '&', '*', '-', '+', '=', '/'];
        let checked_positions = self.get_surrounding_positions();

        for SchematicPosition{x_coord, y_coord} in checked_positions {
            if symbols.contains(&lines[y_coord].chars().nth(x_coord).unwrap()) {
                return true
            }
        }

        false
    }

    fn get_surrounding_positions(&self) -> Vec<SchematicPosition> {
        let mut result = Vec::new();
        //top left
        if self.position.x_coord > 0 && self.position.y_coord > 0 {
            result.push(SchematicPosition { x_coord: self.position.x_coord - 1, 
                                            y_coord: self.position.y_coord - 1 });
        }
        //top
        if self.position.y_coord > 0 {
            let my_x_coord = self.position.x_coord;
            for x_coord in my_x_coord..(my_x_coord + self.get_peiece_length()) {
                result.push(SchematicPosition { x_coord, y_coord: self.position.y_coord - 1 });
            }
        }
        //top right
        if self.position.x_coord + self.get_peiece_length() < *LINE_LENGTH && self.position.y_coord > 0 {
            result.push(SchematicPosition { x_coord: self.position.x_coord + self.get_peiece_length(), 
                                            y_coord: self.position.y_coord - 1 });
        }
        //right
        if self.position.x_coord + self.get_peiece_length() < *LINE_LENGTH {
            result.push(SchematicPosition { x_coord: self.position.x_coord + self.get_peiece_length(), 
                                            y_coord: self.position.y_coord });
        }
        //bottom right
        if self.position.x_coord + self.get_peiece_length() < *LINE_LENGTH && self.position.y_coord + 1 < *LINE_COUNT {
            result.push(SchematicPosition { x_coord: self.position.x_coord + self.get_peiece_length(), 
                                            y_coord: self.position.y_coord + 1});
        }
        //bottom
        if self.position.y_coord + 1 < *LINE_COUNT {
            let my_x_coord = self.position.x_coord;
            for x_coord in my_x_coord..(my_x_coord + self.get_peiece_length()) {
                result.push(SchematicPosition { x_coord, y_coord: self.position.y_coord + 1 });
            }
        }
        //bottom left
        if self.position.x_coord > 0 && self.position.y_coord + 1 < *LINE_COUNT {
            result.push(SchematicPosition { x_coord: self.position.x_coord - 1, 
                                            y_coord: self.position.y_coord + 1});
        }
        //left
        if self.position.x_coord > 0 {
            result.push(SchematicPosition { x_coord: self.position.x_coord - 1, 
                                            y_coord: self.position.y_coord});
        }

        result
    }

    fn get_surrounding_gears(&self, lines: &[String]) -> Vec<Gear> {
        let mut result = Vec::new();

        let checked_positions = self.get_surrounding_positions();
        for SchematicPosition{x_coord, y_coord} in checked_positions {
            if '*' == lines[y_coord].chars().nth(x_coord).unwrap() {
                result.push(Gear {held_ratios: vec!(self.peice_number), position: SchematicPosition { x_coord, y_coord }});
            }
        }

        result
    }
}

//top left is (0, 0)
#[derive(Debug)]
struct SchematicPosition {
    x_coord: usize,
    y_coord: usize,
}

impl PartialEq for SchematicPosition {
    fn eq(&self, other: &Self) -> bool {
        self.x_coord == other.x_coord && self.y_coord == other.y_coord
    }
}

fn get_piece_number(line: &str, x_coord: usize) -> u32 {
    let mut current_value = 0;
    let mut x_offset = 0;

    loop {
        let retrieved_value = line.chars().nth(x_coord + x_offset);
        if retrieved_value.is_none() {
            break;
        }
        if !retrieved_value.unwrap().is_numeric() {
            break;
        }

        current_value *= 10;
        current_value += retrieved_value.unwrap().to_digit(10).unwrap();
        x_offset += 1;
    }

    current_value
}


fn get_possible_pieces(lines: &[String]) -> Vec<Peice> {
    let mut result = Vec::new();

    for (y_coord, line) in lines.iter().enumerate() {
        let mut on_piece = false;
        for (x_coord, char) in line.chars().enumerate() {
            if !on_piece && char.is_numeric() {
                let peice_number = get_piece_number(line, x_coord);
                let peice_position: SchematicPosition = SchematicPosition{x_coord, y_coord};
                
                result.push(Peice { peice_number, position: peice_position });
            }
            on_piece = char.is_numeric();
        }
    }

    result
}

fn puzzle_1() -> u32 {
    let lines: Vec<String> = get_input().collect::<Result<_, _>>().unwrap();
    let possible_peices = get_possible_pieces(&lines);

    possible_peices
        .into_iter()
        .filter(|x| x.is_near_symbol(&lines))
        .map(|x| x.peice_number)
        .sum()
}

struct Gear {
    held_ratios: Vec<u32>,
    position: SchematicPosition,
}

fn get_possible_gears(lines: &Vec<String>) -> Vec<Gear> {
    let mut result = Vec::new();

    let possible_peices = get_possible_pieces(lines);
    for possible_peice in possible_peices {
        result.append(&mut possible_peice.get_surrounding_gears(lines))
    }

    result
}

//combine gears on the same position
fn combine_gears(gears: Vec<Gear>) -> Vec<Gear> {
    let mut result: Vec<Gear> = Vec::new();

    for added_gear in gears {
        let mut gear_found_flag = false;
        for tested_gear in &mut result {
            if tested_gear.position == added_gear.position {
                tested_gear.held_ratios.push(added_gear.held_ratios[0]);
                gear_found_flag = true;
            }
        }
        if !gear_found_flag {
            result.push(added_gear)
        }
    }

    result
}

fn puzzle_2() -> u32 {
    let lines: Vec<String> = get_input().collect::<Result<_, _>>().unwrap();
    let possible_gears = get_possible_gears(&lines);

    combine_gears(possible_gears)
        .iter()
        .filter(|x| x.held_ratios.len() >= 2)
        .map(|x| x.held_ratios.iter().product::<u32>())
        .sum()
}

fn main() {
    println!("{}", puzzle_1());
    println!("{}", puzzle_2());
}