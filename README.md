# Advent of Code 2023 - Rust

Mostly a learning experience for Rust: solutions will be pretty hacky and non-idiomatic. But I might revisit and try to optimise later on (e.g. using threads or proper error handling).

### Notes
- Day 2: Parsing was a nightmare. The logic I came up with fine, but fought with the compiler for a long time to get the outputs I needed. Had to google how to get the regex into a vector for each color (the whole filter_map() and flatten()). I'll go back and try to figure out why my approach didn't work: was getting all kinds of Results and Options and Regex types that I couldn't operate on.
  - Part 2 was pretty easy after copying the code. But I think there could be some much better refactoring of the logic. I should have parsed the line in a function, and checked if possible separately.
- Day 3: The 2d array search is interesting. I ended up using the same logic yesterday to walk through the line and parse numbers from the individual digits.
 - Had to find a method online to come up with a creative way to ID which gears are validly associated with parts. Solution was to find any gear near a part and use the coordinates of that gear in a HashMap. Then can look for any gear that has 2 parts associated with it.
- Day 4: Could make things a lot more concise by splitting the string, but I wanted to play about a little with the parsing logic (moving bounds looking for digits and :| symbols). I think the set intersection method is the best way I can think of to find the matching elements: it takes one pass to build the sets and then lookups are quick.
  - Part 2 was pretty confusing, but the logic worked. Spent a long time fighting the compiler with the HashMap and updating values.