fn main() {
    let input = include_str!("../adventofcode.com-2022-day06-input.txt");
    part1(input);
    part2(input);
}

fn part1(input: &str) {
    let output = find_message_start(input, 4);
    println!("part 1: {output}");
}

fn part2(input: &str) {
    let output = find_message_start(input, 14);
    println!("part 2: {output}");
}

fn find_message_start(input: &str, header_size: usize) -> usize {
    let vec: Vec<_> = input.chars().collect();
    let chars = vec.as_slice();
    for id in 0..=(chars.len() - header_size) {
        let mut differents = true;
        for i in 0..(header_size - 1) {
            for j in (i + 1)..header_size {
                if chars[id + i] == chars[id + j] {
                    differents = false;
                    break;
                }
            }
        }
        if differents {
            return id + header_size;
        }
    }
    panic!("Não há uma sequência de {header_size} caracteres diferentes");
}
