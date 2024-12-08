mod days;

pub use days::*;

#[derive(Debug)]
pub struct AOC24 {
    pub days: Vec<Box<dyn utils::AocDay>>,
    test: bool,
}

impl AOC24 {
    pub fn new() -> Self {
        Self {
            days: vec![AdventDay1::new(), AdventDay2::new(), AdventDay3::new()],
            test: false,
        }
    }

    pub fn _test() -> Self {
        let mut s = Self::new();
        s.test = true;
        s
    }

    pub fn run(self) {
        for (idx, day) in self.days.iter().enumerate() {
            let name = if self.test {
                day.get_day().get_example()
            } else {
                day.get_day().get_input()
            };

            let file_content = utils::read_file(name);

            println!(
                "Day{}\n{}\nPart1: {}\nPart2: {}\n\n",
                idx + 1,
                "*".repeat(5),
                day.part1(&file_content),
                day.part2(&file_content)
            )
        }
    }
}

impl Default for AOC24 {
    fn default() -> Self {
        Self::new()
    }
}
