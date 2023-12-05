use std::fs;
use std::collections::HashMap;
//use std::env;

/*
The engine schematic (your puzzle input) consists of a visual representation of the engine.
There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol,
even diagonally, is a "part number" and should be included in your sum.
(Periods (.) do not count as a symbol.)

Seems like we are trying to collect digits into numbers, and then look around each number (across three lines) to understand if it's a valid part number.

I have the logic working here, but it's only looking at the first line of the input and I don't know why.
Either it's not looking beyond that line, or it's only pushing values from that line and no other.
It was a stupid mistake where I had break instead of continue in the find_parts function.
*/

const FILE: &str = "../inputs/day3_1.txt";

fn input_parser(file_name: &str) -> Vec<Vec<char>> {
    let file = fs::read_to_string(file_name).expect("File read failure");
    let input = file
        .lines();
    let matrix = input
        .map(|line| line.chars().collect::<Vec<_>>() )
        .filter(|line| !line.is_empty() )
        .collect::<Vec<_>>();
    matrix
}

fn find_parts(inp: Vec<Vec<char>>, parts_vec: &mut Vec<u32>) {
    'next_line: for (r, line) in inp.iter().enumerate() {
        let mut c1 = 0;
        let mut c2;

        while c1 < line.len() {
            while !line[c1].is_numeric() {
                c1 += 1;
                if c1 >= line.len() {
                    continue 'next_line;
                }
            }

            c2 = c1;

            while c2 < line.len() && line[c2].is_numeric() {
                c2 += 1;
            }

            let n: String = line[c1..c2].iter().copied().collect();
            let n: u32 = n.parse().unwrap();

            let check_upper = if r > 0 { r - 1 } else { 0 };
            let check_lower = (r + 2).min(inp.len());
            let left_col = if c1 > 0 { c1 - 1 } else { 0 };
            let right_col = (c2 + 1).min(line.len());

            'bounds: for i in check_upper..check_lower {
                for j in left_col..right_col {
                    if !inp[i][j].is_numeric() && inp[i][j] != '.' {
                        parts_vec.push(n);
                        break 'bounds;
                    }
                }
            }
            c1 = c2;
        }
    }
}

fn find_gears(inp: Vec<Vec<char>>) -> HashMap<(usize, usize), Vec<u32>> {
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    'next_line: for (r, line) in inp.iter().enumerate() {
        let mut c1 = 0;
        let mut c2;

        while c1 < line.len() {
            while !line[c1].is_numeric() {
                c1 += 1;
                if c1 >= line.len() {
                    continue 'next_line;
                }
            }

            c2 = c1;

            while c2 < line.len() && line[c2].is_numeric() {
                c2 += 1;
            }

            let n: String = line[c1..c2].iter().copied().collect();
            let n: u32 = n.parse().unwrap();

            let check_upper = if r > 0 { r - 1 } else { 0 };
            let check_lower = (r + 2).min(inp.len());
            let left_col = if c1 > 0 { c1 - 1 } else { 0 };
            let right_col = (c2 + 1).min(line.len());

            'bounds: for i in check_upper..check_lower {
                for j in left_col..right_col {
                    if !inp[i][j].is_numeric() && inp[i][j] != '.' {
                        if inp[i][j] == '*' {
                            if let Some(x) = gears.get_mut(&(i,j)) {
                                x.push(n);
                            }
                            else {
                                gears.insert((i, j), vec![n]);
                            }
                        }
                        break 'bounds;
                    }
                }
            }
            c1 = c2;
        }
    }
    return gears
}

fn part_one() {
    let matrix: Vec<Vec<char>> = input_parser(FILE);
    let mut parts: Vec<u32> = vec![];
    
    find_parts(matrix, &mut parts);
    
    let total_parts: u32 = parts.iter().sum();
    println!("Sum of parts is {}", total_parts)
}

fn part_two() {
    let matrix: Vec<Vec<char>> = input_parser(FILE);
    
    let gears = find_gears(matrix);

    // We made a HashMap when we found a part that had a * symbol around it. We noted the coordinates of that * symbol, and then added the part to a vector associated with that * symbol coordinate as the key.
    // This means that we can look for any * symbol coordinate that has two part numbers in the vector associated with it, and then we know it's a valid gear.
    // The gear/part association is all done in the find_gears() function, and then below we extract the gears that have two parts associated with them, and sumproduct the relevant part values.
    
    let gear_ratios: Vec<u32> = gears.into_values().filter(|m| m.len() == 2).map(|m| m.into_iter().product::<u32>()).collect();
    let total_ratios: u32 = gear_ratios.iter().sum();
    println!("Sum of gear ratios is {}", total_ratios);
}

fn main() {
    part_one();
    part_two();
}
