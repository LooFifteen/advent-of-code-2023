use std::collections::HashMap;
use std::fmt::Display;

const INPUT_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");

#[derive(Clone, Copy)]
enum Digit {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: &str = match self {
            Digit::One => "one",
            Digit::Two => "two",
            Digit::Three => "three",
            Digit::Four => "four",
            Digit::Five => "five",
            Digit::Six => "six",
            Digit::Seven => "seven",
            Digit::Eight => "eight",
            Digit::Nine => "nine"
        };
        write!(f, "{}", str)
    }
}

const DIGITS: [Digit; 9] = [
    Digit::One,
    Digit::Two,
    Digit::Three,
    Digit::Four,
    Digit::Five,
    Digit::Six,
    Digit::Seven,
    Digit::Eight,
    Digit::Nine
];

fn main() {
    // read input from file
    let input = std::fs::read_to_string(INPUT_FILE).expect("Failed to read input file.");

    // initialize sum
    let mut sum = 0;

    // iterate through each line
    for line in input.lines() {
        // find first occurrence of each digit
        let digits: HashMap<i32, Option<usize>> = DIGITS.iter().map(|d| {
            let digit = line.find(d.to_string().as_str());
            let number = line.find((*d as i32).to_string().as_str());
            // find the lowest index
            let index = compare_option(digit, number, std::cmp::min);
            (*d as i32, index)
        }).filter(|(_, index)| index.is_some()).collect();

        // get the digit with the lowest index
        let first = digits.iter().min_by_key(|(_, index)| index.unwrap()).unwrap().0;

        // do again for last digit
        let digits: HashMap<i32, Option<usize>> = DIGITS.iter().map(|d| {
            let digit = line.rfind(d.to_string().as_str());
            let number = line.rfind((*d as i32).to_string().as_str());
            // find the highest index
            let index = compare_option(digit, number, std::cmp::max);
            (*d as i32, index)
        }).filter(|(_, index)| index.is_some()).collect();

        // get the digit with the highest index
        let last = digits.iter().max_by_key(|(_, index)| index.unwrap()).unwrap().0;

        // form number from first and last
        let number = first * 10 + last;

        // add to sum
        sum += number;
    }

    // print sum
    println!("Sum: {}", sum);
}

fn compare_option<T: Ord, F>(a: Option<T>, b: Option<T>, mut function: F) -> Option<T>
    where F: FnMut(T, T) -> T {
    match (a, b) {
        (Some(a), Some(b)) => Some(function(a, b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None
    }
}
