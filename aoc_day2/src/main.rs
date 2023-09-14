use std::fs::File;
use std::io::prelude::*;

fn main(){
    let mut file = File::open("files/puzzle_input.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    input.push('\n');
    
    println!("{}", get_surface_area_with_extra(input));
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

//fn getSurfaceAreaWithExtra(width: i32, length: i32, height: i32) -> i32 {
//    2*length*width + 2*width*height + 2*height*length + if length*width > width*height && length*width > height*length { 2*length*width } else if width*height > height*length { 2*width*height} else { 2*height*length }
//}