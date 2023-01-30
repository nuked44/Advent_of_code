use std::fs::File;
use std::io::{BufReader, BufRead};


pub fn day01() {
    let _test_data = vec![
    "1000",
    "2000",
    "3000",
    "",
    "4000",
    "",
    "5000",
    "6000",
    "",
    "7000",
    "8000",
    "9000",
    "",
    "10000",
    ];

    let mut current: usize = 0;
    let max_total: usize;
    let mut temp: usize;
    let mut max1: usize = 0;
    let mut max2: usize = 0;
    let mut max3: usize = 0;
    for elf in _read_file_lines("./src/year2022/day01/input.txt") {
            if elf != "" {
            current += elf.parse::<usize>().unwrap()
        } else {
            if current > max3 {
                max3 = current;
                if max3 > max2 {
                    temp = max2;
                    max2 = max3;
                    max3 = temp;
                    if max2 > max1 {
                        temp = max1;
                        max1 = max2;
                        max2 = temp;
                    }
                }
            }
            current = 0;
        }
    }
    if current > max3 {
        max3 = current;
        if max3 > max2 {
            temp = max2;
            max2 = max3;
            max3 = temp;
            if max2 > max1 {
                temp = max1;
                max1 = max2;
                max2 = temp;
            }
        }
    }
    max_total = max1 + max2 + max3;
    println!("{max_total}")
}

fn _read_file_lines(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            lines.push("".to_owned());
        } else {
            lines.push(line);
        }
    }

    lines
}
