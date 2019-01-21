extern crate rand;
use self::rand::Rng;

#[derive(Debug)]
pub struct RollConfig {
  lower_bound: i32,
  upper_bound: i32,
  times_to_roll: u8,
  roll_type: RollType
}

impl RollConfig {
  pub fn new(a: i32, b: i32, r: u8, t: RollType) -> RollConfig {
    RollConfig {
      lower_bound: a,
      upper_bound: b,
      times_to_roll: r,
      roll_type: t
    }
  }
  /// Instantiates new Roll struct from config variables
  pub fn roll(&self) -> Roll {
    let mut roll_results = Vec::new();
    let mut counter = 0;
    while counter < self.times_to_roll {
      let roll_result = rand::thread_rng()
          .gen_range(self.lower_bound, self.upper_bound);
      roll_results.push(roll_result);
      counter += 1;
    }
    Roll::new(self.roll_type.clone(), roll_results)
  }
}

#[derive(Debug)]
pub struct Roll {
  roll_type: RollType,
  roll_results: Vec<i32>,
  roll_sum: i32
}

impl Roll {
  fn new(roll_type: RollType, roll_results: Vec<i32>) -> Roll {
    let roll_sum = roll_results.iter().sum();
    Roll {
      roll_type,
      roll_results,
      roll_sum
    }
  }
}

#[derive(Debug, Clone)]
pub enum RollType {
  Die(String),
  Range(RangeType),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RangeType {
  Positive,
  Negative,
  PositiveToPositive,
  PositiveToNegative,
  NegativeToPositive,
  NegativeToNegative
}
