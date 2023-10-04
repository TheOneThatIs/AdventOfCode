use std::fs;


#[derive(Default, PartialEq, Copy, Clone)]
struct Pos {
    x: i32, y: i32 
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    
    
    println!("{} houses received at least one present.", part1(&input));
}

fn part1(input: &str) -> usize {
    let mut houses_visited: Vec<(Pos, i32)> = Vec::new();
    
    let mut current_pos: Pos = Default::default();
    
    for input in input.chars() {
        match input {
            '^' => current_pos.y -= 1,
            '>' => current_pos.x += 1,
            'v' => current_pos.y += 1,
            '<' => current_pos.x -= 1,
            _ => continue,
        }
        
        match houses_visited.iter_mut().find(|(pos, _)| *pos == current_pos) {
            Some((_, count)) => *count += 1, // if current position already exists
            None => houses_visited.push((current_pos, 1)), // if the current position isn't in the vector already
        }
    }
    houses_visited.len()
}