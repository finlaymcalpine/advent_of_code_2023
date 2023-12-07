use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
//use std::env;

const FILE: &str = "../inputs/day7_1.txt";

fn parse_file(filename: &str) -> Vec<((u32, u32, u32, u32, u32, u32), u32)> {
    let file = File::open(filename).expect("Cannot read file");
    let reader = BufReader::new(file);

    let mut hand_ranks = vec![];

    for line in reader.lines() {
        let s1 = line.unwrap();
        let input = s1.split_whitespace().collect::<Vec<&str>>();
        let b = input[1].trim().parse::<u32>().unwrap();

        let hand_rank = get_type(input[0]);

        hand_ranks.push((hand_rank, b));
    }

    hand_ranks
}

fn get_type(hand: &str) -> (u32, u32, u32, u32, u32, u32) {
    // have changed from Part One array, with Joker now the weakets card.
    let card_arr = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'
    ];
    let c1 = hand.chars().nth(0).unwrap();
    let c2 = hand.chars().nth(1).unwrap();
    let c3 = hand.chars().nth(2).unwrap();
    let c4 = hand.chars().nth(3).unwrap();
    let c5 = hand.chars().nth(4).unwrap();
    let first = (card_arr.len() - card_arr.iter().position(|x: &char| x == &c1).unwrap()) as u32;
    let second = (card_arr.len() - card_arr.iter().position(|x: &char| x == &c2).unwrap()) as u32;
    let third = (card_arr.len() - card_arr.iter().position(|x: &char| x == &c3).unwrap()) as u32;
    let fourth = (card_arr.len() - card_arr.iter().position(|x: &char| x == &c4).unwrap()) as u32;
    let fifth = (card_arr.len() - card_arr.iter().position(|x: &char| x == &c5).unwrap()) as u32;

    let card_set: HashMap<char, u32> = hand.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    let most_common = card_set.iter().filter(|entry| entry.0 != &'J').max_by_key(|entry | entry.1).unwrap_or((&'J', &5));
    /*
    match most_common {
        Some(x) => ,
        None => ,
    };
    */
    //println!("{:?}", most_common);
    let new_hand = hand.replace('J', most_common.0.to_string().as_str());
    let new_card_set: HashMap<char, u32> = new_hand.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });
    //println!("{:?}", card_set);
    let size = new_card_set.keys().len();

    let card_type: u32 = if size != 2 && size != 3 {
        if size == 1 {
            6
        } else if size == 4 {
            1
        } else {
            0
        }
    } else if size == 2 {
        if new_card_set.values().collect::<Vec<&u32>>().contains(&&1) {
            5
        } else {
            4
        }
    } else if new_card_set.values().collect::<Vec<&u32>>().contains(&&3) {
        3
    } else {
        2
    };

    (card_type, first, second, third, fourth, fifth)
}

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    let mut data = parse_file(FILE);
    data.sort_by_key(|a| a.0 .5);
    data.sort_by_key(|a| a.0 .4);
    data.sort_by_key(|a| a.0 .3);
    data.sort_by_key(|a| a.0 .2);
    data.sort_by_key(|a| a.0 .1);
    data.sort_by_key(|a| a.0 .0);
    //println!("{:?}", data);

    let mut total: u32 = 0;
    for (index, hand) in data.iter().enumerate() {
        total += (index as u32 + 1) * hand.1;
    }
    println!("Part 1: {}", total);
}
