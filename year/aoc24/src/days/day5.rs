use std::collections::{HashMap, VecDeque};

// generated by GEN=1 cargo build
#[derive(Debug)]
pub struct AdventDay5 {
    pub day: utils::Day,
}

impl AdventDay5 {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            day: utils::Day::Five,
        })
    }
}

impl utils::AocHelper for AdventDay5 {
    fn get_day(&self) -> utils::Day {
        self.day
    }
}

impl utils::AocDay for AdventDay5 {
    fn part1(&self, content: &str) -> usize {
        let (rules, updates) = parse_rules_updates(content);

        let graph = build_graph(&rules);

        updates
            .iter()
            .filter(|x| is_valid_order(&graph, x))
            .map(|x| middle_page(x))
            .sum::<usize>()
    }

    fn part2(&self, content: &str) -> usize {
        let (rules, updates) = parse_rules_updates(content);

        let graph = build_graph(&rules);

        updates
            .iter()
            .filter(|x| !is_valid_order(&graph, x))
            .map(|x| {
                let sorted = topological_sort(&graph, x);
                middle_page(&sorted)
            })
            .sum::<usize>()
    }
}

fn parse_rules_updates(content: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let parse_rule = |line: &str| {
        let parts = line.trim().split("|").map(|x| x.trim()).collect::<Vec<_>>();
        if parts.len() != 2 {
            panic!("No?")
        }

        (
            parts[0].parse::<usize>().expect("Failed to parse rule1"),
            parts[1].parse::<usize>().expect("Failed to parse rule2"),
        )
    };

    let parse_update = |line: &str| {
        line.trim()
            .split(',')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    };

    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let mut in_update = false;

    for line in content.lines() {
        let line = line.trim();

        if line.is_empty() {
            in_update = true;
            continue;
        }

        if in_update {
            updates.push(parse_update(line));
        } else {
            rules.push(parse_rule(line));
        }
    }

    (rules, updates)
}

fn build_graph(rules: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
    rules.iter().fold(HashMap::new(), |mut acc, &(x, y)| {
        acc.entry(x).or_default().push(y);

        acc
    })
}

fn is_valid_order(graph: &HashMap<usize, Vec<usize>>, update: &[usize]) -> bool {
    let mut positions = HashMap::new();
    for (i, &page) in update.iter().enumerate() {
        positions.insert(page, i);
    }

    for (&x, dependencies) in graph.iter() {
        if let Some(&x_pos) = positions.get(&x) {
            for &y in dependencies {
                if let Some(&y_pos) = positions.get(&y) {
                    if x_pos >= y_pos {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn topological_sort(graph: &HashMap<usize, Vec<usize>>, update: &[usize]) -> Vec<usize> {
    // Build an in-degree map and adjacency list for the update-specific subgraph
    let mut in_degree = HashMap::new();
    let mut adj_list = HashMap::new();

    for &page in update {
        in_degree.insert(page, 0);
        adj_list.insert(page, Vec::new());
    }

    for (&x, dependencies) in graph.iter() {
        if update.contains(&x) {
            for &y in dependencies {
                if update.contains(&y) {
                    *in_degree.entry(y).or_insert(0) += 1;
                    adj_list.entry(x).or_default().push(y);
                }
            }
        }
    }

    // Perform a topological sort
    let mut queue = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&page, _)| page)
        .collect::<VecDeque<_>>();
    let mut sorted = Vec::new();

    while let Some(page) = queue.pop_front() {
        sorted.push(page);
        if let Some(neighbors) = adj_list.get(&page) {
            for &neighbor in neighbors {
                if let Some(deg) = in_degree.get_mut(&neighbor) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    sorted
}

fn middle_page(update: &[usize]) -> usize {
    let n = update.len() / 2;
    // println!(
    //     "{n} => {update:?} => {}",
    //     update.get(n).cloned().unwrap_or_default()
    // );
    update.get(n).cloned().unwrap_or_default()
}
