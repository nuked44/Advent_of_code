use advent_of_code::read_file_lines;

pub fn day06() {
    let _test_data = vec![
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_owned(),
        "bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned(),
        "nppdvjthqldpwncqszvftbrmjlhg".to_owned(),
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned(),
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned(),
    ];

    print!("Part 1: ");
    part1(read_file_lines("./src/year2022/inputs/day06.txt"));
    print!("Part 2: ");
    part2(read_file_lines("./src/year2022/inputs/day06.txt"));
}

struct PacketStack {
    size: usize,
    data: [char; 4],
}

impl PacketStack {
    fn new() -> PacketStack {
        PacketStack {
            size: 0,
            data: [' '; 4],
        }
    }

    fn push(&mut self, value: char) {
        if self.size == 4 {
            for i in 0..3 {
                self.data[i] = self.data[i + 1];
            }
            self.data[3] = value;
        } else {
            self.data[self.size] = value;
            self.size += 1;
        }
    }

    fn has_unique_values(&self) -> bool {
        for i in 0..self.size {
            for j in (i + 1)..self.size {
                if self.data[i] == self.data[j] {
                    return false;
                }
            }
        }
        true
    }
}

struct MessageStack {
    size: usize,
    data: [char; 14],
}

impl MessageStack {
    fn new() -> MessageStack {
        MessageStack {
            size: 0,
            data: [' '; 14],
        }
    }

    fn push(&mut self, value: char) {
        if self.size == 14 {
            for i in 0..13 {
                self.data[i] = self.data[i + 1];
            }
            self.data[13] = value;
        } else {
            self.data[self.size] = value;
            self.size += 1;
        }
    }

    fn has_unique_values(&self) -> bool {
        for i in 0..self.size {
            for j in (i + 1)..self.size {
                if self.data[i] == self.data[j] {
                    return false;
                }
            }
        }
        true
    }
}

fn part1(input: Vec<String>) {
    let mut out: Vec<usize> = Vec::new();
    for line in input {
        let mut stack = PacketStack::new();
        for (i, c) in line.chars().enumerate() {
            stack.push(c);
            if stack.size == 4 && stack.has_unique_values() {
                out.push(i + 1);
                break;
            }
        }
    }
    println!("{}", out.pop().unwrap())
}

fn part2(input: Vec<String>) {
    let mut out: Vec<usize> = Vec::new();
    for line in input {
        let mut stack = MessageStack::new();
        for (i, c) in line.chars().enumerate() {
            stack.push(c);
            if stack.size == 14 && stack.has_unique_values() {
                out.push(i + 1);
                break;
            }
        }
    }
    println!("{}", out.pop().unwrap())
}
