use advent_of_code::read_file_lines;

pub fn day04() {
    let _test_data = vec![
        "2-4,6-8".to_owned(),
        "2-3,4-5".to_owned(),
        "5-7,7-9".to_owned(),
        "2-8,3-7".to_owned(),
        "6-6,4-6".to_owned(),
        "2-6,4-8".to_owned(),
    ];

    print!("Part 1: ");
    part1(read_file_lines("./src/year2022/inputs/day04.txt"));
    print!("Part 2: ");
    part2(read_file_lines("./src/year2022/inputs/day04.txt"));
}

fn check_full_overlap(input: &[&str]) -> bool {
    (input[0].parse::<usize>().unwrap() <= input[2].parse::<usize>().unwrap()
        && input[1].parse::<usize>().unwrap() >= input[3].parse::<usize>().unwrap())
        || (input[0].parse::<usize>().unwrap() >= input[2].parse::<usize>().unwrap()
            && input[1].parse::<usize>().unwrap() <= input[3].parse::<usize>().unwrap())
}

fn part1(input: Vec<String>) {
    let mut total: usize = 0;
    for i in input {
        let pair: Vec<&str> = i.split([',', '-']).collect();
        if check_full_overlap(&pair) {
            total += 1;
        }
    }
    println!("{total}");
}

fn part2(input: Vec<String>) {
    let mut total: usize = 0;
    for i in input {
        let pair: Vec<&str> = i.split([',', '-']).collect();
        'a: for j in pair[0].parse::<usize>().unwrap()..=pair[1].parse::<usize>().unwrap() {
            for k in pair[2].parse::<usize>().unwrap()..=pair[3].parse::<usize>().unwrap() {
                if j == k {
                    total += 1;
                    break 'a;
                }
            }
        }
    }
    println!("{total}");
}
