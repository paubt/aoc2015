use std::fs;

fn part_two(s: &str) -> Option<u32> {
    'aunt_loop: for aunt in s.split('\n') {
        if aunt.is_empty() {
            break;
        }
        let aunt_split: Vec<&str> = aunt.split(" ").collect::<Vec<&str>>();
        let mut aunt_iter = aunt_split.iter().skip(1);
        let aunt_number = aunt_iter.next().unwrap();
        let aunt_number = aunt_number[0..aunt_number.len() - 1]
            .parse::<u32>()
            .unwrap();
        let mut match_counter = 0;
        'compound_loop: while let Some(s) = aunt_iter.next() {
            let checked_against: u32;
            if *s == "children:" {
                checked_against = 3;
            } else if *s == "cats:" {
                checked_against = 7;
            } else if *s == "samoyeds:" {
                checked_against = 2;
            } else if *s == "pomeranians:" {
                checked_against = 3;
            } else if *s == "akitas:" {
                checked_against = 0;
            } else if *s == "vizslas:" {
                checked_against = 0;
            } else if *s == "goldfish:" {
                checked_against = 5;
            } else if *s == "trees:" {
                checked_against = 3;
            } else if *s == "cars:" {
                checked_against = 2;
            } else if *s == "perfumes:" {
                checked_against = 1;
            } else {
                panic!("one s (this) not covered by if's above");
            }
            // get the next item that is the amount of the catergory beforehead
            let n: u32 = match aunt_iter.next() {
                Some(n) => n.split(',').nth(0).unwrap().parse::<u32>().unwrap(),
                None => panic!("probably uncomlete pair of compound and amount"),
            };
            // ######## changes from part_one start
            if *s == "cats:" || *s == "trees:" {
                if checked_against < n {
                    match_counter = match_counter + 1;
                    continue 'compound_loop;
                } else {
                    continue 'aunt_loop;
                }
            } else if *s == "pomeranians:" || *s == "goldfish:" {
                if checked_against > n {
                    match_counter = match_counter + 1;
                    continue 'compound_loop;
                } else {
                    continue 'aunt_loop;
                }
            } else if checked_against == n {
                // check the next compund
                match_counter = match_counter + 1;
                continue 'compound_loop;
            } else {
                // try next aunt
                continue 'aunt_loop;
            }
        }
        // if we arrive here we found a aunt that matches
        //print!("aunt {} matched with: {}\n", aunt_number, aunt);
        return Some(aunt_number);
    }

    None
}

fn part_one(s: &str) -> Option<u32> {
    'aunt_loop: for aunt in s.split('\n') {
        if aunt.is_empty() {
            break;
        }
        let aunt_split: Vec<&str> = aunt.split(" ").collect::<Vec<&str>>();
        let mut aunt_iter = aunt_split.iter().skip(1);
        let aunt_number = aunt_iter.next().unwrap();
        let aunt_number = aunt_number[0..aunt_number.len() - 1]
            .parse::<u32>()
            .unwrap();
        let mut match_counter = 0;
        'compound_loop: while let Some(s) = aunt_iter.next() {
            let checked_against: u32;
            if *s == "children:" {
                checked_against = 3;
            } else if *s == "cats:" {
                checked_against = 7;
            } else if *s == "samoyeds:" {
                checked_against = 2;
            } else if *s == "pomeranians:" {
                checked_against = 3;
            } else if *s == "akitas:" {
                checked_against = 0;
            } else if *s == "vizslas:" {
                checked_against = 0;
            } else if *s == "goldfish:" {
                checked_against = 5;
            } else if *s == "trees:" {
                checked_against = 3;
            } else if *s == "cars:" {
                checked_against = 2;
            } else if *s == "perfumes:" {
                checked_against = 1;
            } else {
                panic!("one s (this) not covered by if's above");
            }
            // get the next item that is the amount of the catergory beforehead
            let n: u32 = match aunt_iter.next() {
                Some(n) => n.split(',').nth(0).unwrap().parse::<u32>().unwrap(),
                None => panic!("probably uncomlete pair of compound and amount"),
            };
            if checked_against == n {
                // check the next compund
                match_counter = match_counter + 1;
                continue 'compound_loop;
            } else {
                // try next aunt
                continue 'aunt_loop;
            }
        }
        // if we arrive here we found a aunt that matches
        //print!("aunt {} matched with: {}\n", aunt_number, aunt);
        return Some(aunt_number);
    }

    None
}

fn main() {
    let instructions =
        // using atom-script package
        // fs::read_to_string("dataSources/day13/input.csv").expect("read in file failed");
        // using the cargo in terminal
        fs::read_to_string("../dataSources/day16/input.csv").expect("read in file failed");

    let answer_part_one = part_one(&instructions).unwrap();

    let answer_part_two = part_two(&instructions).unwrap();

    println!(
        "answer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}
