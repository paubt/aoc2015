use std::fs;

#[derive(Debug)]
enum InstructionSet {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(i32),
    Jie(char, i32),
    Jio(char, i32),
}

fn part_one(instructions: &str) -> u32 {
    use InstructionSet::*;
    let instructions: Vec<InstructionSet> = instructions
        .lines()
        .map(|l| {
            // not first tow chars not "ji" -> ergo unary operation
            let s = l.split(" ").collect::<Vec<&str>>();
            return match s[0] {
                "hlf" => Hlf(s[1].chars().next().unwrap()),
                "tpl" => Tpl(s[1].chars().next().unwrap()),
                "inc" => Inc(s[1].chars().next().unwrap()),
                "jmp" => Jmp(s[1].parse::<i32>().unwrap()),
                "jie" => Jie(s[1].chars().next().unwrap(), s[2].parse::<i32>().unwrap()),
                "jio" => Jio(s[1].chars().next().unwrap(), s[2].parse::<i32>().unwrap()),
                _ => panic!("no match for instruction"),
            };
        })
        .collect();

    // let mut instrucion_counter: u32 = 0;
    let mut reg_a: u32 = 0;
    let mut reg_b: u32 = 0;
    let mut exec_position: i32 = 0;
    //println!("{:?}", instructions);

    'outta: while 0 <= exec_position && exec_position < instructions.len() as i32 {
        /*
        {
            println!(
                "counter:{} rega|b={}|{} exec_pos={} {:?}",
                instrucion_counter,
                reg_a,
                reg_b,
                exec_position,
                instructions[exec_position as usize]
            );
            instrucion_counter = instrucion_counter + 1;
        }
        */
        match instructions[exec_position as usize] {
            Hlf(r) => match r {
                'a' => reg_a = reg_a / 2,
                'b' => reg_b = reg_b / 2,
                _ => panic!("hlf panic"),
            },
            Tpl(r) => match r {
                'a' => reg_a = reg_a * 3,
                'b' => reg_b = reg_b * 3,
                _ => panic!("tpl panic"),
            },
            Inc(r) => match r {
                'a' => reg_a = reg_a + 1,
                'b' => reg_b = reg_b + 1,
                _ => panic!("inc panic"),
            },
            Jmp(d) => {
                exec_position = exec_position + d;
                continue 'outta;
            }
            Jie(r, d) => match r {
                'a' => {
                    if reg_a % 2 == 0 {
                        exec_position = exec_position + d;
                        continue 'outta;
                    }
                }
                'b' => {
                    if reg_b % 2 == 0 {
                        exec_position = exec_position + d;
                        continue 'outta;
                    }
                }
                _ => panic!("jie panic"),
            },
            Jio(r, d) => match r {
                'a' => {
                    if reg_a == 1 {
                        exec_position = exec_position + d;
                        continue 'outta;
                    }
                }
                'b' => {
                    if reg_b == 1 {
                        exec_position = exec_position + d;
                        continue 'outta;
                    }
                }
                _ => panic!("jio panic"),
            },
        }

        exec_position = exec_position + 1;
    }

    reg_b
}

fn part_two(instructions: &str) -> u32 {
    use InstructionSet::*;
    let instructions: Vec<InstructionSet> = instructions
        .lines()
        .map(|l| {
            // not first tow chars not "ji" -> ergo unary operation
            let s = l.split(" ").collect::<Vec<&str>>();
            return match s[0] {
                "hlf" => Hlf(s[1].chars().next().unwrap()),
                "tpl" => Tpl(s[1].chars().next().unwrap()),
                "inc" => Inc(s[1].chars().next().unwrap()),
                "jmp" => Jmp(s[1].parse::<i32>().unwrap()),
                "jie" => Jie(s[1].chars().next().unwrap(), s[2].parse::<i32>().unwrap()),
                "jio" => Jio(s[1].chars().next().unwrap(), s[2].parse::<i32>().unwrap()),
                _ => panic!("no match for instruction"),
            };
        })
        .collect();

    // let mut instrucion_counter: u32 = 0;
    let mut reg_a: u32 = 1;
    let mut reg_b: u32 = 0;
    let mut exec_position: i32 = 0;
    //println!("{:?}", instructions);

    'outta: while 0 <= exec_position && exec_position < instructions.len() as i32 {
        /*
        {
            println!(
                "counter:{} rega|b={}|{} exec_pos={} {:?}",
                instrucion_counter,
                reg_a,
                reg_b,
                exec_position,
                instructions[exec_position as usize]
            );
            instrucion_counter = instrucion_counter + 1;
        }
        */
        match instructions[exec_position as usize] {
            Hlf(r) => match r {
                'a' => reg_a = reg_a / 2,
                'b' => reg_b = reg_b / 2,
                _ => panic!("hlf panic"),
            },
            Tpl(r) => match r {
                'a' => reg_a = reg_a * 3,
                'b' => reg_b = reg_b * 3,
                _ => panic!("tpl panic"),
            },
            Inc(r) => match r {
                'a' => reg_a = reg_a + 1,
                'b' => reg_b = reg_b + 1,
                _ => panic!("inc panic"),
            },
            Jmp(d) => {
                exec_position = exec_position + d;
                continue 'outta;
            }
            Jie(r, d) => match r {
                'a' => {
                    if reg_a % 2 == 0 {
                        exec_position = exec_position + d;
                        continue 'outta;
                    }
                }
                'b' => {
                    if reg_b % 2 == 0 {
                        exec_position = exec_position + d;
                        continue 'outta;
                    }
                }
                _ => panic!("jie panic"),
            },
            Jio(r, d) => match r {
                'a' => {
                    if reg_a == 1 {
                        exec_position = exec_position + d;
                        continue 'outta;
                    }
                }
                'b' => {
                    if reg_b == 1 {
                        exec_position = exec_position + d;
                        continue 'outta;
                    }
                }
                _ => panic!("jio panic"),
            },
        }

        exec_position = exec_position + 1;
    }

    reg_b
}

fn main() {
    let instructions: String =
        fs::read_to_string("../dataSources/day23/input.csv").expect("read in file failed");

    let answer_part_one = part_one(&instructions);

    let answer_part_two = part_two(&instructions);

    println!(
        "answer part 1: {}\nanswer part 2: {}",
        answer_part_one, answer_part_two
    );
}
