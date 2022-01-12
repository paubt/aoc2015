use std::fs;

#[derive(Debug)]
struct Reindeer {
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

#[derive(Debug)]
struct ReindeerPart2 {
    speed: i32,
    run_time: i32,
    period_time: i32,
    current_distance: i32,
    current_position_in_period: i32,
    counter_first: i32,
}

fn part_two(s: &str) -> i32 {
    let sec: i32 = 2503;
    let mut herd: Vec<ReindeerPart2> = vec![];
    for line in s.split('\n') {
        if line.is_empty() {
            break;
        }
        let x = line.split(' ').collect::<Vec<&str>>();
        // print!("{:#?}", x);
        let new_reindeer = ReindeerPart2 {
            speed: x[3].parse::<i32>().unwrap(),
            run_time: x[6].parse::<i32>().unwrap(),
            period_time: x[6].parse::<i32>().unwrap() + x[13].parse::<i32>().unwrap(),
            current_distance: 0,
            current_position_in_period: 0,
            counter_first: 0,
        };
        herd.push(new_reindeer);
    }
    // repeat sec times the increment for all
    for _ in 0..sec {
        // increment point in period and corresponding distance change for each reindeer
        for r in &mut herd {
            r.current_position_in_period = r.current_position_in_period + 1;
            // if the current position is smaller or equal than the run_time
            if r.current_position_in_period <= r.run_time {
                r.current_distance = r.current_distance + r.speed;
            } else {
                // no update of current_distance but if the current_position_in_period is higher than the
                // period set back to 0
                if r.current_position_in_period >= r.period_time {
                    r.current_position_in_period = 0;
                }
            }
        }
        // determine what is the max distance traveled so far
        let mut current_best_distance: i32 = 0;
        for r in &herd {
            if r.current_distance > current_best_distance {
                current_best_distance = r.current_distance;
            }
        }
        // increment all reindeer with the distance equal to current_best_distance
        for r in &mut herd {
            if r.current_distance == current_best_distance {
                r.counter_first = r.counter_first + 1;
            }
        }
    }
    let mut most_times_first: i32 = 0;
    for r in &herd {
        if r.counter_first > most_times_first {
            most_times_first = r.counter_first;
        }
    }
    most_times_first
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
