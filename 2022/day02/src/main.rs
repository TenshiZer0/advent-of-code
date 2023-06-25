const ROCK_POINT: i32 = 1;
const PAPER_POINT: i32 = 2;
const SCISOR_POINT: i32 = 3;

const LOSS_POINT: i32 = 0;
const DRAW_POINT: i32 = 3;
const VICTORY_POINT: i32 = 6;

fn main() {
    // let input = include_str!("../test.txt");
    let input = include_str!("../adventofcode.com-2022-day02-input.txt");

    let score1: i32 = input
        .lines()
        .map(|line| match line {
            "A X" => DRAW_POINT + ROCK_POINT,
            "A Y" => VICTORY_POINT + PAPER_POINT,
            "A Z" => LOSS_POINT + SCISOR_POINT,
            "B X" => LOSS_POINT + ROCK_POINT,
            "B Y" => DRAW_POINT + PAPER_POINT,
            "B Z" => VICTORY_POINT + SCISOR_POINT,
            "C X" => VICTORY_POINT + ROCK_POINT,
            "C Y" => LOSS_POINT + PAPER_POINT,
            "C Z" => DRAW_POINT + SCISOR_POINT,
            _ => panic!("ERROR"),
        })
        .sum();

    let score2: i32 = input
        .lines()
        .map(|line| match line {
            "A X" => LOSS_POINT + SCISOR_POINT,
            "A Y" => DRAW_POINT + ROCK_POINT,
            "A Z" => VICTORY_POINT + PAPER_POINT,
            "B X" => LOSS_POINT + ROCK_POINT,
            "B Y" => DRAW_POINT + PAPER_POINT,
            "B Z" => VICTORY_POINT + SCISOR_POINT,
            "C X" => LOSS_POINT + PAPER_POINT,
            "C Y" => DRAW_POINT + SCISOR_POINT,
            "C Z" => VICTORY_POINT + ROCK_POINT,
            _ => panic!("ERROR"),
        })
        .sum();

    println!("strategy score: {}", score1);
    println!("right strategy score: {}", score2);
}
