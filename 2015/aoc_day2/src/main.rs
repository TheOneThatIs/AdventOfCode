use std::fs::File;
use std::io::prelude::*;
use std::time;

fn main(){
    let mut file = File::open("files/puzzle_input.txt").unwrap();
    let mut input = String::new();
    const ITERATIONS: i32 = 100;
    let mut now = time::Instant::now();
    file.read_to_string(&mut input).unwrap();
    input.push('\n');
    
    for i in 0..100 {
        part_1(&input);
    }
    let elapsed = now.elapsed().as_nanos() as f64;
    println!("Running function part_1() {} times\nTotal Time: {:?}ms\nAvg Time: {:?}", ITERATIONS, elapsed/1_000_000.0, (elapsed/ITERATIONS as f64)/1_000_000.0);
    now = time::Instant::now();
    
    for i in 0..100 {
        part_2("files/puzzle_input.txt");
    }
    let elapsed = now.elapsed().as_nanos() as f64;
    println!("Running function part_2() {} times\nTotal Time: {:?}ms\nAvg Time: {:?}", ITERATIONS, elapsed/1_000_000.0, (elapsed/ITERATIONS as f64)/1_000_000.0);
}

fn part_1(input: &str) -> i32 {
    let mut length: i32 = -1;
    let mut width: i32 = -1;
    let mut height: i32 = -1;
    let mut area = 0;
    
    
    let mut num = String::new();
    for (index, character) in input.chars().enumerate() {
        if (character == 'x' || character == '\n') && num.len() > 0 {
            if length == -1 { length = num.parse::<i32>().unwrap(); }
            else if width == -1 { width = num.parse::<i32>().unwrap(); }
            else if height == -1 { height = num.parse::<i32>().unwrap(); }
            num.clear();
        } else {
            num.push(character);
        }
        if character == '\n' {
            let tmp = std::cmp::min(length*width, width*height);
            area += std::cmp::min(tmp, height*length);
            area += 2 * (length*width + width*height + height*length);
            
            length = -1;
            width = -1;
            height = -1;
        }
    }
    area
}

fn part_2(input_filepath: &str) -> i32 {
    let mut length: i32 = -1;
    let mut width: i32 = -1;
    let mut height: i32 = -1;
    let mut cubic_area = 0;
    let mut ribbon_length_total = 0;
    
    let file = File::open(input_filepath).unwrap();
    let reader = std::io::BufReader::new(file);
    
    for line in reader.lines() {
        let tmp = line.unwrap();
        let dimensions: Vec<&str> = tmp.split('x').collect();
        length = dimensions[0].parse().unwrap();
        width = dimensions[1].parse().unwrap();
        height = dimensions[2].parse().unwrap();
        
        let mut smallest = std::cmp::min(length, width);
        smallest = std::cmp::min(smallest, height);
        
        let mut biggest = std::cmp::max(length, width);
        biggest = std::cmp::max(biggest, height);
        
        let second_smallest = length + width + height - smallest - biggest;
        let ribbon_length = (smallest * 2 + second_smallest * 2)+length*width*height;
        ribbon_length_total += ribbon_length;
    }
    return ribbon_length_total;
}