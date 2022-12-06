use std::{collections::HashSet, fs};

fn find_first_position(buffer: &str, len: usize) -> usize {
    let mut last_chars = Vec::new();

    for (i, c) in buffer.chars().enumerate() {
        last_chars.push(c);
        if last_chars.len() > len {
            last_chars.remove(0);
        }

        let h: HashSet<char> = last_chars.clone().into_iter().collect();
        if last_chars.len() == len && h.len() == len {
            return i + 1;
        }
    }

    0
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Unable to read file");

    let part1 = find_first_position(input.as_str(), 4);
    let part2 = find_first_position(input.as_str(), 14);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
