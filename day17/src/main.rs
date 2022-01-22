use std::collections::HashSet;
use std::collections::VecDeque;
use std::{fs, vec};

#[derive(Debug, Clone, Copy)]
struct Contaier {
    id: u32,
    size: u32,
}

impl Contaier {
    fn new(id: u32, size: u32) -> Contaier {
        Contaier { id, size }
    }
}

#[derive(Debug, Clone)]
struct ConNode {
    individual_size: Contaier,
    accumulated_size: u32,
    accumulated_container_ids: Vec<u32>,
    unused_containers: Vec<Contaier>,
    child_containers: Vec<ConNode>,
}

impl ConNode {
    fn new(
        individual_size: Contaier,
        accumulated_size: u32,
        accumulated_container_ids: Vec<u32>,
        unused_containers: Vec<Contaier>,
    ) -> ConNode {
        ConNode {
            individual_size,
            accumulated_size,
            accumulated_container_ids,
            unused_containers,
            child_containers: vec![],
        }
    }
}

fn part_one(s: &str) -> u32 {
    let mut final_options: HashSet<Vec<u32>> = HashSet::new();
    let mut counter: u32 = 0;
    // sub
    fn create_subtrees(
        n: &mut ConNode,
        size_limit: u32,
        f_o: &mut HashSet<Vec<u32>>,
        counter: &mut u32,
        queue: &mut VecDeque<ConNode>,
    ) {
        *counter = *counter + 1;
        if *counter % 1000000 == 0 {
            println!("{} {:?}", *counter, n.accumulated_container_ids);
        }
        // go through all the options/ unused containers and check if we could add them
        /*
        let viable_options: Vec<u32> = n
            .unused_containers
            .iter()
            .filter(|s| *s + n.accumulated_size <= size_limit)
            .cloned()
            .collect();
        */
        // check if this equals size_limit to add it as a final viable option
        if n.accumulated_size == size_limit {
            let mut t = n.accumulated_container_ids.clone();
            t.sort();
            let t = t.drain(1..).collect();
            f_o.insert(t);
        }

        let c: Vec<ConNode> = n
            .unused_containers
            .iter()
            .filter(|s| s.size + n.accumulated_size <= size_limit)
            .cloned()
            .map(|x| {
                let mut t = n.accumulated_container_ids.clone();
                t.insert(n.accumulated_container_ids.len(), x.id);
                ConNode::new(
                    x,
                    n.accumulated_size + x.size,
                    t,
                    n.unused_containers
                        .iter()
                        .filter(|f| f.id != x.id)
                        .cloned()
                        .collect(),
                )
            })
            .collect();
        // assing the children to the node
        n.child_containers = c;
        // call for each child the subtree generator

        n.child_containers
            .iter()
            .clone()
            .for_each(|x| queue.push_back(x.clone()));
    }
    let size_limit: u32 = 150;
    // parse input line wise into a vec of u32
    let unused_containers: Vec<Contaier> = s
        .lines()
        .enumerate()
        .map(|(i, x)| Contaier::new(i as u32 + 1, x.parse::<u32>().unwrap()))
        .collect();
    // start node aka root of tree
    let mut root: ConNode = ConNode::new(Contaier::new(0, 0), 0, vec![0], unused_containers);
    // queue for open nodes
    let mut open_node_queue: VecDeque<ConNode> = VecDeque::new();
    open_node_queue.push_back(root);
    // create the tree
    while !open_node_queue.is_empty() {
        let mut n = open_node_queue.pop_back().unwrap();

        create_subtrees(
            &mut n,
            size_limit,
            &mut final_options,
            &mut counter,
            &mut open_node_queue,
        );
    }
    //dbg!(root);
    //dbg!(final_options);
    final_options.iter().count() as u32
}

fn part_two(s: &str) -> u32 {
    let mut final_options: HashSet<Vec<u32>> = HashSet::new();
    let mut counter: u32 = 0;
    // sub
    fn create_subtrees(
        n: &mut ConNode,
        size_limit: u32,
        f_o: &mut HashSet<Vec<u32>>,
        counter: &mut u32,
        queue: &mut VecDeque<ConNode>,
    ) {
        *counter = *counter + 1;
        if *counter % 1000000 == 0 {
            println!("{} {:?}", *counter, n.accumulated_container_ids);
        }
        // go through all the options/ unused containers and check if we could add them
        /*
        let viable_options: Vec<u32> = n
            .unused_containers
            .iter()
            .filter(|s| *s + n.accumulated_size <= size_limit)
            .cloned()
            .collect();
        */
        // check if this equals size_limit to add it as a final viable option
        if n.accumulated_size == size_limit {
            let mut t = n.accumulated_container_ids.clone();
            t.sort();
            let t = t.drain(1..).collect();
            f_o.insert(t);
        }

        let c: Vec<ConNode> = n
            .unused_containers
            .iter()
            .filter(|s| s.size + n.accumulated_size <= size_limit)
            .cloned()
            .map(|x| {
                let mut t = n.accumulated_container_ids.clone();
                t.insert(n.accumulated_container_ids.len(), x.id);
                ConNode::new(
                    x,
                    n.accumulated_size + x.size,
                    t,
                    n.unused_containers
                        .iter()
                        .filter(|f| f.id != x.id)
                        .cloned()
                        .collect(),
                )
            })
            .collect();
        // assing the children to the node
        n.child_containers = c;
        // call for each child the subtree generator

        n.child_containers
            .iter()
            .clone()
            .for_each(|x| queue.push_back(x.clone()));
    }
    let size_limit: u32 = 150;
    // parse input line wise into a vec of u32
    let unused_containers: Vec<Contaier> = s
        .lines()
        .enumerate()
        .map(|(i, x)| Contaier::new(i as u32 + 1, x.parse::<u32>().unwrap()))
        .collect();
    // start node aka root of tree
    let mut root: ConNode = ConNode::new(Contaier::new(0, 0), 0, vec![0], unused_containers);
    // queue for open nodes
    let mut open_node_queue: VecDeque<ConNode> = VecDeque::new();
    open_node_queue.push_back(root);
    // create the tree
    while !open_node_queue.is_empty() {
        let mut n = open_node_queue.pop_back().unwrap();

        create_subtrees(
            &mut n,
            size_limit,
            &mut final_options,
            &mut counter,
            &mut open_node_queue,
        );
    }
    //dbg!(root);
    //dbg!(final_options);
    let min = final_options.iter().min_by_key(|v| v.len()).unwrap().len();
    final_options.iter().filter(|v| v.len() == min).count() as u32
}

fn main() {
    let instructions =
    // using atom-script package
    // fs::read_to_string("dataSources/day13/input.csv").expect("read in file failed");
    // using the cargo in terminal
    fs::read_to_string("../dataSources/day17/input.csv").expect("read in file failed");

    // change to compute part1
    //let answer_part_one = part_one(&instructions);
    let answer_part_one = 1304;

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
