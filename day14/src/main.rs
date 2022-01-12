use std::fs;

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: i32,
    run_time: i32,
    rest_time: i32,
}
// Vixen can fly 19 km/s for 7 seconds, but then must rest for 124 seconds.

fn calc_distance_after_n_sec(r: &Reindeer, sec: i32) -> i32 {
    // the period of on run and rest together
    let period: i32 = r.run_time + r.rest_time;
    // the amount run each full period
    let period_distance: i32 = r.run_time * r.speed;
    // the number of complete periods the reindeer is allowed to do
    let complete_periods: i32 = sec / period;
    // the remaining seconds
    let remaining_seconds: i32 = sec % period;

    if remaining_seconds == 0 {
        // if there is no remaining seconds only multiply the period_distance with the complete_periods
        // to get the total distance
        complete_periods * period_distance
    } else if remaining_seconds > r.run_time {
        // as the remaining_seconds extend over the run_time we add a "whole" period to it
        (complete_periods + 1) * period_distance
    } else {
        // as remaining_seconds is somewhere in the range of 0..run_time we add the distance of
        // the proportion to the result
        complete_periods * period_distance + remaining_seconds * r.speed
    }
}

fn part_one(s: &str) -> i32 {
    let sec: i32 = 2503;
    let mut herd: Vec<Reindeer> = vec![];
    for line in s.split('\n') {
        if line.is_empty() {
            break;
        }
        let x = line.split(' ').collect::<Vec<&str>>();
        // print!("{:#?}", x);
        let new_reindeer = Reindeer {
            name: x[0].to_string(),
            speed: x[3].parse::<i32>().unwrap(),
            run_time: x[6].parse::<i32>().unwrap(),
            rest_time: x[13].parse::<i32>().unwrap(),
        };
        herd.push(new_reindeer);
    }
    let mut max_distance: i32 = 0;
    // iterate over the herd and calc the distance
    for r in &herd {
        let distance: i32 = calc_distance_after_n_sec(r, sec);
        if distance > max_distance {
            max_distance = distance;
        }
    }
    max_distance
}

fn part_two(s: &str) -> i32 {
    // TODO add part_two
    0
}

fn main() {
    let instructions =
        // using atom-script package
        // fs::read_to_string("dataSources/day13/input.csv").expect("read in file failed");
        // using the cargo in terminal
        fs::read_to_string("../dataSources/day14/input.csv").expect("read in file failed");

    let answer_part_one = part_one(&instructions);

    let answer_part_two = part_two(&instructions);

    println!(
        "answer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}
