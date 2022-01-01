use std::fs;
use std::collections::HashMap;

fn part_one(s: &str) -> u16 {
    // hashmap to store the the wire identifier and the corresponding value
    let mut wires: HashMap<String, u16> = HashMap::new();
    //
    let full_circuit_instructions: Vec<&str> = s.split('\n').collect();
    let mut circuit_instructions: Vec<Vec<&str>> = Vec::new();
    for i in full_circuit_instructions.iter() {
        let g: Vec<&str>= (*i.split(' ').collect::<Vec<&str>>()).to_vec();
        circuit_instructions.push(g);
    }

    'lv0: while !circuit_instructions.is_empty() {
        'lv1: for i in 0..circuit_instructions.len() {
            // found a declarative statement -> add to wires
            if circuit_instructions[i].len() == 3 {
                // check that the new key to be added doesnt exists already
                assert!(!wires.contains_key(circuit_instructions[i][2]));
                // check if the left value is a literal or a variable
                let r: u16 = match circuit_instructions[i][0].parse() {
                    Ok(t) => t,
                    Err(_) => {
                        if wires.contains_key(circuit_instructions[i][0]) {
                            *wires.get(circuit_instructions[i][0]).unwrap()
                        } else {
                            continue 'lv1;
                        }
                    },
                };

                wires.insert(circuit_instructions[i][2].to_string(), r);
                circuit_instructions.swap_remove(i);
                // removes the instruction with the last one and continues with the outer loop
                // to account for the downsizing of the array
                continue 'lv0;
            }
            // a unary statement aka negation -> check if input wire is in wires
            if circuit_instructions[i].len() == 4 {
                if wires.contains_key(circuit_instructions[i][1]) {
                    let v: u16 = !(*wires.get(circuit_instructions[i][1]).unwrap());
                    wires.insert(circuit_instructions[i][3].to_string(), v);
                    circuit_instructions.swap_remove(i);
                    continue 'lv0;
                } else {
                    continue 'lv1;
                }
            }
            // a binary operator
            if circuit_instructions[i].len() == 5 {
                // check that left operator exists in wires aka already derived of other instructions
                if wires.contains_key(circuit_instructions[i][0]) {
                    if circuit_instructions[i][1] == "LSHIFT" {
                        let l: u16 = *wires.get(circuit_instructions[i][0]).unwrap();
                        let r: usize =  circuit_instructions[i][2].parse().unwrap();
                        let v: u16 = l << r;
                        wires.insert(circuit_instructions[i][4].to_string(), v);
                        circuit_instructions.swap_remove(i);
                        continue 'lv0;
                    } else if circuit_instructions[i][1] == "RSHIFT" {
                        let l: u16 = *wires.get(circuit_instructions[i][0]).unwrap();
                        let r: usize =  circuit_instructions[i][2].parse().unwrap();
                        let v: u16 = l >> r;
                        wires.insert(circuit_instructions[i][4].to_string(), v);
                        circuit_instructions.swap_remove(i);
                        continue 'lv0;
                    // check if the rigth operant is in wires to do the binary operation AND or OR
                    } else if wires.contains_key(circuit_instructions[i][2]) {
                        if circuit_instructions[i][1] == "AND" {
                            let l: u16 = *wires.get(circuit_instructions[i][0]).unwrap();
                            let r: u16 = *wires.get(circuit_instructions[i][2]).unwrap();
                            let v: u16 = l & r;
                            wires.insert(circuit_instructions[i][4].to_string(), v);
                            circuit_instructions.swap_remove(i);
                            continue 'lv0;
                        } else if circuit_instructions[i][1] == "OR" {
                            let l: u16 = *wires.get(circuit_instructions[i][0]).unwrap();
                            let r: u16 = *wires.get(circuit_instructions[i][2]).unwrap();
                            let v: u16 = l | r;
                            wires.insert(circuit_instructions[i][4].to_string(), v);
                            circuit_instructions.swap_remove(i);
                            continue 'lv0;
                        }
                    }
                }
                // check if the left operand is a literal

                match circuit_instructions[i][0].parse::<u16>() {
                    Ok(l) => {
                        if wires.contains_key(circuit_instructions[i][2])
                            && circuit_instructions[i][1] == "AND" {
                            let r: u16 = *wires.get(circuit_instructions[i][2]).unwrap();
                            let v: u16 = l & r;
                            wires.insert(circuit_instructions[i][4].to_string(), v);
                            circuit_instructions.swap_remove(i);
                            continue 'lv0;
                        }
                    }
                    Err(_) => (),
                };
            }
        }
    }
    *wires.get("a").unwrap()
}

fn part_two(s: &str) -> u16 {
    part_one(s)
}

fn main() {
    let instructions1 = fs::read_to_string("../dataSources/day7/input1.csv")
        .expect("read in file failed");

    let instructions2 = fs::read_to_string("../dataSources/day7/input2.csv")
        .expect("read in file failed");

    let answer_part_one = part_one(&instructions1);
    let answer_part_two = part_two(&instructions2);

    println!("answer part 1: {}\nanswer part 2: {} ", answer_part_one, answer_part_two);
}