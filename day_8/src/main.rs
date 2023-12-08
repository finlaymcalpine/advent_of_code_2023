use regex::Regex;
use std::collections::HashMap;
use std::fs;

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

fn main() {
    part_one();
}
