use std::fs;
use std::collections::HashSet;

enum NiceNaughty {
    Nice,
    Naughty,
}

fn part_one(input: &str) -> u32 {
    // helper function nice or naughty only valid for part1
    fn nice_or_naughty(s: &str) -> NiceNaughty {
        // used by twice and forbidden helper
        let mut last_character : char = '0';
        // 3 nice string conditions
        let mut twice_condition = false;
        // let mut forbidden_condition = false;
        // vowel stuff
        let mut vowel_counter : u32 = 0;
        fn check_vowel(c: &char) -> bool {
            if *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u' {
                return true
            }
            false
        }
        // twice stuff
        fn check_twice(c: &char, last: &char) -> bool{
            if c == last {
                return true
            }
            false
        }
        // forbidden stuff
        // set for forbidden combinations
        let mut forbidden_combinations : HashSet<String> = HashSet::new();
        forbidden_combinations.insert("ab".to_string());
        forbidden_combinations.insert("cd".to_string());
        forbidden_combinations.insert("pq".to_string());
        forbidden_combinations.insert("xy".to_string());
        let forbidden_combinations : HashSet<String> = forbidden_combinations;
        assert_eq!(forbidden_combinations.contains("ab"), true);
        assert_eq!(forbidden_combinations.contains("ls"), false);
        // main check loop
        for c in s.chars() {
            if vowel_counter < 3 {
                match check_vowel(&c) {
                    true => vowel_counter += 1,
                    false => (),
                }
            }
            if !(twice_condition) {
                match check_twice(&c, &last_character) {
                    true => twice_condition = true,
                    false => (),
                }
            }
            let last_two_chars = format!("{}{}", last_character, c);
            match forbidden_combinations.contains(&last_two_chars) {
                true => return NiceNaughty::Naughty,
                false => (),
            }
            last_character = c;
        }
        // final nice check
        return if vowel_counter > 2 && twice_condition {
            NiceNaughty::Nice
        } else {
            NiceNaughty::Naughty
        }
    }

    let mut nice_counter : u32 = 0;
    //split the string
    for i in  input.split('\n') {
        match nice_or_naughty(i) {
            NiceNaughty::Nice => nice_counter += 1,
            NiceNaughty::Naughty => (),
        }
    }
    nice_counter
}

fn part_two(input: &str) -> u32 {
    // for 2 same pairs
    // use hashMap
    // if same char (aa, jj) only add if 2 second last is not same
    // (210|aaa is not okay)(210|hbb is okay)
    // if already exist increment else input 1
    // in end iterate over hashmap and condition true if one with 2 exists

    // condition 2
    // track last and last-last (last of last) char (210|njk with k the current char)
    // if char at 2 == char at 1 -> condition true

    // if both true => Nice

    fn nice_or_naughty(s: &str) -> NiceNaughty {
        print!("{}", s);
        NiceNaughty::Naughty
    }

    let mut nice_counter : u32 = 0;
    //split the string
    for i in  input.split('\n') {
        match nice_or_naughty(i) {
            NiceNaughty::Nice => nice_counter += 1,
            NiceNaughty::Naughty => (),
        }
    }
    nice_counter
}

fn main() {
    let instructions = fs::read_to_string("../dataSources/day5/input.csv")
        .expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!("answer part 1: {}\nanswer part 2: {} ", answer_part_one, answer_part_two);
}
