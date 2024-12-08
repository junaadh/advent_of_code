#!/bin/bash

# Check if a day argument is provided, otherwise use the current day

if [[ -n "$1" ]]; then
  current_day=$1
fi

# Get the current year
current_year=$(date +%y)

# Convert the day number to words (using a simple array for the first 31 days)
day_words=("Zero" "One" "Two" "Three" "Four" "Five" "Six" "Seven" "Eight" "Nine" "Ten" 
"Eleven" "Twelve" "Thirteen" "Fourteen" "Fifteen" "Sixteen" "Seventeen" "Eighteen" 
"Nineteen" "Twenty" "TwentyOne" "TwentyTwo" "TwentyThree" "TwentyFour" 
"TwentyFive")

# Get the word for the current day
current_day_word=${day_words[$current_day]}

# Define the Rust code template
RUST_CODE='
#[derive(Debug)]
pub struct AdventDay3 {
  pub day: utils::Day,
}

impl AdventDay3 {
  pub fn new() -> Box<Self> {
      Box::new(Self {
          day: utils::Day::Three,
      })
  }
}

impl utils::AocHelper for AdventDay3 {
  fn get_day(&self) -> utils::Day {
      self.day
  }
}

impl utils::AocDay for AdventDay3 {
  fn part1(&self, content: &str) -> usize {
    todo!()
  }

  fn part2(&self, content: &str) -> usize {
    todo!()
  }
}
'

# Replace placeholders in the Rust code
parsed_code=$(echo "$RUST_CODE" | sed "s/3/$current_day/g; s/Three/$current_day_word/g")

advent=$(echo "AdventDay$current_day::new()")

# Check if input/day$current_day.txt or example/day$current_day.txt exists
if [[ -e "input/day$current_day.txt" || -e "example/day$current_day.txt" || $(grep -q "$advent" "year/aoc$current_year/src/lib.rs") ]]; then
  echo "Files already exist or advent already defined. No changes made."
  exit 0
fi

echo "mod day$current_day;
pub use day$current_day::*;
"  >> "year/aoc$current_year/src/days.rs"
sed -i "s/\(vec!\[.*\)\]/\1, $advent]/" "year/aoc$current_year/src/lib.rs"

echo "$parsed_code" > "year/aoc$current_year/src/days/day$current_day.rs"
touch "input/day$current_day.txt" "example/day$current_day.txt"
