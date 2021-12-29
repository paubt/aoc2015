use std::{fs};

fn main() {

    let instructions = fs::read_to_string("../dataSources/day2/input.csv")
        .expect("fail to read in file into string");
    // read in file into 2d vec of strings
    let mut dimensions_l_w_h = vec![];
    for instruction in instructions.split('\n') {
        let mut t = vec![];
        for x in instruction.split('x') {
            t.push(x.parse::<i32>().unwrap());
        }
        dimensions_l_w_h.push(t);
    }

    let mut total_needed_wrap_paper = 0;
    let mut total_needed_ribbon_length = 0;
    for dimension in dimensions_l_w_h.iter() {
        // stuff for the wrap paper
        {
            let mut area = 2 * (dimension[0] * dimension[1] + dimension[1] * dimension[2] + dimension[2] * dimension[0]);
            if dimension[0] * dimension[1] <= dimension[1] * dimension[2] &&
                dimension[0] * dimension[1] <= dimension[2] * dimension[0] {
                area += dimension[0] * dimension[1];
            } else if dimension[1] * dimension[2] <= dimension[2] * dimension[0] {
                area += dimension[1] * dimension[2];
            } else {
                area += dimension[2] * dimension[0];
            }
            total_needed_wrap_paper += area;
        }
        // stuff for the ribbon
        {
            // perfect bow length
            let mut length = dimension[0]*dimension[1]*dimension[2];
            // [2] is larger than both
            if dimension[0] <= dimension[2] && dimension[1] <= dimension[2] {
                length += 2*(dimension[0]+dimension[1]);
                // as [2] is at least smaller than [1] or [2] -> find the smaller and do smaller*[2]
            } else if dimension[1] >= dimension[0] {
                length += 2*(dimension[0]+dimension[2]);
            } else {
                length += 2*(dimension[1]+dimension[2])
            }
            total_needed_ribbon_length += length;
        }
    }

    println!("warping paper = {}\nribbon length = {}",
             total_needed_wrap_paper, total_needed_ribbon_length);
}