use std::fs;

fn main() {
    let filename = "input.txt";

    println!("Reading file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Could not read the input file");

    let mut num_x:Option<i32> = None;
    let mut num_y:Option<i32> = None;
    let mut num_z:Option<i32> = None;
    let mut previous_sum:Option<i32> = None;
    let mut count = 0;

    for line in contents.lines() {
        let num:i32 = line.trim().parse().expect("Invalid number");

        num_z = num_y;
        num_y = num_x;
        num_x = Some(num);
        
        let sum = match (num_x, num_y, num_z) {
            (Some(x), Some(y), Some(z)) => x + y + z,
            (_, _, _) => continue,
        };

        let msg = match previous_sum {
            Some(x) if sum > x => "increased",
            Some(x) if sum < x => "decreased",
            Some(x) if sum == x => "unchanged",
            Some(_) => panic!("Invalid"),
            None => "N/A - no previous sum",
        };

        previous_sum = Some(sum);

        if msg.eq("increased")
        {
            count += 1;
        }

        println!("{} ({})", sum, msg);
    }

    println!("Increased {} times.", count);
}