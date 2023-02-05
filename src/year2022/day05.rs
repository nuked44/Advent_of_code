use advent_of_code::read_file_lines;

pub fn day05() {

    let _test_data = vec![
        "    [D]    ".to_owned(),
        "[N] [C]    ".to_owned(),
        "[Z] [M] [P]".to_owned(),
        " 1   2   3 ".to_owned(),
        "".to_owned(),
        "move 1 from 2 to 1".to_owned(),
        "move 3 from 1 to 3".to_owned(),
        "move 2 from 2 to 1".to_owned(),
        "move 1 from 1 to 2".to_owned(),
    ];
    
    print!("Part 1: ");
    part1(read_file_lines("./src/year2022/inputs/day05.txt"));
    print!("Part 2: ");
    part2(read_file_lines("./src/year2022/inputs/day05.txt"));

}

fn construct_stacks(construct: &mut Vec<Vec<char>>, num_of_stacks: usize, line: &String) {
    for i in 0..num_of_stacks {
        match line.as_bytes()[2 + i * 4] {
            b' ' => {}
            _ => { 
                construct[i].push(line.as_bytes()[1 + i * 4] as char);
            }
        }
    }
}

fn crate_mover(construct: &mut Vec<Vec<char>>, iterations: usize, start_stack: usize, end_stack: usize) {
    if iterations == 1 {
        let cur = construct[start_stack - 1].pop().unwrap();
        construct[end_stack - 1].push(cur);
    } else {
        for _ in 0..iterations {
            let cur = construct[start_stack - 1].pop().unwrap();
            construct[end_stack - 1].push(cur);
        }
    }
}

fn part1(input: Vec<String>) {
    let num_of_stacks: usize = (input[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for _ in 0..num_of_stacks {
        stacks.push(vec![]);
    }
    
    for line in input{
        if line == "" {
            for i in 0..num_of_stacks {
                stacks[i].reverse()
            }
        } else if line.contains('[') {
            construct_stacks(&mut stacks, num_of_stacks, &line);
        } else if line.starts_with("move") {
            let buffer: Vec<&str> = line.split_whitespace().collect();
            crate_mover(&mut stacks, buffer[1].parse().unwrap(), buffer[3].parse().unwrap(), buffer[5].parse().unwrap())
        }
    }
    let mut output: String = String::new();
    for i in 0..num_of_stacks {
        output.push(stacks[i].pop().unwrap())
    }
    println!("{output}\t");

}

fn crate_mover_9001(construct: &mut Vec<Vec<char>>, num_of_crates: usize, start_stack: usize, end_stack: usize) {
    let mut crate_buffer: Vec<char> = Vec::new();
    if num_of_crates == 1 {
        let cur = construct[start_stack - 1].pop().unwrap();
        construct[end_stack - 1].push(cur);
    } else {
        for _ in 0..num_of_crates {
            let cur = construct[start_stack - 1].pop().unwrap();
            crate_buffer.push(cur);
        }
        for _ in 0..crate_buffer.len() {
            let cur = crate_buffer.pop().unwrap();
            construct[end_stack - 1].push(cur);
        }
    }
}

fn part2(input: Vec<String>) {
    let num_of_stacks: usize = (input[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for _ in 0..num_of_stacks {
        stacks.push(vec![]);
    }

    for line in input{
        if line == "" {
            for i in 0..num_of_stacks {
                stacks[i].reverse()
            }
        } else if line.contains('[') {
            construct_stacks(&mut stacks, num_of_stacks, &line);
        } else if line.starts_with("move") {
            let buffer: Vec<&str> = line.split_whitespace().collect();
            crate_mover_9001(&mut stacks, buffer[1].parse().unwrap(), buffer[3].parse().unwrap(), buffer[5].parse().unwrap())
        }
    }
    let mut output: String = String::new();
    for i in 0..num_of_stacks {
        output.push(stacks[i].pop().unwrap())
    }
    println!("{output}\t");
}