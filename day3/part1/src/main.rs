use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input file");

    let len = input.lines().next().unwrap().chars().count();
    let mut gammaStr = String::new();
    let mut epsilonStr = String::new();

    for n in 0..len {
        let mut zero_count = 0;
        let mut one_count = 0;

        for line in input.lines() {
            match line.chars().nth(n).unwrap() {
                '0' => zero_count += 1,
                '1' => one_count += 1,
                _ => panic!("Invalid character"),
            }
        }

        if zero_count > one_count {
            gammaStr += "0";
            epsilonStr += "1";
        }
        else {
            gammaStr += "1";
            epsilonStr += "0";
        }
    }

    let gamma = u32::from_str_radix(&gammaStr, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilonStr, 2).unwrap();
    println!("Gamma: {} Epsilon: {}", gamma, epsilon);
}
