use advent_of_code::run;
use std::fs;
use std::io::prelude::*;

mod year2022;

fn main() {
    let entrys: Vec<Vec<fn()>> = vec![
        // 2015 -> [0]    
        vec![],
        // 2016 -> [1]    
        vec![],
        // 2017 -> [2]    
        vec![],
        // 2018 -> [3]    
        vec![],
        // 2019 -> [4]    
        vec![],
        // 2020 -> [5]    
        vec![],
        // 2021 -> [6]    
        vec![],
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
        if args[1] != "make" {
            if let Ok(year) = args[1].parse::<usize>(){
                which_year = year - 2015;
            } else {
                println!("no valid year given");
                std::process::exit(1);
            }
            if let Ok(day) = args[2].parse::<usize>(){
                which_day = day - 1;
            } else {
                println!("no valid day given");
                std::process::exit(1);
            }
            println!("---- {} - {} ----\n", &args[1], &args[2]);
            run(entrys[which_year][which_day]);
        } else {
            for i in 1..=25{
                let path;
                let finput;
                if i < 10 {
                    path = format!("./src/year{}/day0{i}", args[2]);
                    finput = format!("pub fn day0{i}() {{\n\n}}")
                } else {
                    path = format!("./src/year{}/day{i}", args[2]);
                    finput = format!("pub fn day{i}() {{\n\n}}")
                }
                    fs::create_dir_all(&path)
                        .expect(format!("Failed dir {}", path).as_str());
                    let mut file: fs::File = fs::File::create(format!("{}/mod.rs", &path))
                        .expect(format!("Failed file {}", path).as_str());
                    write!(&mut file, "{}", finput).expect(format!("Failed finput {}", path).as_str());
            }
        }
    } else {
        println!("Add args <year> <day>");
        std::process::exit(1);
    }
}
