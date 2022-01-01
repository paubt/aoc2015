use std::fs;

fn part_one(s: &str) -> u32 {
    let size = 1000;
    // 2d array
    let mut light_grid: Vec<Vec<bool>> = vec![vec![false; size]; size];



    for line in s.split('\n') {

        let words: Vec<&str> = line.split(' ').collect();
        if words[0] == "toggle" {
            let from: Vec<&str> = words[1].split(',').collect();
            let from_x: usize = from[0].parse().unwrap();
            let from_y: usize = from[1].parse().unwrap();
            let to: Vec<&str> = words[3].split(',').collect();
            let to_x: usize = to[0].parse().unwrap();
            let to_y: usize = to[1].parse().unwrap();
            assert!( from_x < to_x);
            assert!( from_y < to_y);
            for x in from_x..to_x+1 {
                for y in from_y..to_y+1 {
                    if light_grid[x][y] {
                        light_grid[x][y] = false;
                    } else {
                        light_grid[x][y] = true;
                    }
                }
            }
        } else if words[1] == "on" {
            let from: Vec<&str> = words[2].split(',').collect();
            let from_x: usize = from[0].parse().unwrap();
            let from_y: usize = from[1].parse().unwrap();
            let to: Vec<&str> = words[4].split(',').collect();
            let to_x: usize = to[0].parse().unwrap();
            let to_y: usize = to[1].parse().unwrap();
            assert!( from_x < to_x);
            assert!( from_y < to_y);
            for x in from_x..to_x+1 {
                for y in from_y..to_y+1 {
                    light_grid[x][y] = true;
                }
            }
        } else {
            let from: Vec<&str> = words[2].split(',').collect();
            let from_x: usize = from[0].parse().unwrap();
            let from_y: usize = from[1].parse().unwrap();
            let to: Vec<&str> = words[4].split(',').collect();
            let to_x: usize = to[0].parse().unwrap();
            let to_y: usize = to[1].parse().unwrap();
            assert!(from_x < to_x);
            assert!(from_y < to_y);
            for x in from_x..to_x+1 {
                for y in from_y..to_y+1 {
                    light_grid[x][y] = false;
                }
            }
        }
    }
    let mut counter: u32 = 0;
    for line in light_grid {
        for element in line {
            if element {
                counter += 1;
            }
        }
    }
    counter
}


fn part_two(s: &str) -> u32 {
    let size = 1000;
    // 2d array
    let mut light_grid: Vec<Vec<u32>> = vec![vec![0; size]; size];

    for line in s.split('\n') {
        let words: Vec<&str> = line.split(' ').collect();
        if words[0] == "toggle" {
            let from: Vec<&str> = words[1].split(',').collect();
            let from_x: usize = from[0].parse().unwrap();
            let from_y: usize = from[1].parse().unwrap();
            let to: Vec<&str> = words[3].split(',').collect();
            let to_x: usize = to[0].parse().unwrap();
            let to_y: usize = to[1].parse().unwrap();
            assert!( from_x < to_x);
            assert!( from_y < to_y);
            for x in from_x..to_x+1 {
                for y in from_y..to_y+1 {
                    light_grid[x][y] += 2;
                }
            }
        } else if words[1] == "on" {
            let from: Vec<&str> = words[2].split(',').collect();
            let from_x: usize = from[0].parse().unwrap();
            let from_y: usize = from[1].parse().unwrap();
            let to: Vec<&str> = words[4].split(',').collect();
            let to_x: usize = to[0].parse().unwrap();
            let to_y: usize = to[1].parse().unwrap();
            assert!( from_x < to_x);
            assert!( from_y < to_y);
            for x in from_x..to_x+1 {
                for y in from_y..to_y+1 {
                    light_grid[x][y] += 1;
                }
            }
        } else {
            let from: Vec<&str> = words[2].split(',').collect();
            let from_x: usize = from[0].parse().unwrap();
            let from_y: usize = from[1].parse().unwrap();
            let to: Vec<&str> = words[4].split(',').collect();
            let to_x: usize = to[0].parse().unwrap();
            let to_y: usize = to[1].parse().unwrap();
            assert!(from_x < to_x);
            assert!(from_y < to_y);
            for x in from_x..to_x+1 {
                for y in from_y..to_y+1 {
                    if light_grid[x][y] > 0 {
                        light_grid[x][y] -= 1;
                    }
                }
            }
        }
    }
    let mut counter: u32 = 0;
    for line in light_grid {
        for element in line {
                counter += element;
        }
    }
    counter
}

fn main() {
    let instructions = fs::read_to_string("../dataSources/day6/input.csv")
        .expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);


    println!("answer part 1: {}\nanswer part 2: {} ", answer_part_one, answer_part_two);
}
