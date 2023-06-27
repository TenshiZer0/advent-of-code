fn main() {
    let input = include_str!("../adventofcode.com-2022-day03-input.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let result: i32 = input
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
    let mut lines = input.lines();
    let first_items = lines.clone().step_by(3);
    lines.next();
    let second_items = lines.clone().step_by(3);
    lines.next();
    let third_items = lines.clone().step_by(3);
    let groups = first_items.zip(second_items).zip(third_items);

    let result: i32 = groups
        .map(|((first, second), third)| get_priority(find_badge(first, second, third)))
        .sum();

    println!("sum of priorities of groups' badges: {}", result);
}

fn find_badge(first: &str, second: &str, third: &str) -> char {
    for char1 in first.chars() {
        for char2 in second.chars() {
            for char3 in third.chars() {
                if char1 == char2 && char1 == char3 {
                    return char1;
                }
            }
        }
    }
    panic!("Did'nt find common items");
}

fn get_priority(category: char) -> i32 {
    match category {
        'a'..='z' => category as i32 - 'a' as i32 + 1,
        'A'..='Z' => category as i32 - 'A' as i32 + 27,
        _ => panic!("{:?}", category),
    }
}
