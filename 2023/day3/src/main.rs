use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part1() {
    let file = File::open("files/puzzle_input.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    for line in lines {
        let mut current_number = String::new();
        for character in line.chars() {
            if character.is_numeric() {
                current_number.push(character);
                print!("{} ", current_number);
                // Once it hits a non-number character, push current_number onto the number map
            }
        }
    }
}

fn part2() {

}

