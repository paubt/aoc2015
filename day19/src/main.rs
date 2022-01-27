use std::collections::HashSet;
use std::fs;

fn part_one(s: &str) -> usize {
    let start_mol = "CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgArCaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCaSiRnMgArFYSiRnFYCaFArSiThCaSiThPBPTiMgArCaPRnSiAlArPBCaCaSiRnFYSiThCaRnFArArCaCaSiRnPBSiRnFArMgYCaCaCaCaSiThCaCaSiAlArCaCaSiRnPBSiAlArBCaCaCaCaSiThCaPBSiThPBPBCaSiRnFYFArSiThCaSiRnFArBCaCaSiRnFYFArSiThCaPBSiThCaSiRnPMgArRnFArPTiBCaPRnFArCaCaCaCaSiRnCaCaSiRnFYFArFArBCaSiThFArThSiThSiRnTiRnPMgArFArCaSiThCaPBCaSiRnBFArCaCaPRnCaCaPMgArSiRnFYFArCaSiThRnPBPMgAr";
    let mut possibleas_mol: HashSet<String> = HashSet::new();
    for line in s.lines() {
        let line = line.split(' ').collect::<Vec<&str>>();
        let p_match: Vec<(usize, &str)> = start_mol.match_indices(line[0]).collect();
        //break;
        for (index, pattern) in p_match {
            let new_mol: &str = &format!(
                "{}{}{}",
                &start_mol[0..index],
                line[2],
                &start_mol[index + pattern.len()..start_mol.len()]
            );
            possibleas_mol.insert(new_mol.to_string());
        }
    }
    possibleas_mol.iter().count()
}

fn part_two(s: &str) -> u32 {
    // saw some solution on Reddit but was only simple greedy reduction
    // could probably be made with A*
    // this expands from e to the target molecule
    // a score will guide the search -> a priority queue
    //
    0
}

fn main() {
    let instructions =
    // using atom-script package
    // fs::read_to_string("dataSources/day13/input.csv").expect("read in file failed");
    // using the cargo in terminal
    fs::read_to_string("../dataSources/day19/input.csv").expect("read in file failed");

    // change to compute part1
    let answer_part_one = part_one(&instructions);

    let answer_part_two = part_two(&instructions);

    println!(
        "answer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}
