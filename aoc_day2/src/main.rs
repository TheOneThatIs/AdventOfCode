use std::fs::File;
use std::io::prelude::*;

fn main(){
    let mut file = File::open("files/puzzle_input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    input.push('\n');
    
    //println!("{}", get_surface_area_with_extra(input));
    println!("{}", part_2("files/puzzle_input.txt"));
}

fn get_surface_area_with_extra(input: String) -> i32 {
    let mut length: i32 = -1;
    let mut width: i32 = -1;
    let mut height: i32 = -1;
    let mut area = 0;
    
    
    let mut num = String::new();
    for (index, character) in input.chars().enumerate() {
        print!("{}", character);
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
        //print!("[{},{},{}]\n", dimensions[0], dimensions[1], dimensions[2]);
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

//fn getSurfaceAreaWithExtra(width: i32, length: i32, height: i32) -> i32 {
//    2*length*width + 2*width*height + 2*height*length + if length*width > width*height && length*width > height*length { 2*length*width } else if width*height > height*length { 2*width*height} else { 2*height*length }
//}