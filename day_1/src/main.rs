use std::fs::File;
use std::io::{BufRead, BufReader};

/*
The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover.
On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

Assuming all digits will be +ve integers, so using unsigned int for running total counter.
We cannot assume there is more than one digit in a line. In that case, first and last digits are the same.
Handled that by assigning first and last to the first character seen, then moving on to re-assign second if we see another digit.

Answer to Part 1 was 53_386
Answer to Part 2 was 53_312

Interesting alternative approach here that uses two passes of string (look from the front to the first number, and then from the back to the last number): 
https://www.reddit.com/r/adventofcode/comments/1886cjs/2023_day_1_part_2_dont_overthink_it/
Could possibly just do a regex for one pass, but I'd have to think about how that would work...
Maybe something starting with a capturing group like this (?:zero|one|two|three|four|five|six|seven|eight|nine|\d)
*/

const FILE: &str = "../inputs/day1_1.txt";

fn part_one() {
    let file = File::open(FILE).expect("Cannot read file");
    let reader = BufReader::new(file);

    let mut sigma: u32 = 0;

    for line in reader.lines() {
        let mut first_digit: i32 = -1;
        let mut second_digit: u32 = 0;

        for c in line.unwrap().chars() {
            if c.is_ascii_digit() && first_digit == -1 {
                first_digit = c.to_digit(10).unwrap() as i32;
                second_digit = c.to_digit(10).unwrap();
            }
            if c.is_ascii_digit() {
                second_digit = c.to_digit(10).unwrap();
            }
        }

        sigma += first_digit as u32 * 10 + second_digit
    }

    println!("Sum of lines is {}", sigma);
}

fn part_two() {
    let file = File::open(FILE).expect("Cannot read file");
    let reader = BufReader::new(file);

    let mut sigma: u32 = 0;

    for line in reader.lines() {
        let mut first_digit: i32 = -1;
        let mut second_digit: u32 = 0;
        let line = line
            .unwrap()
            .replace("zero", "z0o")
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");

        for c in line.chars() {
            if c.is_ascii_digit() && first_digit == -1 {
                first_digit = c.to_digit(10).unwrap() as i32;
                second_digit = c.to_digit(10).unwrap();
            }
            if c.is_ascii_digit() {
                second_digit = c.to_digit(10).unwrap();
            }
        }

        sigma += first_digit as u32 * 10 + second_digit
    }

    println!("Sum of lines is {}", sigma);
}

fn main() {
    part_one();
    part_two();
}
