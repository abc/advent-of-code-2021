use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to read file.");

    let mut depth = 0;
    let mut position = 0;

    for line in input.lines() {
        let command = line.split(' ').nth(0)
            .expect("Input was invalid");

        let magnitude:i32 = line.split(' ').nth(1)
            .expect("Input was invalid")
            .parse()
            .expect("Magnitude was not a valid integer");

        match command {
            "forward" => position += magnitude,
            "down" => depth += magnitude,
            "up" => depth -= magnitude,
            _ => panic!("Invalid direction")
        }

        println!("Depth: {}  Position: {}", depth, position);
        println!("Product: {}", depth * position);
    }
}
