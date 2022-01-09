use std::fs;

fn number_count(s: &str) -> i32 {
    let mut counter: i32 = 0;
    let mut temp_string: String = String::new();
    let mut minus: bool = false;
    let mut last_was_digit: bool = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            temp_string.push(c);
            last_was_digit = true;
        } else {
            if last_was_digit {
                if minus {
                    counter -= temp_string.parse::<i32>().unwrap();
                    temp_string.clear();
                    last_was_digit = false;
                } else {
                    counter += temp_string.parse::<i32>().unwrap();
                    temp_string.clear();
                    last_was_digit = false;
                }
            }
            if c == '-' {
                minus = true
            } else {
                minus = false
            }
        }

    }
    counter
}

fn part_one(s: &str) -> i32 {
    let mut counter: i32 = 0;
    for line in s.split('\n') {
        counter += number_count(line);
    }
    counter
}


fn part_two(s: &str) -> i32 {
    0
}


fn main() {
    let instructions = fs::read_to_string("../dataSources/day12/input.csv")
        .expect("read in file failed");

    let answer_part_one = part_one(&instructions);

    let answer_part_two = part_two(&instructions);

    println!("answer part 1: {}\nanswer part 2: {} ", answer_part_one, answer_part_two);
}