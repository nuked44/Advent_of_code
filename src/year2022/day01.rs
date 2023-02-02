use advent_of_code::read_file_lines;

pub fn day01() {
    let _test_data = vec![
        "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
        "10000",
    ];

    let mut current: usize = 0;
    let mut temp: usize;
    let mut max1: usize = 0;
    let mut max2: usize = 0;
    let mut max3: usize = 0;
    for elf in read_file_lines("./src/year2022/input/day01.txt") {
        if !elf.is_empty() {
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
    println!("{}", max1 + max2 + max3);
}
