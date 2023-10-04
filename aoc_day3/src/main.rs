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
    let mut houses_visited: Vec<Pos> = Vec::new();
    
    let mut current_pos: Pos = Default::default();
    
    for input in input.chars() {
        match input {
            '^' => current_pos.y -= 1,
            '>' => current_pos.x += 1,
            'v' => current_pos.y += 1,
            '<' => current_pos.x -= 1,
            _ => continue,
        }
        
        if !houses_visited.contains(&current_pos) {
            houses_visited.push(current_pos);
        }
    }
    houses_visited.len()
}