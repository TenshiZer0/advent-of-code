fn main() {
    let input = include_str!("../adventofcode.com-2022-day05-input.txt");

    let first_line = input.split_once('\n').unwrap().0;
    let number_of_stacks = (first_line.len() + 1) / 4;

    let blank_line_position = input.find("\n\n").unwrap();
    let (stack_input, movements_input) = input.split_at(blank_line_position);

    // reading stacks
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(number_of_stacks);
    for _ in 0..number_of_stacks {
        stacks.push(Vec::new());
    }

    for line in stack_input.lines() {
        for stack_id in 0..number_of_stacks {
            let position = 4 * stack_id + 1;
            if let Some(item) = line.chars().nth(position) {
                if item.is_ascii_alphabetic() {
                    stacks[stack_id].push(item);
                }
            }
        }
    }

    for id in 0..stacks.len() {
        stacks[id].reverse();
    }

    // reading movements
    let mut stacks1 = stacks.clone();
    let lines = movements_input.lines().filter(|line| !line.is_empty());
    for line in lines {
        let mut tokens = line.split(' ');
        let quantity = tokens.nth(1).unwrap().parse::<usize>().unwrap();
        let from = tokens.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = tokens.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        // println!("move {} from {} to {}", quantity, from, to);
        for _ in 0..quantity {
            let item = stacks1[from].pop().unwrap();
            stacks1[to].push(item);
        }
    }

    // printing result part 1
    let mut top_items = String::new();
    for mut stack in stacks1 {
        top_items.push(stack.pop().unwrap());
    }

    println!("part 1: {}", top_items);

    // reading movements
    let lines = movements_input.lines().filter(|line| !line.is_empty());
    for line in lines {
        let mut tokens = line.split(' ');
        let quantity = tokens.nth(1).unwrap().parse::<usize>().unwrap();
        let from = tokens.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = tokens.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        // println!("move {} from {} to {}", quantity, from, to);
        let mut temp_stack = Vec::with_capacity(quantity);
        for _ in 0..quantity {
            let item = stacks[from].pop().unwrap();
            temp_stack.push(item);
        }
        temp_stack.reverse();
        for item in temp_stack {
            stacks[to].push(item);
        }
    }

    // printing result part 2
    let mut top_items = String::new();
    for mut stack in stacks {
        top_items.push(stack.pop().unwrap());
    }

    println!("part 2: {}", top_items);
}
