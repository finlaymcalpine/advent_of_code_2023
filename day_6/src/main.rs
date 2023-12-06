use itertools::Itertools;
use std::fs;
use std::iter::zip;

/*
As part of signing up, you get a sheet of paper (your puzzle input) that lists the time allowed for each race and also the best distance ever recorded in that race. 
To guarantee you win the grand prize, you need to make sure you go farther in each race than the current record holder.

The organizer brings you over to the area where the boat races are held. The boats are much smaller than you expected - they're actually toy boats, each with a big button on top. 
Holding down the button charges the boat, and releasing the button allows the boat to move. 
Boats move faster if their button was held longer, but time spent holding the button counts against the total race time. 
You can only hold the button at the start of the race, and boats don't move until the button is released.
*/

const FILE: &str = "../inputs/day6_1.txt";

fn parse_inputs(filename: &str) -> Vec<(u32, u32)> {
    let file = fs::read_to_string(FILE).expect("File read failure");

    let (t, d) = file.split_once("\n").unwrap();

    let time: Vec<u32> = t.split_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();
    let distance: Vec<u32> = d.split_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();

    return zip(time, distance).collect();

}

fn updated_parser(filename: &str) -> (u64, u64) {
    let file = fs::read_to_string(FILE).expect("File read failure");

    let (t, d) = file.split_once("\n").unwrap();

    let time: u64 = t.split_whitespace().skip(1).collect::<String>().parse().unwrap();
    let distance: u64 = d.split_whitespace().skip(1).collect::<String>().parse().unwrap();

    return (time, distance);
}

fn part_one() {
    let races = parse_inputs(FILE);
    let mut race_wins: Vec<u32> = vec![];
    
    for (time, distance) in races {
        let mut wins: u32 = 0;
        for i in 0..=time {
            let d: u32 = (time - i) * i;
            if d > distance {
                wins += 1;
            }
        }
        race_wins.push(wins);
    }

    let outcome: u32 = race_wins.iter().product();

    println!("{}", outcome)
}

fn part_two() {
    let (time, distance) = updated_parser(FILE);

    let mut wins: u64 = 0;
    for i in 0..=time {
        let d: u64 = (time - i) * i;
        
        if d > distance {
            wins += 1;
        }
    }
    
    println!("{}", wins)
}

fn main() {
    part_one();
    part_two();
}
