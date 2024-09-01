use std::fs::File;
use std::io::{BufRead, BufReader};

const SPELLING_TO_DIGIT: [(&str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn starts_with_spelled_digit(word: &str) -> Option<i32> {
    for (spelling, digit) in SPELLING_TO_DIGIT {
        match word.starts_with(spelling) {
            true => return Some(digit),
            false => (),
        }
    }

    None
}

fn tail(line: &str, start: usize) -> String {
    let mut index = 0;
    let mut str = String::new();

    for c in line.chars() {
        if index >= start {
            str.push(c);
        }

        index += 1;
    }

    str
}

fn get_calibration(line: &str) -> i32 {
    let mut digits = String::new();

    let mut index: usize = 0;
    let len = line.len();

    while index < len {
        let word = tail(line, index);
        let c = line.chars().nth(index).expect("Error c");
        if c.is_numeric() {
            digits.push(c);
        } else {
            match starts_with_spelled_digit(&word) {
                Some(digit) => {
                    for d in digit.to_string().chars() {
                        digits.push(d);
                    }
                }
                _ => (),
            }
        }

        index += 1;
    }

    let first = digits.chars().nth(0).unwrap().to_string();
    let last = digits.chars().last().unwrap().to_string();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stars_with_spelled_digit_exact_matches() {
        assert_eq!(starts_with_spelled_digit("one"), Some(1));
        assert_eq!(starts_with_spelled_digit("two"), Some(2));
        assert_eq!(starts_with_spelled_digit("three"), Some(3));
        assert_eq!(starts_with_spelled_digit("four"), Some(4));
        assert_eq!(starts_with_spelled_digit("five"), Some(5));
        assert_eq!(starts_with_spelled_digit("six"), Some(6));
        assert_eq!(starts_with_spelled_digit("seven"), Some(7));
        assert_eq!(starts_with_spelled_digit("eight"), Some(8));
        assert_eq!(starts_with_spelled_digit("nine"), Some(9));
    }

    #[test]
    fn test_stars_with_spelled_digit_partial_matches() {
        assert_eq!(starts_with_spelled_digit("oneth"), Some(1));
        assert_eq!(starts_with_spelled_digit("twot"), Some(2));
        assert_eq!(starts_with_spelled_digit("threefold"), Some(3));
        assert_eq!(starts_with_spelled_digit("fourth"), Some(4));
        assert_eq!(starts_with_spelled_digit("fives"), Some(5));
        assert_eq!(starts_with_spelled_digit("sixth"), Some(6));
        assert_eq!(starts_with_spelled_digit("seventh"), Some(7));
        assert_eq!(starts_with_spelled_digit("eighteen"), Some(8));
        assert_eq!(starts_with_spelled_digit("ninety"), Some(9));
    }

    #[test]
    fn test_tail() {
        assert_eq!(tail("string", 2), "ring");
        assert_eq!(tail("string", 0), "string");
        assert_eq!(tail("string", 6), "");
    }
}
