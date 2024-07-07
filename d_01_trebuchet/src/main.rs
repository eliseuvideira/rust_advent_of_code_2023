use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_calibration(line: &str) -> i32 {
    let mut first = String::new();
    let mut last = String::new();

    for c in line.chars() {
        if c.is_numeric() {
            if first.is_empty() {
                first = c.to_string();
                last = c.to_string();
            } else {
                last = c.to_string();
            }
        }
    }

    let number = format!("{}{}", first, last);

    let calibration: i32 = number.trim().parse().unwrap();

    calibration
}

fn main() {
    let filename = "input.txt";

    let file = File::open(filename).expect("Could not open file");

    let reader = BufReader::new(file);

    let mut total_calibration = 0;

    for line in reader.lines() {
        let current_line = line.expect("Could not read line");

        match current_line.is_empty() {
            true => continue,
            false => (),
        }

        let calibration = get_calibration(&current_line);

        total_calibration += calibration;
    }

    println!("{}", total_calibration);
}
