use std::fs;

fn literal_string_length(s: &str) -> i32 {
    s.len() as i32
}

fn in_memory_string_length(s: &str) -> i32 {
    let mut counter: i32 = 0;
    let mut c1 = s.char_indices().peekable();
    // skip the first char
    c1.next();
    'overChars: while let Some((_,ch)) = c1.next() {
        if ch == '\\' {
            let nc = match c1.next() {
                Some((_, nc)) => nc,
                None => panic!("error in next"),
            };
            if nc == '\\' || nc == '\"' {
                counter += 1;
                continue 'overChars;
            } else {
                // skip the next to aswell cause its a 2 hex digit ascii code
                c1.next();
                c1.next();
                counter += 1;
                continue 'overChars;
            }
        } else if ch == '\"' {
            break 'overChars;
        }
        counter += 1;
    }
    counter
}

fn encoded_string_length(s: &str) -> i32 {
    let mut counter: i32 = 2;
    let mut c1 = s.char_indices();
    'forChar: while let Some((_,ch)) = c1.next() {
        if '\"' == ch || '\\' == ch {
            counter += 2;
        } else {
            counter += 1;
        }
    }
    counter
}

fn diff_off_lengths_one(s: &str) -> i32 {
   literal_string_length(s) - in_memory_string_length(s)
}

fn diff_off_lengths_two(s: &str) -> i32 {
    encoded_string_length(s) - literal_string_length(s)
}

fn part_one(s: &str) -> i32 {
    let mut counter: i32 = 0;
    for line in s.split('\n') {
        counter += diff_off_lengths_one(line);
    }
    counter
}

fn part_two(s: &str) -> i32 {
    let mut counter: i32 = 0;
    for line in s.split('\n') {
        counter += diff_off_lengths_two(line);
    }
    counter
}

fn main() {
    let instructions1 = fs::read_to_string("../dataSources/day8/input.csv")
        .expect("read in file failed");

    let answer_part_one = part_one(&instructions1);
    let answer_part_two = part_two(&instructions1);

    println!("answer part 1: {}\nanswer part 2: {} ", answer_part_one, answer_part_two);
}