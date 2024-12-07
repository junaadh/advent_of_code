use std::{fmt::Debug, fs, io::Read};

pub trait AocHelper: Debug {
    fn get_day(&self) -> Day;
}

pub trait AocDay: Debug + AocHelper {
    fn part1(&self, content: &str) -> usize;
    fn part2(&self, content: &str) -> usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Day {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
}

impl Day {
    pub fn get_input(self) -> String {
        format!("input/day{}.txt", self as u8)
    }

    pub fn get_example(self) -> String {
        format!("example/day{}.txt", self as u8)
    }
}

pub fn read_file(filename: String) -> String {
    let mut buffer = String::new();
    fs::File::open(filename)
        .expect("NONO this doesn't happen")
        .read_to_string(&mut buffer)
        .expect("Also nope not finna happen");

    buffer
}
