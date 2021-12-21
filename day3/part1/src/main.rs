use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read input file");

    let len = input.lines().next().unwrap().chars().count();
    let mut gamma:u32 = 0;

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

        gamma = gamma << 1;

        if one_count > zero_count {
            gamma += 1;
        }
    }

    let epsilon = !gamma;
    println!("Gamma: {} Epsilon: {}", gamma, epsilon);
}
