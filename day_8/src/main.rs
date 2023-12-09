use regex::Regex;
use std::collections::HashMap;
use std::fs;

/*
LCM functions from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
*/

const FILE: &str = "../inputs/day8_1.txt";

fn parse_inputs(filename: &str) -> (Vec<char>, HashMap<String, Vec<String>>) {
    let mut node_map: HashMap<String, Vec<String>> = HashMap::new();

    let file = fs::read_to_string(filename).expect("File read failure");

    let (directions, nodes) = file.split_once("\n\n").unwrap();

    let rgx = Regex::new(r"(\w{3})").unwrap();

    for line in nodes.lines() {
        let matches: Vec<String> = rgx
            .find_iter(line)
            .map(|x| x.as_str().to_string())
            .collect();

        node_map.insert(
            matches[0].clone(),
            vec![matches[1].clone(), matches[2].clone()],
        );
    }

    (directions.chars().collect::<Vec<char>>(), node_map)
}

fn part_one() {
    let mut step_count: u32 = 0;
    let mut point = "AAA".to_string();
    let (directions, data) = parse_inputs(FILE);
    let direction_map = HashMap::from([('L', 0), ('R', 1)]);
    let mut indexer: usize = 0;
    let max_index = directions.len() - 1;

    while point != "ZZZ".to_string() {
        let d: usize = direction_map[&directions[indexer]];
        point = data[&point][d].clone();
        if indexer < max_index {
            indexer += 1;
        } else {
            indexer = 0;
        }
        step_count += 1
    }

    println!("Part One: {:?}", step_count)
}

fn step_counter(directions: &Vec<char>, data: &HashMap<String, Vec<String>>, start_point: String, counter_list: &mut Vec<usize>) {
    let mut step_count: u32 = 0;
    let mut point = start_point;
    let direction_map = HashMap::from([('L', 0), ('R', 1)]);
    let mut indexer: usize = 0;
    let max_index = directions.len() - 1;

    while !point.ends_with('Z') {
        let d: usize = direction_map[&directions[indexer]];
        point = data[&point][d].clone();
        if indexer < max_index {
            indexer += 1;
        } else {
            indexer = 0;
        }
        step_count += 1
    }

    counter_list.push(step_count as usize);
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn part_two() {

    let mut counter_list: Vec<usize> = vec![];

    let (directions, data) = parse_inputs(FILE);
    
    let start_points: Vec<String> = data.keys().filter(|x| x.ends_with('A')).cloned().collect();

    for start_point in start_points {
        step_counter(&directions, &data, start_point, &mut counter_list)
    }

    let lcm = lcm(&counter_list[0..counter_list.len()]);

    println!("Part Two: {:?}", lcm);
}

fn main() {
    part_one();
    part_two();
}
