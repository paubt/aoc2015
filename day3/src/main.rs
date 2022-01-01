use std::fs;
use std::collections::HashMap;

enum HowsTurn {
    Santa,
    Robot,
}

fn part_one(input: &str) -> usize {
    // create a hash map for position -> occurrence
    let mut map: HashMap<(i32,i32), u32> = HashMap::new();
    // position x and y with zero as start
    let mut x= 0;
    let mut y = 0;
    // add the start to the map
    {
        let temp_count = map.entry((x,y)).or_insert(0);
        *temp_count += 1;
    }
    // iterate over chars/directions in the input
    for c in input.chars() {
        // update the position based on the direction given
        if c == '>' {
            x += 1;
        } else if c == '<' {
            x -= 1;
        } else if c == '^' {
            y += 1;
        } else if c == 'v' {
            y -= 1;
        } else {
            panic!("unexpected char in the input");
        }
        // add the to the updated position one in the map
        let temp_count = map.entry((x,y)).or_insert(0);
        *temp_count += 1;
    }
    map.len()
}

fn part_two(input: &str) -> usize {
    // create a hash map for position -> occurrence
    let mut map: HashMap<(i32,i32), u32> = HashMap::new();
    // position x and y with zero as start
    let mut santa_x= 0;
    let mut santa_y = 0;
    let mut robot_x= 0;
    let mut robot_y = 0;
    // add the start to the map
    {
        let temp_count = map.entry((santa_x,santa_x)).or_insert(0);
        *temp_count += 2;
    }
    // santa starts
    let mut turn = HowsTurn::Santa;
    // iterate over chars/directions in the input
    for c in input.chars() {
        match turn {
            HowsTurn::Santa=> {
                // update the position based on the direction given
                if c == '>' {
                    santa_x += 1;
                } else if c == '<' {
                    santa_x -= 1;
                } else if c == '^' {
                    santa_y += 1;
                } else if c == 'v' {
                    santa_y -= 1;
                } else {
                    panic!("unexpected char in the input");
                }
                // add the to the updated position one in the map
                let temp_count = map.entry((santa_x,santa_y)).or_insert(0);
                *temp_count += 1;
                // change turn to robot
                turn = HowsTurn::Robot
            }
            HowsTurn::Robot=> {
                // update the position based on the direction given
                if c == '>' {
                    robot_x += 1;
                } else if c == '<' {
                    robot_x -= 1;
                } else if c == '^' {
                    robot_y += 1;
                } else if c == 'v' {
                    robot_y -= 1;
                } else {
                    panic!("unexpected char in the input");
                }
                // add the to the updated position one in the map
                let temp_count = map.entry((robot_x,robot_y)).or_insert(0);
                *temp_count += 1;
                // change turn to santa
                turn = HowsTurn::Santa
            }
        }

    }
    map.len()
}

fn main() {
    let instructions = fs::read_to_string("../dataSources/day3/input1.csv")
        .expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!("answer part 1: {}\nanswer part 2: {} ", answer_part_one, answer_part_two);
}
