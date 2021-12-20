use std::fs;

fn main() {
    let filename = "input.txt";

    println!("Reading file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Could not read the input file");

    let mut previous_num:Option<i32> = None;
    let mut count = 0;

    for line in contents.lines() {
        let num:i32 = line.trim().parse().expect("Invalid number");
        let msg = get_msg(previous_num, num);
        previous_num = Some(num);
        println!("{} ({})", num, msg);

        if msg == "increased"
        {
            count += 1;
        }
    }

    println!("Increased {} times.", count);
}

fn get_msg(previous_num: Option<i32>, num: i32) -> String {
    match previous_num {
        Some(x) if x > num => String::from("decreased"),
        Some(x) if x < num => String::from("increased"),
        Some(x) if x == num => String::from("unchanged"),
        Some(_) => panic!("An error occurred when comparing the numbers"),
        None => String::from("N/A - no previous measurement"),
    }
}