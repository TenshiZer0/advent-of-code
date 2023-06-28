fn main() {
    let input = include_str!("../adventofcode.com-2022-day02-input.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let result: u32 = input
        .lines()
        .map(|line| {
            let me = Jokenpo::from(line.chars().nth(2).unwrap());
            let other = Jokenpo::from(line.chars().nth(0).unwrap());
            let result = me.compete(other);
            result.get_score() + me.get_score()
        })
        .sum();
    println!("part 1: {}", result);
}

fn part2(input: &str) {
    let result: u32 = input
        .lines()
        .map(|line| {
            let other = Jokenpo::from(line.chars().nth(0).unwrap());
            let result = RoundResult::from(line.chars().nth(2).unwrap());
            let me = result.over(other);
            result.get_score() + me.get_score()
        })
        .sum();
    println!("part 2: {}", result);
}

#[derive(Clone, Copy)]
enum Jokenpo {
    Rock,
    Paper,
    Scisor,
}

impl Jokenpo {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scisor,
            _ => panic!("Invalid char {}", c),
        }
    }

    fn compete(&self, other: Jokenpo) -> RoundResult {
        match (self, other) {
            (Self::Rock, Self::Paper) => RoundResult::Loss,
            (Self::Rock, Self::Scisor) => RoundResult::Victory,
            (Self::Paper, Self::Rock) => RoundResult::Victory,
            (Self::Paper, Self::Scisor) => RoundResult::Loss,
            (Self::Scisor, Self::Rock) => RoundResult::Loss,
            (Self::Scisor, Self::Paper) => RoundResult::Victory,
            _ => RoundResult::Draw,
        }
    }

    fn get_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scisor => 3,
        }
    }
}

#[derive(Clone, Copy)]
enum RoundResult {
    Loss,
    Draw,
    Victory,
}

impl RoundResult {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Victory,
            _ => panic!("Invalid char {}", c),
        }
    }

    fn over(&self, jokenpo: Jokenpo) -> Jokenpo {
        match (self, jokenpo) {
            (Self::Loss, Jokenpo::Rock) => Jokenpo::Scisor,
            (Self::Loss, Jokenpo::Paper) => Jokenpo::Rock,
            (Self::Loss, Jokenpo::Scisor) => Jokenpo::Paper,
            (Self::Victory, Jokenpo::Rock) => Jokenpo::Paper,
            (Self::Victory, Jokenpo::Paper) => Jokenpo::Scisor,
            (Self::Victory, Jokenpo::Scisor) => Jokenpo::Rock,
            _ => jokenpo,
        }
    }

    fn get_score(&self) -> u32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Victory => 6,
        }
    }
}
