use std::{fs::File, 
    io::{BufReader, BufRead}, 
    time::Instant,};

pub fn read_file_lines(file_path: &str) -> Vec<String> {
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

pub fn run(func: fn()) -> f64 {
    let now = Instant::now();
    func();
    let elapsed = now.elapsed();

        elapsed.as_micros() as f64 / 1000.
}