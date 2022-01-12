use std::fs;

struct Reindeer {
    name: String,
    speed: i32,
    run_time: i32,
    rest_time: i32,
}

fn part_one(s: &str) -> i32 {
    for line in s.split('\n').collect::<Vec<&str>>() {
        let x: Vec<&str> = line.split(' ').collect();
    }

    0
}

fn part_two(s: &str) -> i32 {
    0
}

fn main() {
    let instructions =
        // using atom-script package
        // fs::read_to_string("dataSources/day13/input.csv").expect("read in file failed");
        // using the cargo in terminal
        fs::read_to_string("../dataSources/day13/input.csv").expect("read in file failed");

    let answer_part_one = part_one(&instructions);

    let answer_part_two = part_two(&instructions);

    println!(
        "answer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}
