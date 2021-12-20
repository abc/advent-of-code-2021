use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to read file.");

    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;

    for line in input.lines() {
        let command = line.split(' ').nth(0)
            .expect("Input was invalid");

        let magnitude:i32 = line.split(' ').nth(1)
            .expect("Input was invalid")
            .parse()
            .expect("Magnitude was not a valid integer");

        match command {
            "forward" => {
                position += magnitude;
                depth += aim * magnitude;
            },
            "down" => aim += magnitude,
            "up" => aim -= magnitude,
            _ => panic!("Invalid direction")
        }

        println!("Depth: {}  Position: {}, Aim: {}", depth, position, aim);
        println!("Product: {}", depth * position);
    }
}
