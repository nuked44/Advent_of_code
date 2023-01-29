use std::time::Instant;

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
            ],
    ];

    let args: Vec<String> = std::env::args().collect();
    let which_year;
    let which_day;

    if args.len() > 2 {
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
        println!("Add args <year> <day>");
        std::process::exit(1);
    }
}

fn run(func: fn()) {
    let now = Instant::now();
    func();
    let elapsed = now.elapsed();
    println!(
        "\nCompletion time: {:6.2}ms\n",
        elapsed.as_micros() as f64 / 1000.
    );
}