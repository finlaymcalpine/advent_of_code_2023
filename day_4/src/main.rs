use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
The Elf leads you over to the pile of colorful cards. There, you discover dozens of scratchcards, all with their opaque covering already scratched off.
Picking one up, it looks like each card has two lists of numbers separated by a vertical bar (|): a list of winning numbers and then a list of numbers you have.
You organize the information into a table (your puzzle input).

As far as the Elf has been able to figure out, you have to figure out which of the numbers you have appear in the list of winning numbers.
The first match makes the card worth one point and each match after the first doubles the point value of that card.

First step is to read the file line by line and create a set of the digits after the ':' (we don't need the card number) and before the '|' and then gather all the number after '|' into another set.
We then need to find the number of matching numbers, N, for each card (the size of the set intersection between winning vals and our numbers) and then 2^(N-1) for each card. Sum up those values.

There is a far shorter way to get the numbers, which is to split the line on ':', and then split the line on '|', and then collect that into two vecs split on whitespace.
Then we can either compare the vecs or create the hashsets
*/

const FILE: &str = "../inputs/day4_1.txt";

fn parse_line(s: &str) -> u32 {
    let mut winning: HashSet<u32> = HashSet::new();
    let mut values: HashSet<u32> = HashSet::new();

    let c: Vec<char> = s.chars().collect::<Vec<_>>();

    let mut left: usize = 0;
    let mut right: usize;
    let mut colon_seen: bool = false;
    let mut glyph_seen: bool = false;

    'outer: while left < c.len() {
        if !colon_seen && c[left] != ':' {
            left += 1;
            continue 'outer;
        }

        colon_seen = true;

        if !glyph_seen && c[left] != '|' {
            if !c[left].is_numeric() {
                left += 1;
                continue 'outer;
            }

            right = left;

            while right < c.len() && c[right].is_numeric() {
                right += 1
            }

            let n: u32 = c[left..right]
                .iter()
                .copied()
                .collect::<String>()
                .parse()
                .unwrap();

            winning.insert(n);

            left = right;

            continue 'outer;
        }

        glyph_seen = true;

        if !c[left].is_numeric() {
            left += 1;
            continue 'outer;
        }

        right = left;

        while right < c.len() && c[right].is_numeric() {
            right += 1
        }

        let n: u32 = c[left..right]
            .iter()
            .copied()
            .collect::<String>()
            .parse()
            .unwrap();

        values.insert(n);

        left = right;

        continue 'outer;
    }

    let matches = winning
        .intersection(&values)
        .collect::<HashSet<&u32>>()
        .len() as u32;

    return matches;
}

fn points(m: &u32) -> u32 {
    return if m > &0 { 2_u32.pow(m - 1) } else { 0 };
}

fn part_one() {
    let file = File::open(FILE).expect("Cannot read file");
    let reader = BufReader::new(file);

    let mut running_total: u32 = 0;

    for line in reader.lines() {
        let s1 = line.unwrap();
        let matches = parse_line(&s1);

        let points = points(&matches);

        running_total += points;
    }

    println!("Total points is {}", running_total);
}

fn part_two() {
    let file = File::open(FILE).expect("Cannot read file");
    let reader = BufReader::new(file);

    let mut match_vec: Vec<u32> = vec![];
    let mut card_count: HashMap<usize, u32> = HashMap::new();

    for line in reader.lines() {
        let s1 = line.unwrap();
        let matches = parse_line(&s1);
        match_vec.push(matches);
    }

    for i in 0..match_vec.len() { 
        card_count.insert(i, 1);
    }

    for (i, matches) in match_vec.into_iter().enumerate() {
        let value = card_count[&i];
        for j in i + 1..i + matches as usize + 1 {
            if let Some(x) = card_count.get_mut(&j) {
                *x += value;
            }
        }
    }

    let totals: u32 = card_count.values().sum();

    println!("total cards is {}", totals);
}

fn main() {
    part_one();
    part_two();
}
