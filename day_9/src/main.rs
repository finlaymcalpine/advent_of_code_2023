use std::fs::File;
use std::io::{BufRead, BufReader};

/*
When I first wrote this, I was passing a mutable vector to the sequence finder functions, and then returning that vector from the function. I thought that might be a valid way to do it, but it was yielding a result far too high.
I'm not yet sure why.
*/

const FILE: &str = "../inputs/day9_1.txt";

fn parser(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("Cannot read file");
    BufReader::new(file)
}

fn sequence_finder(digits: Vec<isize>) -> Vec<Vec<isize>> {
    let mut sequences = vec![digits];

    while !sequences.last().unwrap().iter().all(|v| *v == 0isize) {
        let diffs: Vec<isize> = sequences
            .last()
            .unwrap()
            .windows(2)
            //.map(|vs| {
            //    let [x, y] = vs else { unreachable!() };
            //    y - x
            //})
            .map(|w| w[1] - w[0])
            .collect();

        sequences.push(diffs);
    }

    sequences
}

fn extrapolate(sequence: Vec<Vec<isize>>) -> isize {
    let mut next_val = 0;
    for sequence in sequence.iter().rev().skip(1) {
        next_val += sequence.last().unwrap();
    }
    next_val
}

fn extrapolate_back(sequence: Vec<Vec<isize>>) -> isize {
    let mut next_val = 0;
    for sequence in sequence.iter().rev().skip(1) {
        next_val = sequence.first().unwrap() - next_val;
    }
    next_val
}

fn main() {
    let file = parser(FILE);
    let mut sigma: isize = 0;
    let mut sigma2: isize = 0;

    for line in file.lines() {
        let s = line.unwrap();
        let numbers: Vec<isize> = s
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let seq = sequence_finder(numbers);
        sigma += extrapolate(seq.clone());
        sigma2 += extrapolate_back(seq.clone())
    }

    println!("Part One: {}", sigma);

    println!("Part Two: {}", sigma2);
}
