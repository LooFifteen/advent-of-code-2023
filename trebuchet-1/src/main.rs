const INPUT_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");

fn main() {
    // read input from file
    let input = std::fs::read_to_string(INPUT_FILE).expect("Failed to read input file.");

    // initialize sum
    let mut sum = 0;

    // iterate through each line
    for line in input.lines() {
        // filter line to only include numbers
        let numbers: Vec<u32> = line
            .chars()
            .filter(|s| s.is_numeric())
            .map(|s| s.to_digit(10).unwrap())
            .collect();

        // get first and last number
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();

        // form number from first and last
        let number = first * 10 + last;

        // add to sum
        sum += number;
    }

    // print sum
    println!("Sum: {}", sum);
}
