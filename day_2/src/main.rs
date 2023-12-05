use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes,
show them to you, and then put them back in the bag.
He'll do this a few times per game.
The Elf is drawing with replacement from the bag.

The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
A game is impossible if any single draw contains more than the number of cubes of a given color that are in the bag. E.g. 14 red cubes appear at once.

Can we assume the input only contains one entry for each color per draw? I.e. there won't be input like "7 red, 2 blue, 6 red; 3 green, 4 red, 1 blue"
I am going to work on that basis for now, so we don't have to add the same color within a draw.

Answer to Part 1 was 2632
Answer to Part 2 was 69629

Couple of refactoring things we could do:
- use format!() to create the regex for red, green, blue without duplication.
- create a function to parse color vector for line, to avoid captures, map, flatten, collect duplication.
- keep possible/color_minimums functions, but use the above functions within them.
*/

const FILE: &str = "../inputs/day2_1.txt";

fn possible(s: &String) -> bool {
    let red_regex: regex::Regex = Regex::new(r"(\d+) red").unwrap();
    let blue_regex: regex::Regex = Regex::new(r"(\d+) blue").unwrap();
    let green_regex: regex::Regex = Regex::new(r"(\d+) green").unwrap();

    // let blue: Vec<u32> = blue_regex.find_iter(s).filter_map(|c| c.as_str().parse::<u32>().ok()).collect::<Vec<u32>>(); DID NOT WORK. LED TO NONE IN LN34. NOT SURE WHY?
    let red: Vec<u32> = red_regex
        .captures_iter(s)
        .filter_map(|c| c.get(1).map(|m| m.as_str().parse::<u32>().ok()))
        .flatten()
        .collect();
    let blue: Vec<u32> = blue_regex
        .captures_iter(s)
        .filter_map(|c| c.get(1).map(|m| m.as_str().parse::<u32>().ok()))
        .flatten()
        .collect();
    let green: Vec<u32> = green_regex
        .captures_iter(s)
        .filter_map(|c| c.get(1).map(|m| m.as_str().parse::<u32>().ok()))
        .flatten()
        .collect();

    if red.iter().max().unwrap() > &12
        || green.iter().max().unwrap() > &13
        || blue.iter().max().unwrap() > &14
    {
        return false;
    }
    return true;
}

fn color_minimums(s: &String) -> u32 {
    let red_regex: regex::Regex = Regex::new(r"(\d+) red").unwrap();
    let blue_regex: regex::Regex = Regex::new(r"(\d+) blue").unwrap();
    let green_regex: regex::Regex = Regex::new(r"(\d+) green").unwrap();

    let red: Vec<u32> = red_regex
        .captures_iter(s)
        .filter_map(|c| c.get(1).map(|m| m.as_str().parse::<u32>().ok()))
        .flatten()
        .collect();
    let blue: Vec<u32> = blue_regex
        .captures_iter(s)
        .filter_map(|c| c.get(1).map(|m| m.as_str().parse::<u32>().ok()))
        .flatten()
        .collect();
    let green: Vec<u32> = green_regex
        .captures_iter(s)
        .filter_map(|c| c.get(1).map(|m| m.as_str().parse::<u32>().ok()))
        .flatten()
        .collect();

    let red_min = red.iter().max().unwrap();
    let blue_min = blue.iter().max().unwrap();
    let green_min = green.iter().max().unwrap();

    red_min * blue_min * green_min
}

fn part_one() {
    let game_regex: regex::Regex = Regex::new(r"^Game (\d+)").unwrap();

    let file = File::open(FILE).expect("Cannot read file");
    let reader = BufReader::new(file);

    let mut poss_games = Vec::new();

    for line in reader.lines() {
        let s1 = line.unwrap();
        let game: u32 = game_regex
            .captures(&s1)
            .unwrap()
            .get(1)
            .map(|x| x.as_str().parse::<u32>())
            .unwrap()
            .expect("Failed to parse game number");

        let game_possible: bool = possible(&s1);

        if game_possible {
            poss_games.push(game);
        }
    }

    let total: u32 = poss_games.iter().sum();

    println!("Total of possible game numbers is {}", total);
}

fn part_two() {
    // Want to know the maximum observed number of cubes for each color. Again, we can ignore the draws, because each color appears only once per draw.
    // Want the max because we need at least that many cubes of a color for the draw to be possible.
    let file = File::open(FILE).expect("Cannot read file");
    let reader = BufReader::new(file);

    let mut games = Vec::new();

    for line in reader.lines() {
        let s1 = line.unwrap();

        let game_power: u32 = color_minimums(&s1);

        games.push(game_power);
    }

    let total: u32 = games.iter().sum();

    println!("Total of game powers is {}", total);
}

fn main() {
    part_one();
    part_two();
}
