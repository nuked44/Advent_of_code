use advent_of_code::read_file_lines;

pub fn day03() {
    let _test_data: Vec<String> = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp".to_owned(),
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_owned(),
        "PmmdzqPrVvPwwTWBwg".to_owned(),
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_owned(),
        "ttgJtRGJQctTZtZT".to_owned(),
        "CrZsJsPPZsGzwwsLwLmpwMDw".to_owned(),
    ];

    print!("Part 1: ");
    part1(read_file_lines("./src/year2022/day03/input.txt"));
    print!("\nPart 2: ");
    part2(read_file_lines("./src/year2022/day03/input.txt"));
}

fn char_to_val(c: char) -> usize {
    let mut value = 0;
    if ('a'..='z').contains(&c) {
        value = (c as u8 - b'a' + 1) as usize;
    } else if ('A'..='Z').contains(&c) {
        value = (c as u8 - b'A' + 27) as usize;
    }
    value
}

fn part1(input: Vec<String>) {
    let mut priority: usize = 0;
    for current_rucksack in input {
        'a: for comp1 in 0..(current_rucksack.len() / 2) {
            for comp2 in (current_rucksack.len() / 2)..current_rucksack.len() {
                if current_rucksack.as_bytes()[comp1] == current_rucksack.as_bytes()[comp2] {
                    priority += char_to_val(current_rucksack.as_bytes()[comp1] as char);
                    break 'a;
                }
            }
        }
    }
    println!("{priority}");
}

fn part2(input: Vec<String>) {
    let mut priority: usize = 0;
    for group in 0..input.len() / 3 {
        let l1 = &input[3 * group];
        let l2 = &input[3 * group + 1];
        let l3 = &input[3 * group + 2];
        'a: for i in l1.chars() {
            for j in l2.chars() {
                for k in l3.chars() {
                    if i == j && i == k {
                        priority += char_to_val(i);
                        break 'a;
                    }
                }
            }
        }
    }
    println!("{priority}");
}
