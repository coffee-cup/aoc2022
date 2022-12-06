use std::{fmt::Display, fs, io::BufRead};

#[derive(PartialEq, Clone, Debug)]
struct Item {
    pub code: char,
}

impl Item {
    fn new(code: char) -> Self {
        Self { code }
    }

    fn priority(&self) -> u32 {
        if self.code.to_string() == self.code.to_lowercase().to_string() {
            self.code as u32 - 96
        } else {
            self.code as u32 - 38
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

#[derive(Clone, Debug)]
struct Rucksack {
    pub items: Vec<Item>,
}

impl Rucksack {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }

    pub fn first(&self) -> Vec<Item> {
        self.items.clone()[..self.half()].to_vec()
    }

    pub fn second(&self) -> Vec<Item> {
        self.items.clone()[self.half()..].to_vec()
    }

    fn half(&self) -> usize {
        self.items.len() / 2
    }
}

impl Display for Rucksack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.items.iter().map(|i| i.code).collect::<String>()
        )
    }
}

fn part1(sacks: &Vec<Rucksack>) -> u32 {
    let mut score: u32 = 0;

    for sack in sacks {
        let first = sack.first();
        let second = sack.second();

        for item in &first {
            if second.contains(item) {
                score += item.priority();

                break;
            }
        }
    }

    score
}

fn part2(sacks: &Vec<Rucksack>) -> u32 {
    let mut score: u32 = 0;

    for i in (1..sacks.len() - 1).step_by(3) {
        let first = sacks[i - 1].items.clone();
        let second = sacks[i].items.clone();
        let third = sacks[i + 1].items.clone();

        // Find common items between all three vectors
        let common = first
            .iter()
            .filter(|item| second.contains(item) && third.contains(item))
            .collect::<Vec<_>>();

        // println!("COMMON: {} = {}", common[0], common[0].priority());

        score += common[0].priority();
    }

    score
}

fn main() {
    let sacks = fs::read("src/input.txt")
        .expect("Error reading file")
        .lines()
        .map(|line| {
            let items = line.unwrap().chars().map(|code| Item::new(code)).collect();
            let rucksack = Rucksack::new(items);
            rucksack
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&sacks));
    println!("Part 2: {}", part2(&sacks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_priority() {
        assert_eq!(1, Item::new('a').priority());
        assert_eq!(26, Item::new('z').priority());
        assert_eq!(27, Item::new('A').priority());
        assert_eq!(52, Item::new('Z').priority());
    }
}
