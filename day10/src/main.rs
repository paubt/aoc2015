use std::fs;

fn part_one_two(s: &str, n: u32) -> u32 {
    // number of iterations
    // transform string lo vector of int
    let mut v: Vec<u32> = vec![];
    // insert chars at int into the vec from the input string
    for c in s.chars() {
        match c.to_digit(10) {
            Some(o) => v.push(o),
            None => panic!["input string chars to int failed!!!"],
        }
    }
    for _ in 0..n {
        let mut new: Vec<u32> = vec![];
        // last type
        let mut amount_last_int: u32 = v[0];
        let mut count_last_int: u32 = 0;
        for x in v {
            if x == amount_last_int {
                count_last_int += 1;
            } else {
                new.push(count_last_int);
                new.push(amount_last_int);
                amount_last_int = x;
                count_last_int = 1;
            }
        }
        new.push(count_last_int);
        new.push(amount_last_int);
        v = new;
    }
    v.len() as u32
}

fn main() {
    let instructions = fs::read_to_string("../dataSources/day10/input.csv")
        .expect("read in file failed");

    let answer_part_one = part_one_two(&instructions, 40);
    let answer_part_two = part_one_two(&instructions, 50);

    println!("answer part 1: {}\nanswer part 2: {} ", answer_part_one, answer_part_two);
}