use itertools::Itertools;
use std::{fs, u64::MAX};
//use std::io::{BufRead, BufReader};

/*
The almanac (your puzzle input) lists all of the seeds that need to be planted.
It also lists what type of soil to use with each kind of seed, what type of fertilizer to use with each kind of soil, what type of water to use with each kind of fertilizer, and so on.
Every type of seed, soil, fertilizer and so on is identified with a number, but numbers are reused by each category - that is, soil 123 and fertilizer 123 aren't necessarily related to each other.

The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.

The rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category.
That is, the section that starts with seed-to-soil map: describes how to convert a seed number (the source) to a soil number (the destination).
This lets the gardener and his team know which soil to use with which seeds, which water to use with which fertilizer, and so on.

Rather than list every source number and its corresponding destination number one by one, the maps describe entire ranges of numbers that can be converted.
Each line within a map contains three numbers: the destination range start, the source range start, and the range length.

https://topaz.github.io/paste/#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
*/

const FILE: &str = "../inputs/day5_1.txt";

fn file_parse(input: String) -> (Vec<u64>, Vec<Vec<Vec<u64>>>) {
    let mut seed_list: Vec<u64> = vec![];
    let mappings: Vec<Vec<Vec<u64>>>;

    let (seeds, remainder) = input.split_once("\n\n").unwrap();

    let seed_elems: Vec<u64> = seeds
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();
    for seed in seed_elems {
        seed_list.push(seed);
    }

    mappings = remainder
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .skip(1)
                .map(|l| {
                    l.split_whitespace()
                        .map(|d| d.parse().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<Vec<u64>>>()
        })
        .collect::<Vec<Vec<Vec<u64>>>>();

    return (seed_list, mappings);
}

fn part_two() {
    let file = fs::read_to_string(FILE).expect("File read failure");
    let mapping = file_parse(file);

    let seeds: Vec<(u64, u64)> = mapping.0.into_iter().tuples().collect();
    let process = mapping.1;

    let mut lowest_location: u64 = MAX;

    for (start, length) in seeds {
        for i in start..(start + length) {
            let mut value = i;

            'a: for chunk in &process {
                for map in chunk {
                    if value >= map[1] && value < (map[1] + map[2]) {
                        value = map[0] + (value - map[1]);
                        continue 'a;
                    } else {
                        value = value
                    };
                }
            }
            if value < lowest_location {
                lowest_location = value;
            }
        }
    }

    //let minimum = outputs.iter().min().unwrap();

    println!("{:?}", lowest_location);
}

fn part_two_v2 () {
    todo!("Will make the part two algorithm more efficient: currently using brute force")
}

fn main() {
    
    let file = fs::read_to_string(FILE).expect("File read failure");
    let mapping = file_parse(file);

    let seeds = mapping.0; // if wanted to reuse mapping.0, we would add .clone() method to the end
    let process = mapping.1;

    let mut outputs: Vec<u64> = vec![];

    for seed in seeds {
        let mut value = seed;

        'a: for chunk in &process {
            for map in chunk {
                if value >= map[1] && value < (map[1] + map[2]) {
                    value = map[0] + (value - map[1]);
                    continue 'a;
                }
                else {value = value};
            }
        }
        outputs.push(value);
    }

    let minimum = outputs.iter().min().unwrap();

    println!("{:?}", minimum);

    part_two();
}
