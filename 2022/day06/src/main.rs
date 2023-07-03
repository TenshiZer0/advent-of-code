fn main() {
    test();
}

fn test() {
    let input = include_str!("../input.txt");
    for line in input.lines() {
        let output = find_message_start(line, 4);
        println!("{output}");
    }
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
