fn main() {
    let input = include_str!("../adventofcode.com-2022-day04-input.txt");

    let input = input
        .lines()
        .filter_map(|line| line.split_once(','))
        .map(|(first, second)| {
            let parser = |(min, max): (&str, &str)| {
                (min.parse::<u32>().unwrap(), max.parse::<u32>().unwrap())
            };
            let first = first.split_once('-').map(parser).unwrap();
            let second = second.split_once('-').map(parser).unwrap();
            (first, second)
        });

    // part 1
    let output = input
        .clone()
        .filter(|(first, second)| {
            (first.0 >= second.0 && first.1 <= second.1)
                || (second.0 >= first.0 && second.1 <= first.1)
        })
        .count();

    println!("part 1: {:?}", output);

    // part 2
    let output = input
        .filter(|(first, second)| {
            (first.0 >= second.0 && first.0 <= second.1)
                || (first.1 >= second.0 && first.0 <= second.1)
                || (second.0 >= first.0 && second.0 <= first.1)
                || (second.1 >= first.0 && second.1 <= first.1)
        })
        .count();

    println!("part 2: {:?}", output);
}
