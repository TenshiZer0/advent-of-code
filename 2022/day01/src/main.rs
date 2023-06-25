fn main() {
    let input = include_str!("../adventofcode.com-2022-day-01-input.txt");

    let mut highest_calories = [0, 0, 0];
    let mut calories_sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            let mut comparation = [
                calories_sum,
                highest_calories[0],
                highest_calories[1],
                highest_calories[2],
            ];
            comparation.sort();
            highest_calories = [comparation[3], comparation[2], comparation[1]];
            calories_sum = 0;
        } else {
            calories_sum += line.parse::<i32>().expect("Erro ao ler calorias");
        }
    }

    println!("highest calories bag: {}", highest_calories[0]);
    println!(
        "sum of three highest calories bag: {}",
        highest_calories.into_iter().sum::<i32>()
    );
}
