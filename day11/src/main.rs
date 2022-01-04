use std::collections::HashSet;
use std::str;

fn is_password_legal(s: &Vec<char>) -> bool {
    let mut increasing_straight: bool = false;
    let mut pairs: HashSet<char> = HashSet::new();
    let mut last_char: u8 = 0;
    let mut last2_char: u8 = 0;
    for x in s {
        // return directly illegal if it contains ambiguous chars
        if *x == 'i' || *x == 'o' || *x == 'l' {
            return false;
        }
        // add to pairs if last and this is same
        if *x as u8 == last_char {
            pairs.insert(*x);
        }
        // check if there is a increasing straight
        if !increasing_straight {
            if *x as u8 - 1 == last_char && *x as u8 - 2 == last2_char {
                increasing_straight = true;
            }
        }
        last2_char = last_char;
        last_char = *x as u8;
    }
    if increasing_straight && pairs.len() > 1 {
        true
    } else {
        false
    }
}

fn part_one(s: &str) -> String {
    let mut pw: Vec<char> = s.chars().collect();
    while !is_password_legal(&pw) {
        // increment the string
        'genNewPW: for i in (0..pw.len()).rev() {
            pw[i] = ((pw[i] as u8) + 1) as char;
            if pw[i] as u8 > 122 {
                pw[i] = 'a';
            } else {
                break 'genNewPW;
            }
        }
    }
    pw.into_iter().collect()
}

fn part_two(s: &str) -> String {
    let mut pw: Vec<char> = s.chars().collect();
    for i in (0..pw.len()).rev() {
        pw[i] = ((pw[i] as u8) + 1) as char;
        if pw[i] as u8 > 122 {
            pw[i] = 'a';
        } else {
            break;
        }
    }
    let s = pw.into_iter().collect::<String>();
    part_one(&s)
}

fn main() {
    let instructions_one = "cqjxjnds" ;

    let answer_part_one = part_one(&instructions_one);

    let answer_part_two = part_two(&answer_part_one);

    println!("answer part 1: {}\nanswer part 2: {} ", answer_part_one, answer_part_two);
}