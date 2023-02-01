use advent_of_code::read_file_lines;
use std::vec;

pub fn day02() {
    let _test_data = vec!["A Y", "B X", "C Z"];

    print!("Part 1: ");
    part1();
    print!("\nPart 2: ");
    part2();
}

fn part1() {
    let mut points_total: usize = 0;

    for i in read_file_lines("./src/year2022/day02/input.txt") {
        let fight: Vec<&str> = i.split_whitespace().collect();
        let add_choice: usize;
        let add_win: usize;

        match fight[0] {
            "A" => {
                //Rock
                match fight[1] {
                    "X" => {
                        //Rock
                        add_win = 3;
                        add_choice = 1;
                    }
                    "Y" => {
                        //Paper
                        add_win = 6;
                        add_choice = 2;
                    }
                    "Z" => {
                        //Scissors
                        add_win = 0;
                        add_choice = 3;
                    }
                    _ => panic!("Failed match fight inner A"),
                }
            }
            "B" => {
                //Paper
                match fight[1] {
                    "X" => {
                        //Rock
                        add_win = 0;
                        add_choice = 1;
                    }
                    "Y" => {
                        //Paper
                        add_win = 3;
                        add_choice = 2;
                    }
                    "Z" => {
                        //Scissors
                        add_win = 6;
                        add_choice = 3;
                    }
                    _ => panic!("Failed match fight inner B"),
                }
            }
            "C" => {
                //Scissors
                match fight[1] {
                    "X" => {
                        //Rock
                        add_win = 6;
                        add_choice = 1;
                    }
                    "Y" => {
                        //Paper
                        add_win = 0;
                        add_choice = 2;
                    }
                    "Z" => {
                        //Scissors
                        add_win = 3;
                        add_choice = 3;
                    }
                    _ => panic!("Failed match fight inner C"),
                }
            }
            _ => panic!("Failed match fight outer"),
        }
        points_total += add_choice;
        points_total += add_win;
    }

    println!("{points_total}");
}

fn part2() {
    let _test_data = vec!["A Y", "B X", "C Z"];

    let mut points_total: usize = 0;

    for i in read_file_lines("./src/year2022/day02/input.txt") {
        let fight: Vec<&str> = i.split_whitespace().collect();
        let add_choice: usize;
        let add_win: usize;

        match fight[0] {
            "A" => {
                //Rock
                match fight[1] {
                    "X" => {
                        //Lose / Scissors
                        add_win = 0;
                        add_choice = 3;
                    }
                    "Y" => {
                        //Draw / Rock
                        add_win = 3;
                        add_choice = 1;
                    }
                    "Z" => {
                        //Win / Paper
                        add_win = 6;
                        add_choice = 2;
                    }
                    _ => panic!("Failed match fight inner A"),
                }
            }
            "B" => {
                //Paper
                match fight[1] {
                    "X" => {
                        //Lose / Rock
                        add_win = 0;
                        add_choice = 1;
                    }
                    "Y" => {
                        //Draw / Paper
                        add_win = 3;
                        add_choice = 2;
                    }
                    "Z" => {
                        //Win / Scissors
                        add_win = 6;
                        add_choice = 3;
                    }
                    _ => panic!("Failed match fight inner A"),
                }
            }
            "C" => {
                //Scissors
                match fight[1] {
                    "X" => {
                        //Lose / Paper
                        add_win = 0;
                        add_choice = 2;
                    }
                    "Y" => {
                        //Draw / Scissors
                        add_win = 3;
                        add_choice = 3;
                    }
                    "Z" => {
                        //Win / Rock
                        add_win = 6;
                        add_choice = 1;
                    }
                    _ => panic!("Failed match fight inner A"),
                }
            }
            _ => panic!("Failed match fight outer"),
        }
        points_total += add_choice;
        points_total += add_win;
    }

    println!("{points_total}");
}
