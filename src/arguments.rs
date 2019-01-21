use std::{result::Result};

use rolls::{
  RollConfig,
  RollType,
  RangeType
};

/// Parse cli arguments and create a vector of Roll configs;
pub fn parse(mut args: std::env::Args) -> Vec<RollConfig> {
  /// skip filename
  args.next();
  let mut parsed_arguments: Vec<RollConfig> = args
    .map(|arg| generate_config(&arg))
    .collect();
  // given process did not yield anything, there were no valid arguments -> roll 100;
  if parsed_arguments.len() == 0 {
    let roll_type = RollType::Range(RangeType::PositiveToPositive);
    parsed_arguments.push(RollConfig::new(1, 100, 1, roll_type));
  }
  parsed_arguments
}

/// Process arguments, creating configs for Roll
/// Determine type of roll, die or range, fork.
fn generate_config(arg: &str) -> RollConfig {
  arg.trim();
  if arg.contains("d") {
    return generate_die_config(arg);
  }
  generate_range_config(arg)
}

/// Generate range config logic:
/// Determine what kind of range roll is that
/// Fork to correct range parser
/// Construct config
fn generate_range_config(arg: &str) -> RollConfig {
  let range_type = detect_range_type(arg);
  let parsed_arguments = match range_type {
    RangeType::Positive => parse_string_vector(&["0", arg]),
    RangeType::Negative => parse_string_vector(&["0", arg]),
    RangeType::NegativeToNegative => parse_neg_start_range(arg),
    RangeType::NegativeToPositive => parse_neg_start_range(arg),
    RangeType::PositiveToNegative => parse_pos_start_range(arg),
    RangeType::PositiveToPositive => parse_pos_start_range(arg)
  };
  let lower = parsed_arguments[0];
  let upper = parsed_arguments[1];
  RollConfig::new(lower, upper, 1, RollType::Range(range_type))
}

/// Given range starts with negative number, we need to split at the second dash;
fn parse_neg_start_range(arg: &str) -> Vec<i32> {
  let dashes: Vec<(_, &str)> = arg.match_indices("-").collect();
  let split_at = dashes[1].0;
  let first = &arg[.. split_at];
  let second = &arg[split_at + 1 ..];
  parse_string_vector(&[first, second])
}

/// Given range starts with positive number, we need to split at the first dash;
fn parse_pos_start_range(arg: &str) -> Vec<i32> {
  let dashes: Vec<(_, &str)> = arg.match_indices("-").collect();
  let split_at = dashes[0].0;
  let first = &arg[.. split_at];
  let second = &arg[split_at + 1 ..];
  parse_string_vector(&[first, second])
}

/// Handling the range roll. Two possible inputs
/// 1. arg is a parseable int
/// 2. arg has two ints separated by a dash.
/// Second option is complicated by the fact that total number of dashes could be three: range of two negative numbers.
/// To address that, we can do match_indices("-") and see the number of dashes as well as their indices.
/// Given there are no dashes we have scenario #1
/// Given there is one dash and it's index is 0, we have scenario #1
/// Given there are two dashes and index of the first one is 0 it is a range of negative to positive.
/// Given there are two dashes and index of the first one is not 0, it is a reverse range of negative to positive.
/// Given there are three dashes it is a range of two negative numbers.
/// All other inputs are considered invalid.
fn detect_range_type(arg: &str) -> RangeType {
  let dashes: Vec<(_, &str)> = arg.match_indices("-").collect();
  let dash_count = dashes.len();
  match dash_count {
    0 => return RangeType::Positive,
    1 =>  match dashes[0].0 {
      0 => return RangeType::Negative,
      _ => return RangeType::PositiveToPositive,
      }
    2 => match dashes[0].0 {
      0 => return RangeType::NegativeToPositive,
      _ => return RangeType::PositiveToNegative,
      }
    3 => return RangeType::NegativeToNegative,
    _ => panic!("Unable to determine roll type")
  }
}

/// Runs through slice of str pointers, attempts to parse each
/// Unless paniced on invalid inputs, produces ordered list
fn parse_string_vector(input: &[&str]) -> Vec<i32> {
  let mut parsed = Vec::new();
  for s in input.iter() {
    parsed.push(
      s.parse().expect("Not an integer!")
    );
  }
  parsed.sort();
  parsed
}



/// Handling the Die roll. Two possible inputs:
/// 1. arg starts with d/D, followed by parseable uint
/// 2. arg starts with uint8, followed by d/D, followed by parseable uint
/// All other inputs are considered invalid.
fn generate_die_config(arg: &str) -> RollConfig {
  /// wildcard for both lower and uppercase D's
  let d: &[_] = &['D', 'd'];
  let split_at = arg.find(d).unwrap();
  let times_to_roll = match split_at {
    0 => 1,
    _ => arg[..split_at].parse().unwrap()
  };
  let upper = arg[split_at+1..].parse().unwrap();
  let roll_type = RollType::Die(format!("{}d{}", times_to_roll, upper));
  RollConfig::new(1, upper, times_to_roll, roll_type)
}

#[cfg(test)]
mod parse_tests {
  use super::*;

  mod detect_range_type_tests {
    use super::{detect_range_type, RangeType};

    #[test]
    /// Given input has no dashes, detects positive integer roll
    fn detect_positive() {
      assert_eq!(detect_range_type("123"), RangeType::Positive)
    }
    #[test]
    /// Given input has one dash, when its index is 0, detects negative integer roll
    fn detect_negative() {
      assert_eq!(detect_range_type("-100"), RangeType::Negative)
    }
    #[test]
    /// Given input has one dash, when its index is not 0, detects positive to positive roll
    fn detect_positive_to_positive() {
      assert_eq!(detect_range_type("100-101"), RangeType::PositiveToPositive)
    }
    #[test]
    /// Given input has two dashes, when index of the first dash is not zero, detects positive to negative roll.
    fn detect_positive_to_negative() {
      assert_eq!(detect_range_type("100--100"), RangeType::PositiveToNegative)
    }
    #[test]
    /// Given input has two dashes, when index of the first dash is zero, detects negative to positive roll.
    fn detect_negative_to_positive() {
      assert_eq!(detect_range_type("-100-100"), RangeType::NegativeToPositive)
    }
    #[test]
    /// Given input has three dashes, detects negative to negative roll.
    fn detect_negative_to_negative() {
      assert_eq!(detect_range_type("-100--88"), RangeType::NegativeToNegative)
    }
  }

  mod parse_string_vector_tests {
    use super::parse_string_vector;

    #[test]
    /// Given slice of valid str inputs, when first is higher than second
    /// Produced vector would be [lower, higher];
    fn orders_many() {
      let input = vec!["-100", "-1010"];
      let result = parse_string_vector(&input);

      assert_eq!(result, vec![-1010, -100]);
    }
    #[test]
    #[should_panic(expected="Not an integer!")]
    /// Given slice of invalid inputs, panics with "Not an integer"
    fn panic_mode() {
      let input = vec!["abc", "x80-_"];
      parse_string_vector(&input);
    }
  }
}

