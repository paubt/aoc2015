use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;


fn part_one(s: &str) -> i32
{
    // read input into HashMap
    let mut happiness_map: HashMap<(String,String), i32> = HashMap::new();
    for line in s.split('\n')
    {
        let s = line.split(' ').collect::<Vec<&str>>();
        let mut h: i32 = 0;
        if s[2] == "gain"
        {
            h = s[3].parse::<i32>().unwrap();
        }
        else
        {
            h = -1 * s[3].parse::<i32>().unwrap();
        }
        let left = s[0].to_string();
        let right = s[10][..s[10].len()-1].to_string();
        // check if the other part is already in the map -> if so just add the value of the
        // other direction to the value
        // else add it as a new key value pair tot the map
        if let Some(x) = happiness_map.get_mut(&(right, left))
        {
            *x += h;
        }
        else
        {
            happiness_map.insert((s[0].to_string(), s[10][..s[10].len()-1].to_string()), h);
        }

    }
    // greedy algorithm
    // takes in the connections and returns the smallest tour length
    fn greedy_search(weighted_connection: HashMap<(String,String), i32>) -> i32
    {
        // vec for the solution
        let mut final_order_of_persons: Vec<&str> = Vec::new();
        let mut final_order_value: i32= 0;
        // set of the reaming vertices
        let mut remaining_persons: HashSet<&str> = HashSet::new();
        // iter over happiness map and insert the names into hash SET
        for ((f,x),_) in weighted_connection.iter()
        {
            remaining_persons.insert(f);
            remaining_persons.insert(x);
        }
        // add the first person from remaining to final as initial position
        {
            let first = remaining_persons.iter().nth(0).unwrap();
            final_order_of_persons.push(first);
            remaining_persons.remove(final_order_of_persons.last().unwrap());
        }

        // repeat until all remaining vertices are integrated
        while !remaining_persons.is_empty()
        {
            let last_added_person: &str = final_order_of_persons.last().unwrap();
            let mut highest_happiness_viable_neighbor_value: i32 = -100000;
            let mut highest_happiness_viable_neighbor: &str = "";
            for &possible_neighbor in remaining_persons.iter()
            {
                // check if there is a left-right key
                if let Some(x) = weighted_connection.get(&(last_added_person.to_string(), possible_neighbor.to_string()))
                {
                    if *x > highest_happiness_viable_neighbor_value
                    {
                        highest_happiness_viable_neighbor_value = *x;
                        highest_happiness_viable_neighbor = possible_neighbor;
                    }
                }
                else if let Some(x) = weighted_connection.get(&(possible_neighbor.to_string(), last_added_person.to_string()))
                {
                    if *x > highest_happiness_viable_neighbor_value
                    {
                        highest_happiness_viable_neighbor_value = *x;
                        highest_happiness_viable_neighbor = possible_neighbor;
                    }
                }
                else
                {
                    panic!("greedy; both the combinations of 2 persons isn't in the weighted map")
                }
            }
            final_order_of_persons.push(highest_happiness_viable_neighbor);
            remaining_persons.remove(highest_happiness_viable_neighbor);
            final_order_value += highest_happiness_viable_neighbor_value;
        }
        // add from the last to the first the happiness value to do the wrap around
        if let Some(x) = weighted_connection.get(&(final_order_of_persons.first().to_string(),
                                                   final_order_of_persons.last().to_string()))
        {
            final_order_value += *x;
        }
        else if let Some(x) = weighted_connection.get(&(final_order_of_persons.last().to_string(),
                                                        final_order_of_persons.first().to_string()))
        {
            final_order_value += *x;
        }
        else
        {

        }
        final_order_value
    }

    print!("greedy result = {} \n", greedy_search(happiness_map.clone()));

    print!("length = {} \n", happiness_map.len());
    dbg!(happiness_map);
    0
}


fn part_two(s: &str) -> i32 {
    0
}

// dude i am stupid
// by adding the individual valuation of the two persons sitting next to each other
// the problem becomes a ordinary TSP
// e.g.: carol->alice: -62 && alice->carol: -76  becomes the carol-alice: -138
// (which is the same as alice-carol)


fn main()
{
    let instructions = fs::read_to_string("../dataSources/day13/input.csv")
        .expect("read in file failed");

    let answer_part_one = part_one(&instructions);

    let answer_part_two = part_two(&instructions);

    println!("answer part 1: {}\nanswer part 2: {} ", answer_part_one, answer_part_two);
}