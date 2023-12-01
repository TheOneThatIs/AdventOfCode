use std::{fs, thread::current};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    print!("{}", part1(&input));
}

fn part1(input: &str) -> i32 {
    let mut current_name = String::new();
    let disallowed_strings = vec!["ab", "cd", "pq", "xy"];
    let vowels = vec!["a", "e", "i", "o", "u"];
    let mut nice_count = 0;
    
    for character in input.chars() {
        current_name.push(character);
        if character == '\n' { 
            if !disallowed_strings.iter().any(|&s| current_name.contains(s)) { // Check if name doesn't contains disallowed strings
                let vowel_count = current_name.chars().filter(|c| match c {
                    'a' | 'e' | 'i' | 'o' | 'u' => true,
                    _ => false,
                }).count();
                if vowel_count >= 3 {
                    if current_name.chars().zip(current_name.chars().skip(1)).any(|(a, b)| a == b) {
                        nice_count += 1;
                    }
                }
            }
            
            current_name.clear();
        }
    }
    nice_count
}