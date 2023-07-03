use std::collections::HashMap;

fn main() {
    println!("start!");
    let input = include_str!("../adventofcode.com-2022-day07-input.txt");
    let output = total_size_dirs_le_100_000(input);
    println!("part 1: {output}");
    println!("end!");
}

fn total_size_dirs_le_100_000(input: &str) -> usize {
    let mut root = Directory::new();
    let current_directory = &mut root;

    for line in input.lines() {
        if let Some((token, tail)) = line.split_once(' ') {
            if token == "$" {
                todo!();
            } else if token == "dir" {
                current_directory
                    .files
                    .insert(tail.to_string(), Item::Directory(Directory::new()));
            } else if let Ok(size) = token.parse::<usize>() {
                current_directory
                    .files
                    .insert(tail.to_string(), Item::File(size));
            }
        }
    }

    root.len()
}

#[derive(Debug)]
struct Directory {
    files: HashMap<String, Item>,
}

impl Directory {
    fn new() -> Self {
        Directory {
            files: HashMap::new(),
        }
    }
}

#[derive(Debug)]
enum Item {
    File(usize),
    Directory(Directory),
}

trait Len {
    fn len(&self) -> usize;
}

impl Len for Item {
    fn len(&self) -> usize {
        match self {
            Self::File(size) => *size,
            Self::Directory(dir) => dir.len(),
        }
    }
}

impl Len for Directory {
    fn len(&self) -> usize {
        self.files.values().map(|item| item.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let input = include_str!("../test-input.txt");
        let output = total_size_dirs_le_100_000(input);
        assert_eq!(output, 95437);
    }
}
