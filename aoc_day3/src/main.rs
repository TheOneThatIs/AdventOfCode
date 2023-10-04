use std::fs;


#[derive(Default, PartialEq, Copy, Clone)]
struct Pos {
    x: i32, y: i32 
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    
    
    println!("Part1: {} houses received at least one present.", part1(&input));
    println!("Part2: {} houses received at least one present.", part2(&input));
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

fn part2(input: &str) -> usize {
    let mut houses_visited: Vec<Pos> = Vec::new();
    let mut santa_current_pos: Pos = Default::default();
    let mut robo_current_pos: Pos = Default::default();
    let mut is_robo_santas_turn: bool = false;
    
    for input in input.chars() {
        if !is_robo_santas_turn {
            match input {
                '^' => santa_current_pos.y -= 1,
                '>' => santa_current_pos.x += 1,
                'v' => santa_current_pos.y += 1,
                '<' => santa_current_pos.x -= 1,
                _ => continue,
            }
            if !houses_visited.contains(&santa_current_pos) {
                houses_visited.push(santa_current_pos);
            }
        } else {
            match input {
                '^' => robo_current_pos.y -= 1,
                '>' => robo_current_pos.x += 1,
                'v' => robo_current_pos.y += 1,
                '<' => robo_current_pos.x -= 1,
                _ => continue,
            }
            if !houses_visited.contains(&robo_current_pos) {
                houses_visited.push(robo_current_pos);
            }
        }
        is_robo_santas_turn = !is_robo_santas_turn;
    }
    houses_visited.len()
}