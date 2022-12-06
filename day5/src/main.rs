use std::{fs, io::BufRead};

use regex::Regex;

type Crate = char;
type Stack = Vec<Crate>;

#[derive(Clone, Debug)]
struct Move {
    pub quantity: u32,
    pub from: u32,
    pub to: u32,
}

impl Move {
    pub fn new(quantity: u32, from: u32, to: u32) -> Move {
        Move { quantity, from, to }
    }
}

fn parse_stacks(lines: &Vec<String>, num_stacks: usize) -> Vec<Stack> {
    let mut stacks: Vec<Stack> = vec![Vec::new(); num_stacks];

    for line in lines {
        if line.trim() == "" {
            break;
        }

        // Loop through each character in the line
        let mut s: usize = 0;
        for i in (0..line.len()).step_by(4) {
            s += 1;
            let c = line.chars().nth(i + 1).unwrap();

            if c == ' ' || !c.is_alphabetic() {
                continue;
            }

            // Push the crate onto the stack
            stacks[s - 1].push(c);
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    stacks
}

fn parse_moves(lines: &Vec<String>) -> Vec<Move> {
    let mut found_moves = false;
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let mut moves: Vec<Move> = Vec::new();

    for line in lines {
        if line.starts_with("move") {
            found_moves = true;
        }

        if !found_moves {
            continue;
        }

        let cap = re.captures(line).unwrap();
        let m = Move::new(
            cap[1].parse::<u32>().unwrap(),
            cap[2].parse::<u32>().unwrap(),
            cap[3].parse::<u32>().unwrap(),
        );
        moves.push(m);
    }

    moves
}

fn part1(stacks: &Vec<Stack>, moves: &Vec<Move>) -> String {
    let mut stacks = stacks.clone();

    for m in moves {
        let from_stack = &mut stacks[m.from as usize - 1];
        let mut moving = from_stack
            .drain(from_stack.len() - m.quantity as usize..)
            .collect::<Vec<_>>();

        moving.reverse();
        stacks[m.to as usize - 1].append(&mut moving);
    }

    stacks.iter().fold(String::new(), |acc, value| {
        format!("{}{}", acc, value[value.len() - 1])
    })
}

fn part2(stacks: &Vec<Stack>, moves: &Vec<Move>) -> String {
    let mut stacks = stacks.clone();

    for m in moves {
        let from_stack = &mut stacks[m.from as usize - 1];
        let mut moving = from_stack
            .drain(from_stack.len() - m.quantity as usize..)
            .collect::<Vec<_>>();

        stacks[m.to as usize - 1].append(&mut moving);
    }

    stacks.iter().fold(String::new(), |acc, value| {
        format!("{}{}", acc, value[value.len() - 1])
    })
}

fn main() {
    let input = fs::read("src/input.txt")
        .expect("Error reading file")
        .lines()
        .map(|line| line.unwrap().to_string())
        .collect::<Vec<_>>();

    let stacks = parse_stacks(&input, 9); // CHANGE TO NUMBER OF STACKS
    let moves = parse_moves(&input);

    println!("Part 1: {}", part1(&stacks, &moves));
    println!("Part 2: {}", part2(&stacks, &moves));
}
