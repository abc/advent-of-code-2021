use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input file");

    for line in input.lines() {
        println!("{}", line.chars().nth(0).expect("Invalid string"));
    }
}
