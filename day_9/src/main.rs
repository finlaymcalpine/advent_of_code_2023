use std::fs::File;
use std::io::{BufRead, BufReader};

/*

*/

const FILE: &str = "../inputs/day9_1.txt";

fn parser(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("Cannot read file");
    BufReader::new(file)
}

fn sequence_finder(digits: Vec<isize>, sequences: &mut Vec<Vec<isize>>) -> &mut Vec<Vec<isize>> {
    sequences.push(digits);

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
        let mut sequence = vec![];
        let s = line.unwrap();
        let numbers: Vec<isize> = s
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let seq = sequence_finder(numbers, &mut sequence);

        sigma += extrapolate(seq.clone());
        sigma2 += extrapolate_back(seq.clone())
    }

    println!("Part One: {}", sigma);

    println!("Part Two: {}", sigma2);
}
