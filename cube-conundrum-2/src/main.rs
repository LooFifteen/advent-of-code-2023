const INPUT_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt");

#[derive(Debug)]
enum Colour {
    Red(u32),
    Green(u32),
    Blue(u32)
}

fn main() {
    // read input from file
    let input = std::fs::read_to_string(INPUT_FILE).expect("Failed to read input file.");

    // initialize sum
    let mut sum = 0;

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

        // iterate through each set
        'sets: for set in sets {
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
                    "red" => Colour::Red(number),
                    "green" => Colour::Green(number),
                    "blue" => Colour::Blue(number),
                    _ => panic!("Invalid colour.")
                };

                // check if number of each colour is valid
                if !match colour {
                    Colour::Red(number) => number <= 12,
                    Colour::Green(number) => number <= 13,
                    Colour::Blue(number) => number <= 14
                } {
                    // if not, set possible to false and break out of loop
                    possible = false;
                    break 'sets;
                }
            }
        }

        // if possible, add number to sum
        if possible {
            sum += number;
        }
    }

    // print sum
    println!("Sum: {}", sum);
}
