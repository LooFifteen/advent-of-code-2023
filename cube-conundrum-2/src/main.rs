use std::collections::HashMap;

const INPUT_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Colour {
    Red,
    Green,
    Blue
}

fn main() {
    // read input from file
    let input = std::fs::read_to_string(INPUT_FILE).expect("Failed to read input file.");

    // initialize sums
    let mut part_1 = 0;
    let mut part_2 = 0;

    // iterate through each line
    for line in input.lines() {
        // split line by colon
        let mut split = line.split(':');
        let name = split.next().unwrap();
        let sets = split.next().unwrap();

        // split name by space
        let mut name = name.split(' ');
        let _ = name.next().unwrap();
        let number: u32 = name.next().unwrap().parse().unwrap();

        // split sets by semi-colon
        let sets = sets.split(';');

        // initialize possible variable
        let mut possible = true;
        let mut minimum_required = HashMap::new();

        // iterate through each set
        for set in sets {
            // split set by comma
            let hands = set.split(',');

            // iterate through each hand
            for hand in hands {
                // split hand by space
                let mut hand = hand.split(' ');
                let _ = hand.next().unwrap();
                let number: u32 = hand.next().unwrap().parse().unwrap();
                let colour = hand.next().unwrap();

                // convert to Colour
                let colour = match colour {
                    "red" => Colour::Red,
                    "green" => Colour::Green,
                    "blue" => Colour::Blue,
                    _ => panic!("Invalid colour.")
                };

                // if the number of this colour is more than the minimum required, set the minimum required to this number
                if let Some(minimum_required) = minimum_required.get_mut(&colour) {
                    if number > *minimum_required {
                        *minimum_required = number;
                    }
                } else {
                    minimum_required.insert(colour, number);
                }

                // check if number of each colour is valid
                if possible && !match colour {
                    Colour::Red => number <= 12,
                    Colour::Green => number <= 13,
                    Colour::Blue => number <= 14
                } {
                    // if not, set possible to false and break out of loop
                    possible = false;
                }
            }
        }

        // multiply each minimum required number
        let mut power = 1;
        for (_, number) in minimum_required {
            power *= number;
        }
        part_2 += power;

        // if possible, add number to sum
        if possible {
            part_1 += number;
        }
    }

    // print sums
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
