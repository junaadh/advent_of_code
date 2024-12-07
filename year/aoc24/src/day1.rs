#[derive(Debug)]
pub struct AdventDay1 {
    pub day: utils::Day,
}

impl AdventDay1 {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            day: utils::Day::One,
        })
    }
}

impl utils::AocHelper for AdventDay1 {
    fn get_day(&self) -> utils::Day {
        self.day
    }
}

impl utils::AocDay for AdventDay1 {
    fn part1(&self, content: &str) -> usize {
        let mut content = content
            .lines()
            .map(|line| {
                let parts = line
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                (parts[0], parts[1])
            })
            .collect::<(Vec<_>, Vec<_>)>();
        content.0.sort();
        content.1.sort();

        content
            .0
            .iter()
            .zip(content.1.iter())
            .fold(0, |acc, (a, b)| acc + a.abs_diff(*b))
    }

    fn part2(&self, content: &str) -> usize {
        let (mut col1, col2) = content
            .lines()
            .map(|line| {
                let parts = line
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                (parts[0], parts[1])
            })
            .collect::<(Vec<_>, Vec<_>)>();

        col1.sort();

        col1.iter().fold(0, |acc, x| {
            let count = col2.iter().filter(|&y| x == y).count();
            acc + (x * count)
        })
    }
}
