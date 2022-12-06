use std::{fs, io::BufRead};

type Range = (u32, u32);
type Assignment = (Range, Range);

fn parse_line(line: &str) -> Assignment {
    let ranges: Vec<Range> = line
        .split(',')
        .map(|range| {
            // Split each range on the hyphen and convert the parts to u32
            let parts: Vec<u32> = range.split('-').map(|part| part.parse().unwrap()).collect();
            // Return a tuple of the start and end of the range
            (parts[0], parts[1])
        })
        .collect();

    (ranges[0], ranges[1])
}

fn part1(assignments: &Vec<Assignment>) -> u32 {
    let mut score: u32 = 0;

    for (r1, r2) in assignments {
        if r1.0 <= r2.0 && r1.1 >= r2.1 {
            score += 1;
        } else if r2.0 <= r1.0 && r2.1 >= r1.1 {
            score += 1;
        }
    }

    score
}

fn part2(assignments: &Vec<Assignment>) -> u32 {
    let mut score: u32 = 0;

    for (r1, r2) in assignments {
        if r1.0 <= r2.1 && r2.0 <= r1.1 {
            score += 1;
        }
    }

    score
}

fn main() {
    let assignments = fs::read("src/input.txt")
        .expect("Error reading file")
        .lines()
        .map(|line| parse_line(&line.unwrap()))
        .collect::<Vec<Assignment>>();

    println!("Part 1: {}", part1(&assignments));
    println!("Part 2: {}", part2(&assignments));
}
