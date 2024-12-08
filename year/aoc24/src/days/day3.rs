
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
