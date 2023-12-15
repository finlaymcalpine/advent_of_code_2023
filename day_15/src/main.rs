use std::fs;

const FILE: &str = "../inputs/day15_1.txt";

fn hasher(input: &str) -> u32 {
    let mut current_value = 0;

    for c in input.chars() {
        current_value += c as u32;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

fn part_one() {
    let file = fs::read_to_string(FILE).expect("Cannot read file");
    let inp: Vec<&str> = file.trim().split(',').collect();
    let mut sigma: u32 = 0;

    for line in inp {
        //println!("{:?}", line);
        sigma += hasher(line);
    }

    println!("Part One: {}", sigma);
}

fn main() {
    part_one();
}
