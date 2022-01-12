use std::fs;

fn part1(input: &str) -> i32 {
    // iterate over chars in the string
    let mut current_level = 0;
    for c in input.chars() {
        if c == '(' {
            current_level += 1;
        } else {
            current_level -= 1;
        }
    }
    current_level
}

fn part2(input: &str) -> usize {
    let mut current_level = 0;
    for (i, c) in input.chars().enumerate() {
        if current_level < 0 {
            return i;
        } else {
            if c == '(' {
                current_level += 1;
            } else {
                current_level -= 1;
            }
        }
    }
    0
}

fn main() {
    // read in file into string and transform result
    let instructions =
        fs::read_to_string("../dataSources/day1/input.csv").expect("read in file failed");
    let answer_part_one = part1(&instructions);
    let answer_part_two = part2(&instructions);
    println!("this is a test for rust on atom ");

    println!(
        "end level is {}\nfirst basement entrance at {}",
        answer_part_one, answer_part_two
    );
}
