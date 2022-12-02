use std::fs;

trait Scoreable {
    fn score(&self) -> u32;
}

#[derive(PartialEq, Clone, Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn from(s: &str) -> Outcome {
        match s {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid outcome"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn from(s: &str) -> Hand {
        match s {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissors,
            c => panic!("Invalid hand: {}", c),
        }
    }

    fn counter(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn loser(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

impl Scoreable for Hand {
    fn score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

#[derive(Clone, Debug)]
struct Round {
    our_hand: Hand,
    their_hand: Hand,
}

impl Round {
    fn new(our_hand: Hand, their_hand: Hand) -> Round {
        Round {
            our_hand: our_hand,
            their_hand: their_hand,
        }
    }

    fn win_score(&self) -> u32 {
        match (&self.our_hand, &self.their_hand) {
            (Hand::Rock, Hand::Scissors) => 6,
            (Hand::Scissors, Hand::Paper) => 6,
            (Hand::Paper, Hand::Rock) => 6,
            (us, them) => {
                if us == them {
                    3
                } else {
                    0
                }
            }
        }
    }
}

impl Scoreable for Round {
    fn score(&self) -> u32 {
        self.win_score() + self.our_hand.score()
    }
}

fn part1(input: &String) -> u32 {
    let rounds = input
        .lines()
        .map(|l| {
            let t = l.split_whitespace().collect::<Vec<_>>();
            let r = Round::new(Hand::from(t[1]), Hand::from(t[0]));
            r
        })
        .collect::<Vec<_>>();

    let score = rounds.iter().fold(0, |acc, r| acc + r.score());
    score
}

fn part2(input: &String) -> u32 {
    let rounds = input
        .lines()
        .map(|l| {
            let t = l.split_whitespace().collect::<Vec<_>>();
            let their_hand = Hand::from(t[0]);
            let our_hand = match Outcome::from(t[1]) {
                Outcome::Win => their_hand.counter(),
                Outcome::Loss => their_hand.loser(),
                Outcome::Draw => their_hand.clone(),
            };

            Round::new(our_hand, their_hand)
        })
        .collect::<Vec<_>>();

    let score = rounds.iter().fold(0, |acc, r| acc + r.score());
    score
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Unable to read file");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
