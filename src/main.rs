use advent_of_code::run;
use std::{fs, io::prelude::*, time::Instant};

mod year2022;

fn main() {
    let entrys: Vec<Vec<fn()>> = vec![
        // 2015 -> [0]
        vec![
            /*
            Also change year2015/mod.rs
            year2015::day01::day01,
            year2015::day02::day02,
            year2015::day03::day03,
            year2015::day04::day04,
            year2015::day05::day05,
            year2015::day06::day06,
            year2015::day07::day07,
            year2015::day08::day08,
            year2015::day09::day09,
            year2015::day10::day10,
            year2015::day11::day11,
            year2015::day12::day12,
            year2015::day13::day13,
            year2015::day14::day14,
            year2015::day15::day15,
            year2015::day16::day16,
            year2015::day17::day17,
            year2015::day18::day18,
            year2015::day19::day19,
            year2015::day20::day20,
            year2015::day21::day21,
            year2015::day22::day22,
            year2015::day23::day23,
            year2015::day24::day24,
            year2015::day25::day25,
            */
        ],
        // 2016 -> [1]
        vec![
            /*
            Also change year2016/mod.rs
            year2016::day01::day01,
            year2016::day02::day02,
            year2016::day03::day03,
            year2016::day04::day04,
            year2016::day05::day05,
            year2016::day06::day06,
            year2016::day07::day07,
            year2016::day08::day08,
            year2016::day09::day09,
            year2016::day10::day10,
            year2016::day11::day11,
            year2016::day12::day12,
            year2016::day13::day13,
            year2016::day14::day14,
            year2016::day15::day15,
            year2016::day16::day16,
            year2016::day17::day17,
            year2016::day18::day18,
            year2016::day19::day19,
            year2016::day20::day20,
            year2016::day21::day21,
            year2016::day22::day22,
            year2016::day23::day23,
            year2016::day24::day24,
            year2016::day25::day25,
            */
        ],
        // 2017 -> [2]
        vec![
            /*
            Also change year2017/mod.rs
            year2017::day01::day01,
            year2017::day02::day02,
            year2017::day03::day03,
            year2017::day04::day04,
            year2017::day05::day05,
            year2017::day06::day06,
            year2017::day07::day07,
            year2017::day08::day08,
            year2017::day09::day09,
            year2017::day10::day10,
            year2017::day11::day11,
            year2017::day12::day12,
            year2017::day13::day13,
            year2017::day14::day14,
            year2017::day15::day15,
            year2017::day16::day16,
            year2017::day17::day17,
            year2017::day18::day18,
            year2017::day19::day19,
            year2017::day20::day20,
            year2017::day21::day21,
            year2017::day22::day22,
            year2017::day23::day23,
            year2017::day24::day24,
            year2017::day25::day25,
            */
        ],
        // 2018 -> [3]
        vec![
            /*
            Also change year2018/mod.rs
            year2018::day01::day01,
            year2018::day02::day02,
            year2018::day03::day03,
            year2018::day04::day04,
            year2018::day05::day05,
            year2018::day06::day06,
            year2018::day07::day07,
            year2018::day08::day08,
            year2018::day09::day09,
            year2018::day10::day10,
            year2018::day11::day11,
            year2018::day12::day12,
            year2018::day13::day13,
            year2018::day14::day14,
            year2018::day15::day15,
            year2018::day16::day16,
            year2018::day17::day17,
            year2018::day18::day18,
            year2018::day19::day19,
            year2018::day20::day20,
            year2018::day21::day21,
            year2018::day22::day22,
            year2018::day23::day23,
            year2018::day24::day24,
            year2018::day25::day25,
            */
        ],
        // 2019 -> [4]
        vec![
            /*
            Also change year2019/mod.rs
            year2019::day01::day01,
            year2019::day02::day02,
            year2019::day03::day03,
            year2019::day04::day04,
            year2019::day05::day05,
            year2019::day06::day06,
            year2019::day07::day07,
            year2019::day08::day08,
            year2019::day09::day09,
            year2019::day10::day10,
            year2019::day11::day11,
            year2019::day12::day12,
            year2019::day13::day13,
            year2019::day14::day14,
            year2019::day15::day15,
            year2019::day16::day16,
            year2019::day17::day17,
            year2019::day18::day18,
            year2019::day19::day19,
            year2019::day20::day20,
            year2019::day21::day21,
            year2019::day22::day22,
            year2019::day23::day23,
            year2019::day24::day24,
            year2019::day25::day25,
            */
        ],
        // 2020 -> [5]
        vec![
            /*
            Also change year2020/mod.rs
            year2020::day01::day01,
            year2020::day02::day02,
            year2020::day03::day03,
            year2020::day04::day04,
            year2020::day05::day05,
            year2020::day06::day06,
            year2020::day07::day07,
            year2020::day08::day08,
            year2020::day09::day09,
            year2020::day10::day10,
            year2020::day11::day11,
            year2020::day12::day12,
            year2020::day13::day13,
            year2020::day14::day14,
            year2020::day15::day15,
            year2020::day16::day16,
            year2020::day17::day17,
            year2020::day18::day18,
            year2020::day19::day19,
            year2020::day20::day20,
            year2020::day21::day21,
            year2020::day22::day22,
            year2020::day23::day23,
            year2020::day24::day24,
            year2020::day25::day25,
            */
        ],
        // 2021 -> [6]
        vec![
            /*
            Also change year2021/mod.rs
            year2021::day01::day01,
            year2021::day02::day02,
            year2021::day03::day03,
            year2021::day04::day04,
            year2021::day05::day05,
            year2021::day06::day06,
            year2021::day07::day07,
            year2021::day08::day08,
            year2021::day09::day09,
            year2021::day10::day10,
            year2021::day11::day11,
            year2021::day12::day12,
            year2021::day13::day13,
            year2021::day14::day14,
            year2021::day15::day15,
            year2021::day16::day16,
            year2021::day17::day17,
            year2021::day18::day18,
            year2021::day19::day19,
            year2021::day20::day20,
            year2021::day21::day21,
            year2021::day22::day22,
            year2021::day23::day23,
            year2021::day24::day24,
            year2021::day25::day25,
            */
        ],
        // 2022 -> [7]
        vec![
            year2022::day01::day01,
            year2022::day02::day02,
            year2022::day03::day03,
            year2022::day04::day04,
            year2022::day05::day05,
            year2022::day06::day06,
            year2022::day07::day07,
            year2022::day08::day08,
            year2022::day09::day09,
            year2022::day10::day10,
            year2022::day11::day11,
            year2022::day12::day12,
            year2022::day13::day13,
            year2022::day14::day14,
            year2022::day15::day15,
            year2022::day16::day16,
            year2022::day17::day17,
            year2022::day18::day18,
            year2022::day19::day19,
            year2022::day20::day20,
            year2022::day21::day21,
            year2022::day22::day22,
            year2022::day23::day23,
            year2022::day24::day24,
            year2022::day25::day25,
        ],
    ];

    let args: Vec<String> = std::env::args().collect();
    let which_year;
    let which_day;

    if args.len() == 3 {
        if args[1] == "make" {
            for i in 1..=25 {
                let path = format!("./src/year{}", args[2]);
                let fpath;
                let finput = if i < 10 {
                    fpath = format!("{}/day0{i}.rs", &path);
                    format!("pub fn day0{i}() {{\n\nprint!(\"Part 1: \");\npart1();\nprint!(\"Part 2: \");\npart2();\n\n}}\n\nfn part1() {{\n\n}}\n\nfn part2() {{\n\n}}")
                } else {
                    fpath = format!("{}/day{i}.rs", &path);
                    format!("pub fn day{i}() {{\n\nprint!(\"Part 1: \");\npart1();\nprint!(\"Part 2: \");\npart2();\n\n}}\n\nfn part1() {{\n\n}}\n\nfn part2() {{\n\n}}")
                };
                fs::create_dir_all(&path).unwrap_or_else(|_| panic!("Failed dir {path}"));
                let mut file: fs::File =
                    fs::File::create(&fpath).unwrap_or_else(|_| panic!("Failed file {path}"));
                write!(&mut file, "{finput}").unwrap_or_else(|_| panic!("Failed finput {path}"));
            }
        } else {
            if let Ok(year) = args[1].parse::<usize>() {
                which_year = year - 2015;
            } else {
                println!("no valid year given");
                std::process::exit(1);
            }
            if let Ok(day) = args[2].parse::<usize>() {
                which_day = day - 1;
            } else {
                println!("no valid day given");
                std::process::exit(1);
            }
            println!("---- {} - {} ----\n", &args[1], &args[2]);
            println!(
                "\nCompletion time: {:6.2}ms",
                run(entrys[which_year][which_day])
            );
        }
    } else if args.len() == 2 {
        if let Ok(year) = args[1].parse::<usize>() {
            which_year = year - 2015;
        } else {
            println!("no valid year given");
            std::process::exit(1);
        }
        let now = Instant::now();
        for i in 0..entrys[which_year].len() {
            println!("---- {} - {} ----\n", &args[1], i + 1);
            println!("\nCompletion time: {:6.2}ms", run(entrys[which_year][i]));
        }
        let elapsed = now.elapsed();
        println!(
            "\nTotal completion time: {:6.2}ms",
            elapsed.as_micros() as f64 / 1000.
        )
    } else {
        println!("Add args -- <year> for whole year");
        println!("Add args -- <year> <day> for specific day");
        println!("Add args -- make <year> to generate templlate fo <year>");
        std::process::exit(1);
    }
}
