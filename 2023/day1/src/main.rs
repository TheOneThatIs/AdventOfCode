use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() {
    part_two();
}

fn part_one() {
    let mut file = File::open("files/puzzle_input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    
    let mut first: char = '*';
    let mut last: char = '*';
    let mut running_count: i32 = 0;
    
    for (index, character) in input.chars().enumerate() {
        if character.is_numeric() && first == '*' {
            first = character;
        }
        if character.is_numeric() {
            last = character;
        }
        
        if character == '\n' {   
            let n1 = first.to_digit(10).unwrap() as i32;
            let n2 = if last != '*' { last.to_digit(10).unwrap() as i32 } else { -1};
            let mut n: i32 = 0;
            
            if last != '*'{
                n = [n1, n2].iter().fold(0, |acc, elem| acc * 10 + elem);
            } else {
                n = n1 as i32;
            }
            
            running_count += n as i32;
            first = '*';
            last = '*';
            
            println!("N1 = {}", n);
        } 
        //print!("{}", character);
    }
    
    println!("Total: {}", running_count);
}

fn part_two() {
    let mut file = File::open("files/puzzle_input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut running_count: i32 = 0;
    let reg = Regex::new(r"1|2|3|4|5|6|7|8|9|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let lines = input.split('\n');
    for line in lines {
        println!("Current Line: {}", line);
        let first = reg.find(&line);
        let last = reg.find_iter(&line).last();
        println!("First: {}, Last: {}", first.unwrap().as_str(), last.unwrap().as_str());
        
        let first_word = first.unwrap().as_str();
        let first: i32 = match word_to_num(first_word) {
            Some(num) => num,
            None => first_word.parse::<i32>().unwrap(),
        };
        
        let last_word = last.unwrap().as_str();
        let last: i32 = match word_to_num(last_word) {
            Some(num) => num,
            None => last_word.parse::<i32>().unwrap(),
        };
        
        let concat_num = format!("{}{}", first, last);
        running_count += concat_num.parse::<i32>().unwrap();
    }
    println!("{}", running_count);
}

fn word_to_num(word: &str) -> Option<i32> {
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}