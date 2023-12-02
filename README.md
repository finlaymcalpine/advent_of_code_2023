# advent_of_code_2023
Advent of Code 2023 in Rust

Mostly a learning experience for Rust: solutions will be pretty hacky and non-idiomatic. But I might revisit and try to optimise later on (e.g. using threads or proper error handling).

### Notes
- Day 2: Parsing was a nightmare. The logic I came up with pretty fast, but fought with the compiler for a loooong time to get the outputs I needed. Had to google how to get the regex into a vector for each color. I'll go back and try to figure out why my approach didn't work - was getting all kinds of Results and Options and Regex types that I couldn't operate on.
  - Part 2 was pretty easy after copying the code. But I think there could be some much better refactoring of the logic. I should have parsed the line in a function, and checked if possible separately.
