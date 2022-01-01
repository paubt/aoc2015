use std::fs;
use md5;

/*
tried to create a custom md5 function but failed miserably
now using the rust standard implementation

fn padding_the_input(input: &str) -> String {
    // first add a 1
    let input = format!("{}1", input);
    // calc needed zeros
    let mut n = 512 - input.len() % 512 - 64;
    // rest
    let rest = format!("{:064b}", input.len());

    let mut pad = String::new();
    while n != 0 {
        pad = format!("{}0", pad);
        n -= 1;
    }
    format!("{}{}{}", input, pad, rest)
}


fn part_one(input: &str) -> i64 {

    let s : Vec<u32> = vec![7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
                 5,  9, 14, 20, 5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
                 4, 11, 16, 23,  4, 11, 16, 23, 4, 11, 16, 23,  4, 11, 16, 23,
                 6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21];

    let mut k: Vec<u32> = Vec::new();

    for x in 0..64 {
        k.push(((2_f64).powf(32_f64)*(x as f64 +1_f64).sin().abs()).floor() as u32);
    }

    let mut a0 :u32 = 0x67452301;  // A
    let mut b0 :u32 = 0xefcdab89;  // B
    let mut c0 :u32 = 0x98badcfe;  // C
    let mut d0 :u32 = 0x10325476;  // D

    print!("as a utf8 encoded: {}\n", input);
    let mut b_input  = String::new();
    let input = input.to_string();

    for c in input.into_bytes() {
        b_input += &format!("0{:b}", c);
    }
    // add some padding
    let input = padding_the_input(&b_input);
    print!("the padded representation: {}\n", input);
    // number of 512-bit chunks in the padded message
    let counter = input.len() % 512 + 1;
    print!("512-bit chunks: {}\n", counter);
    // message string to vec
    let mut input : Vec<char> = input.chars().collect();

    for x in 0..counter {
        // take the first chunk of the message -> 512 first characters of input
        let mut chunk = Vec::new();
        for i in 0..512 {
            chunk.push(input.get(i));
        }
        print!("chunk length {}\n", chunk.len());
        // break chunk into sixteen 32-bit words
        let mut m : Vec<String> = Vec::new();
        for i in 0..16 {
            let mut t = String::new();
            for j in 0..32 {
                t.push(*chunk[(i*32)+j].unwrap())
            }
            m.push(t);
        }
        // init values for this chunk
        let mut a :u32 = a0;
        let mut b :u32 = b0;
        let mut c :u32 = c0;
        let mut d :u32 = d0;
        // main loop:
        for i in 0..64 {
            let mut f : u32 = 0;
            let mut g : u32 = 0;
            if i <= 15 {
                f = (b & c) | (!(b) & d);
                g = i;
            } else if 16 <= i && i <= 31 {
                f = (d & b) | (!(d) & c);
                g = (5 * i + 1) % 16;
            } else if 32 <= i && i <= 47 {
                f = b ^ c ^ d;
                g = (3 * i + 5) % 16;
            } else if 48 <= i && i <= 64 {
                f = c ^ (b | !d);
                g = (7 * i) % 16;
            } else { panic!("js") }
            // from binary string to
            let m_as_binary = u32::from_str_radix(&m[g as usize], 2).unwrap();
            f = f.wrapping_add(a.wrapping_add(k[i as usize].wrapping_add(m_as_binary)));
            a = d;
            d = c;
            c = b;
            // here not sure if use left rotate on decimal or on binary representation
            // binary rotate
            let f_left_rotate = f.rotate_left(s[i as usize]);

            b = b.wrapping_add(f_left_rotate);
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
        print!("iteration {}\n", x);
    }

    let output = String::new();
    print!("{}", output);
    0
}

*/

fn part_one(input: &str) -> u32 {
    let mut counter = 1;
    loop {
        let input_with_counter = format!("{}{}", input, counter);
        let digest = md5::compute(&input_with_counter);
        counter += 1;
        // digest to string
        let digest = format!("{:x}", digest);
        // check if the first five digits are zero
        let first_five = u64::from_str_radix(&digest[..5], 16)
            .expect("slice of the first 5 to number failed");
        if first_five == 0 {
            print!(" {} {} {}\n", counter - 1, first_five, digest);
            break;
        }
    }
    counter - 1
}

fn part_two(input: &str) -> u32 {
    let mut counter = 1;
    loop {
        let input_with_counter = format!("{}{}", input, counter);
        let digest = md5::compute(&input_with_counter);
        counter += 1;
        // digest to string
        let digest = format!("{:x}", digest);
        // check if the first five digits are zero
        let first_five = u64::from_str_radix(&digest[..6], 16)
            .expect("slice of the first 5 to number failed");
        if first_five == 0 {
            print!(" {} {} {}\n", counter - 1, first_five, digest);
            break;
        }
    }
    counter - 1
}


fn main() {
    let instructions = fs::read_to_string("../dataSources/day4/input1.csv")
        .expect("read in file failed");

    let answer_part_one = part_one(&instructions);
    let answer_part_two = part_two(&instructions);

    println!("answer part 1: {}\nanswer part 2: {} ", answer_part_one, answer_part_two);
}
