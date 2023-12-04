use std::ops::Add;
use regex::Regex;

const INPUT_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");

const SYMBOLS: [char; 10] = ['*', '#', '+', '$', '/', '=', '%', '-', '&', '@'];

fn main() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();

    // initialize sum
    let mut part_1 = 0;
    let mut part_2 = 0;

    // iterate through each line
    let lines: Vec<&str> = input.lines().into_iter().collect();
    for i in 0..lines.len() {
        // store the useful lines
        let (previous_line, line, next_line) = get_surrounding_lines(lines.clone(), i);

        // find all numbers in the current line
        let numbers = find_numbers(line);
        // check each number to see if it has a symbol
        for number in numbers {
            if number.has_symbol(previous_line, line, next_line) {
                part_1 += number.value;
            }
        }

        // find all gears in the current line
        let gears = find_gears(line);
        // check each gear to see if it has a symbol
        for gear in gears {
            let numbers = gear.find_numbers(previous_line, line, next_line);
            if numbers.len() < 2 {
                continue;
            }
            let product = numbers.iter().fold(1, |acc, number| acc * number.value);
            part_2 += product;
        }
    }

    // output the sums
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_surrounding_lines(lines: Vec<&str>, index: usize) -> (Option<&str>, &str, Option<&str>) {
    let previous_line = match index {
        0 => None,
        _ => Some(lines.get(index - 1).map(|line| *line).unwrap()),
    };
    let current_line = lines.get(index).unwrap();
    let next_line = lines.get(index + 1).map(|line| *line);

    (previous_line, current_line, next_line)
}

#[derive(Eq, PartialEq, Hash)]
struct Number {
    value: i32,
    range: std::ops::Range<usize>,
}

impl Number {
    fn has_symbol(&self, previous_line: Option<&str>, current_line: &str, next_line: Option<&str>) -> bool {
        // create a range that is 1 character before and after the number
        let range = self.get_expanded_range(current_line.len());

        // check current line first
        if check_for_symbols(current_line, range.clone()) {
            return true;
        }

        // check previous line
        if let Some(previous_line) = previous_line {
            if check_for_symbols(previous_line, range.clone()) {
                return true;
            }
        }

        // check next line
        if let Some(next_line) = next_line {
            if check_for_symbols(next_line, range) {
                return true;
            }
        }

        false
    }

    fn get_expanded_range(&self, current_line_length: usize) -> std::ops::Range<usize> {
        let mut range = self.range.clone();
        range.start = range.start.saturating_sub(1);
        range.end = if range.end == current_line_length {
            range.end
        } else {
            range.end.add(1)
        };
        range
    }
}

fn check_for_symbols(line: &str, range: std::ops::Range<usize>) -> bool {
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

struct Gear {
    index: usize,
}

impl Gear {
    fn find_numbers(&self, previous_line: Option<&str>, current_line: &str, next_line: Option<&str>) -> Vec<Number> {
        let mut numbers = Vec::new();

        // find all numbers in the current line
        numbers.extend(find_numbers(current_line));

        // find all numbers on the previous line
        if let Some(previous_line) = previous_line {
            numbers.extend(find_numbers(previous_line));
        }

        // find all numbers on the next line
        if let Some(next_line) = next_line {
            numbers.extend(find_numbers(next_line));
        }

        // filter out numbers that are not in the range of the gear
        numbers.retain(|number| number.get_expanded_range(current_line.len()).contains(&self.index));

        numbers
    }
}

fn find_gears(line: &str) -> Vec<Gear> {
    let mut gears = Vec::new();

    for capture in Regex::new(r"[*]").unwrap().captures_iter(line) {
        if let Some(capture) = capture.get(0) {
            let index = capture.start();
            gears.push(Gear {
                index,
            });
        }
    }

    gears
}
