use std::fs::File;
use std::io::{BufRead, BufReader};

/*

*/

const FILE: &str = "../inputs/day11_1.txt";

fn grid_extender(file: &str) -> Vec<Vec<char>> {
    let mut grid = vec![];

    let file = File::open(file).expect("Cannot read file");
    let reader = BufReader::new(file);

    // create rows and duplicate if a row is all empty space
    for line in reader.lines() {
        let s1: Vec<char> = line.unwrap().chars().collect();
        if s1.iter().all(|x| *x == '.') {
            let s2 = s1.clone();
            grid.push(s1);
            grid.push(s2);
        } else {
            grid.push(s1);
        }
    }

    let mut i: usize = 0;

    'columns: loop {
        if i >= grid[0].len() {
            break;
        }
        for row in grid.iter() {
            if row[i] != '.' {
                i += 1;
                continue 'columns;
            }
        }
        for j in 0..grid.len() {
            grid[j].insert(i + 1, '.');
        }
        i += 2;
    }

    grid
}

fn find_galaxies(grid: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = vec![];

    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == '#' {
                galaxies.push((i, j));
            }
        }
    }

    galaxies
}

fn manhattan_distance(p1: &(usize, usize), p2: &(usize, usize)) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn part_one() {
    let grid = grid_extender(FILE);
    let galaxies = find_galaxies(grid);
    let mut sigma: usize = 0;

    for (i, point) in galaxies.iter().enumerate() {
        for other in &galaxies[i+1..galaxies.len()] {
            sigma += manhattan_distance(point, other);
        }
    }
    println!("Part One: {}", sigma);
}

fn main() {
    part_one();
}
