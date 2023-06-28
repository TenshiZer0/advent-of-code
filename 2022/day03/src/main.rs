fn main() {
    let input = include_str!("../adventofcode.com-2022-day03-input.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let result: u32 = input
        .lines()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let chars: Vec<char> = second.chars().collect();
            match first.find(chars.as_slice()) {
                Some(position) => get_priority(first.chars().nth(position).unwrap()),
                None => 0,
            }
        })
        .sum();

    println!("sum of priorities of repeated items: {}", result);
}

fn part2(input: &str) {
    let result: u32 = group(input)
        .map(|(first, second, third)| get_priority(find_badge(first, second, third)))
        .sum();

    println!("sum of priorities of groups' badges: {}", result);
}

fn group(input: &str) -> impl Iterator<Item = (&str, &str, &str)> {
    let mut lines = input.lines();
    let first_items = lines.clone().step_by(3);
    lines.next();
    let second_items = lines.clone().step_by(3);
    lines.next();
    let third_items = lines.clone().step_by(3);
    first_items
        .zip(second_items)
        .zip(third_items)
        .map(|((f, s), t)| (f, s, t))
}

fn find_badge(first: &str, second: &str, third: &str) -> char {
    for item in first.chars() {
        if let Some(_) = second.find(item) {
            if let Some(_) = third.find(item) {
                return item;
            }
        }
    }
    panic!("Common item not found!");
}

fn get_priority(category: char) -> u32 {
    const ASCII_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    ASCII_LETTERS.find(category).unwrap() as u32 + 1
}
