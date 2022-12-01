use std::fs;

type Elf = Vec<u32>;

fn part1(elves: &Vec<Elf>) -> u32 {
    let max = elves.iter().map(|e| e.iter().sum()).max().unwrap();
    max
}

fn part2(elves: &Vec<Elf>) -> u32 {
    let mut elves = elves.iter().map(|e| e.iter().sum()).collect::<Vec<u32>>();
    elves.sort_by(|a, b| b.cmp(a));

    elves[0] + elves[1] + elves[2]
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let lines = input.lines().collect::<Vec<_>>();

    let mut elves: Vec<Elf> = Vec::new();
    let mut curr_elf = Elf::new();

    for line in lines {
        if line == "" {
            elves.push(curr_elf);
            curr_elf = Elf::new();
            continue;
        }

        let calories = line.parse::<u32>().unwrap();
        curr_elf.push(calories);
    }

    elves.push(curr_elf);

    println!("Part 1: {}", part1(&elves));
    println!("Part 2: {}", part2(&elves));
}
