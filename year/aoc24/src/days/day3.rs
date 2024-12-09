// generated by GEN=1 cargo build
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
        parse_program(content)
    }

    fn part2(&self, content: &str) -> usize {
        let do_ = content
            .split("do()")
            .fold(Vec::new(), |mut acc, s| {
                let a = s.split("don't()").collect::<Vec<_>>();

                acc.push(a[0]);
                acc
            })
            .join(" ");

        parse_program(&do_)
    }
}

fn parse_program(input: &str) -> usize {
    let mut pos = 0;

    let mut acc = Vec::new();

    while let Some(start) = input[pos..].find("mul(") {
        let start = pos + start;

        // Search for the next closing parenthesis ')'
        let mut end = None;
        let mut search_pos = start + 4; // Start searching after "mul("

        while let Some(close) = input[search_pos..].find(')') {
            let close = search_pos + close;

            // Extract potential content between "mul(" and ")"
            let content = &input[start + 4..close]; // Skip "mul(" and up to ")"

            if let Some((left, right)) = extract_numbers(content) {
                acc.push((left, right));
                end = Some(close);
                break; // Stop searching for this pattern
            }

            // Otherwise, continue searching for a valid closing parenthesis
            search_pos = close + 1;
        }

        // If a valid closing parenthesis was found, move past it; otherwise, skip this "mul("
        pos = end.unwrap_or(start + 4);
    }

    acc.iter().fold(0, |acc, (a, b)| acc + (a * b))
}

fn extract_numbers(content: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = content.split(',').collect();
    if parts.len() == 2 {
        if let (Ok(left), Ok(right)) = (parts[0].trim().parse(), parts[1].trim().parse()) {
            return Some((left, right));
        }
    }
    None
}
