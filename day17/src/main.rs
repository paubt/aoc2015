use std::{fs, vec};

struct ConNode {
    individual_size: u32,
    accumulated_size: u32,
    unused_containers: Vec<u32>,
    child_containers: Vec<ConNode>,
}

fn part_one(s: &str) -> u32 {
    let unused_containers: Vec<&str> = s.lines().collect();
    let t: Vec<u32> = unused_containers
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    print!("{:?}", t);
    0
}

fn part_two(s: &str) -> u32 {
    0
}

fn main() {
    let instructions =
    // using atom-script package
    // fs::read_to_string("dataSources/day13/input.csv").expect("read in file failed");
    // using the cargo in terminal
    fs::read_to_string("../dataSources/day16/input.csv").expect("read in file failed");

    let answer_part_one = part_one(&instructions);

    let answer_part_two = part_two(&instructions);

    println!(
        "answer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}

fn add_one(i: i32) -> i32 {
    i + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_add_one() {
        let t: i32 = 1;
        let s: i32 = add_one(t);
        assert_eq!(s, 2);
    }
}
