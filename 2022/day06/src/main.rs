fn main() {
    let input = include_str!("../adventofcode.com-2022-day06-input.txt");
    let output = find_message_start(input, 4);
    println!("part 1: {output}");
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

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test() {
        const HEADER_SIZE: usize = 4;
        let input = include_str!("../test-input.txt");
        let lines: Vec<_> = input.lines().collect();
        let r0 = find_message_start(lines[0], HEADER_SIZE);
        let r1 = find_message_start(lines[1], HEADER_SIZE);
        let r2 = find_message_start(lines[2], HEADER_SIZE);
        let r3 = find_message_start(lines[3], HEADER_SIZE);
        let r4 = find_message_start(lines[4], HEADER_SIZE);

        assert_eq!(r0, 7);
        assert_eq!(r1, 5);
        assert_eq!(r2, 6);
        assert_eq!(r3, 10);
        assert_eq!(r4, 11);
    }
}
