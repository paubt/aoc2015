fn part_one() -> u32 {
    let presents_min: u32 = 36000000;
    let mut current_presents: u32 = 0;
    let mut elf_counter: u32 = 0;
    let mut max_present: u32 = 0;

    while current_presents < presents_min {
        elf_counter = elf_counter + 1;
        current_presents = 0;
        for x in 1..elf_counter + 1 {
            if elf_counter % x == 0 {
                current_presents = current_presents + x * 10;
            }
        }
        if current_presents > max_present {
            max_present = current_presents;
            println!("at {} new max is {}", elf_counter, max_present);
        }
    }
    print!("{} {}\n", elf_counter, current_presents);
    elf_counter
}

fn part_two() -> usize {
    let presents_min: u32 = 36000000;
    let mut houses: Vec<u32> = vec![];
    let mut elf_counter: usize = 0;
    //let mut skipped_houses: usize = 0;

    while {
        elf_counter = elf_counter + 1;
        //add 50 * elf_counter zeros to the houses
        houses.append(&mut vec![0; ((elf_counter * 50) - houses.len()) as usize]);
        //houses.push(0)
        houses
            .iter_mut()
            .step_by(elf_counter as usize)
            .take(50)
            .for_each(|x| *x = *x + elf_counter as u32 * 11);
        /*
        print!(
            "step={} len={}\n{:?}\n\n",
            elf_counter,
            houses.iter().count(),
            houses
        );
        */
        if elf_counter % 1000 == 0 {
            print!("{} {}\n", elf_counter, houses[0])
        }
        houses.remove(0) < presents_min
    } {}

    elf_counter
}

fn main() {
    // takes a lot of time but works
    let answer_part_one = part_one();
    //let answer_part_one = 831600;

    let answer_part_two = part_two();
    //let answer_part_two = 884520

    println!(
        "answer part 1: {}\nanswer part 2: {} ",
        answer_part_one, answer_part_two
    );
}
