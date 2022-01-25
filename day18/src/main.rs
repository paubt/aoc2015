use std::fs;

fn part_one(s: &str) -> usize {
    let size: usize = 100;
    let steps: u32 = 100;

    // transform into 2d bool array
    let mut light_grid: Vec<Vec<bool>> = s
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| {
                    if c == '#' {
                        true
                    } else if c == '.' {
                        false
                    } else {
                        panic!("unexpected char in input")
                    }
                })
                .collect::<Vec<bool>>()
        })
        .collect();
    // append empty cell front and back
    light_grid.iter_mut().for_each(|r| {
        r.insert(0, false);
        r.insert(r.len(), false);
    });
    // append empty cell line top and bot
    light_grid.insert(0, (0..size + 2).map(|_| false).collect::<Vec<bool>>());
    light_grid.insert(
        light_grid.len(),
        (0..size + 2).map(|_| false).collect::<Vec<bool>>(),
    );

    fn update_cell(row: usize, col: usize, grid: &Vec<Vec<bool>>) -> bool {
        match grid[row][col] {
            true => {
                let neighbors_on: u16 = grid[row - 1][col - 1] as u16
                    + grid[row - 1][col] as u16
                    + grid[row - 1][col + 1] as u16
                    + grid[row][col - 1] as u16
                    + grid[row][col + 1] as u16
                    + grid[row + 1][col - 1] as u16
                    + grid[row + 1][col] as u16
                    + grid[row + 1][col + 1] as u16;
                return if neighbors_on == 2 || neighbors_on == 3 {
                    true
                } else {
                    false
                };
            }
            false => {
                let neighbors_on: u16 = grid[row - 1][col - 1] as u16
                    + grid[row - 1][col] as u16
                    + grid[row - 1][col + 1] as u16
                    + grid[row][col - 1] as u16
                    + grid[row][col + 1] as u16
                    + grid[row + 1][col - 1] as u16
                    + grid[row + 1][col] as u16
                    + grid[row + 1][col + 1] as u16;
                return if neighbors_on == 3 { true } else { false };
            }
        }
    }
    for _ in 0..steps {
        let mut new_grid: Vec<Vec<bool>> = light_grid.clone();

        for row in 1..size + 1 {
            for col in 1..size + 1 {
                new_grid[row][col] = update_cell(row, col, &light_grid);
            }
        }
        light_grid = new_grid.clone();
    }
    // count trues at the end
    light_grid
        .iter()
        .map(|r| r.iter().filter(|v| *v == &true).count())
        .sum()
}

fn part_two(s: &str) -> usize {
    let size: usize = 100;
    let steps: u32 = 100;

    // transform into 2d bool array
    let mut light_grid: Vec<Vec<bool>> = s
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| {
                    if c == '#' {
                        true
                    } else if c == '.' {
                        false
                    } else {
                        panic!("unexpected char in input")
                    }
                })
                .collect::<Vec<bool>>()
        })
        .collect();
    // append empty cell front and back
    light_grid.iter_mut().for_each(|r| {
        r.insert(0, false);
        r.insert(r.len(), false);
    });
    // append empty cell line top and bot
    light_grid.insert(0, (0..size + 2).map(|_| false).collect::<Vec<bool>>());
    light_grid.insert(
        light_grid.len(),
        (0..size + 2).map(|_| false).collect::<Vec<bool>>(),
    );

    fn update_cell(row: usize, col: usize, grid: &Vec<Vec<bool>>) -> bool {
        match grid[row][col] {
            true => {
                let neighbors_on: u16 = grid[row - 1][col - 1] as u16
                    + grid[row - 1][col] as u16
                    + grid[row - 1][col + 1] as u16
                    + grid[row][col - 1] as u16
                    + grid[row][col + 1] as u16
                    + grid[row + 1][col - 1] as u16
                    + grid[row + 1][col] as u16
                    + grid[row + 1][col + 1] as u16;
                return if neighbors_on == 2 || neighbors_on == 3 {
                    true
                } else {
                    false
                };
            }
            false => {
                let neighbors_on: u16 = grid[row - 1][col - 1] as u16
                    + grid[row - 1][col] as u16
                    + grid[row - 1][col + 1] as u16
                    + grid[row][col - 1] as u16
                    + grid[row][col + 1] as u16
                    + grid[row + 1][col - 1] as u16
                    + grid[row + 1][col] as u16
                    + grid[row + 1][col + 1] as u16;
                return if neighbors_on == 3 { true } else { false };
            }
        }
    }
    for _ in 0..steps {
        let mut new_grid: Vec<Vec<bool>> = light_grid.clone();

        for row in 1..size + 1 {
            for col in 1..size + 1 {
                new_grid[row][col] = update_cell(row, col, &light_grid);
            }
        }
        // part2 diff -> set corners to true
        new_grid[1][1] = true;
        new_grid[1][size] = true;
        new_grid[size][1] = true;
        new_grid[size][size] = true;

        light_grid = new_grid.clone();
    }
    // count trues at the end
    light_grid
        .iter()
        .map(|r| r.iter().filter(|v| *v == &true).count())
        .sum()
}

fn main() {
    let instructions =
    // using atom-script package
    // fs::read_to_string("dataSources/day13/input.csv").expect("read in file failed");
    // using the cargo in terminal
    fs::read_to_string("../dataSources/day18/input.csv").expect("read in file failed");

    // change to compute part1
    let answer_part_one = part_one(&instructions);

    let answer_part_two = part_two(&instructions);

    println!(
        "answer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}
