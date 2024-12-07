#[derive(Debug)]
pub struct AdventDay2 {
    pub day: utils::Day,
}

impl AdventDay2 {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            day: utils::Day::Two,
        })
    }
}

impl utils::AocHelper for AdventDay2 {
    fn get_day(&self) -> utils::Day {
        self.day
    }
}

impl utils::AocDay for AdventDay2 {
    fn part1(&self, content: &str) -> usize {
        let reports = parse_reports(content);

        reports.iter().fold(0, |acc, report| match is_safe(report) {
            true => acc + 1,
            false => acc,
        })
    }

    fn part2(&self, content: &str) -> usize {
        let reports = parse_reports(content);

        reports.iter().fold(0, |acc, report| {
            if is_safe(report) || remove_level(report, 0) {
                acc + 1
            } else {
                acc
            }
        })
    }
}

fn is_increasing(list: &[usize]) -> bool {
    match list {
        [] | [_] => true,
        [x, rest @ ..] => x < &rest[0] && is_increasing(rest),
    }
}

fn is_decreasing(list: &[usize]) -> bool {
    match list {
        [] | [_] => true,
        [x, rest @ ..] => x > &rest[0] && is_decreasing(rest),
    }
}

fn check_difference(list: &[usize]) -> bool {
    match list {
        [] | [_] => true,
        [x, rest @ ..] => {
            let diff = rest[0].abs_diff(*x);
            (1..=3).contains(&diff) && check_difference(rest)
        }
    }
}

fn remove_level(list: &[usize], idx: usize) -> bool {
    if idx >= list.len() {
        return false;
    }

    match list {
        x if is_safe(x) => true,
        _ => {
            let reduced = list
                .iter()
                .enumerate()
                .filter_map(|(x, v)| if x != idx { Some(*v) } else { None })
                .collect::<Vec<_>>();
            if is_safe(&reduced) {
                true
            } else {
                remove_level(list, idx + 1)
            }
        }
    }
}

fn is_safe(list: &[usize]) -> bool {
    (is_increasing(list) || is_decreasing(list)) && check_difference(list)
}

fn parse_reports(content: &str) -> Vec<Vec<usize>> {
    content.lines().fold(Vec::new(), |mut acc, line| {
        let lvls = line
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        acc.push(lvls);

        acc
    })
}
