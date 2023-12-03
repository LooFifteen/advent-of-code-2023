use std::ops::Add;
use regex::Regex;

const INPUT_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");

const SYMBOLS: [char; 10] = ['*', '#', '+', '$', '/', '=', '%', '-', '&', '@'];

fn main() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();

    // initialize sum
    let mut sum = 0;

    // iterate through each line
    let lines = input.lines();
    for i in 0..lines.count() {
        // store the useful lines
        let previous_line = match i {
            0 => None,
            _ => Some(input.lines().nth(i - 1).unwrap()),
        };
        let line = input.lines().nth(i).unwrap();
        let next_line = input.lines().nth(i + 1);

        // find all numbers in the current line
        let numbers = find_numbers(line);

        // check each number to see if it has a symbol
        for number in numbers {
            if number.has_symbol(previous_line, line, next_line) {
                sum += number.value;
            }
        }
    }

    // output the sum
    println!("{}", sum);
}

struct Number {
    value: i32,
    range: std::ops::Range<usize>,
}

impl Number {
    fn has_symbol(&self, previous_line: Option<&str>, current_line: &str, next_line: Option<&str>) -> bool {
        // create a range that is 1 character before and after the number
        let mut range = self.range.clone();
        range.start = range.start.saturating_sub(1);
        range.end = if range.end == current_line.len() {
            range.end
        } else {
            range.end.add(1)
        };

        // check current line first
        if check_line(current_line, range.clone()) {
            return true;
        }

        // check previous line
        if let Some(previous_line) = previous_line {
            if check_line(previous_line, range.clone()) {
                return true;
            }
        }

        // check next line
        if let Some(next_line) = next_line {
            if check_line(next_line, range) {
                return true;
            }
        }

        false
    }
}

fn check_line(line: &str, range: std::ops::Range<usize>) -> bool {
    let slice = &line[range];
    for symbol in SYMBOLS {
        if slice.contains(symbol) {
            return true;
        }
    }
    false
}

fn find_numbers(line: &str) -> Vec<Number> {
    let mut numbers = Vec::new();

    for capture in Regex::new(r"[0-9]+").unwrap().captures_iter(line) {
        if let Some(capture) = capture.get(0) {
            let range = capture.range();
            let value = capture.as_str().parse::<i32>().unwrap();
            numbers.push(Number {
                value,
                range,
            });
        }
    }

    numbers
}
