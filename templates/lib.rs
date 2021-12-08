#![allow(unused_imports)]
use std::str::FromStr;
use itertools::Itertools;
use aoch::{AoCDay, DayResult};
use aoch::DayError;
#[cfg(test)] #[allow(unused_imports)]
use aoch::{DayPart, run_test, test_runner, daystr};

pub struct Day{{DayNum}} {
}
impl Day{{DayNum}} {
}

impl AoCDay for Day{{DayNum}} {
	type Answer = usize;

	fn day() -> u8 { {{DayNum}} }
	fn name() -> &'static str { "{{DayName}}" }

	fn parse(input: &str) -> DayResult<Self> {
		aoch::parsing::from_lines(input)	
			.map(|nums| Day{{DayNum}} { nums })
			.map_err(|e| e.into())
	}
	fn part1(&mut self) -> DayResult<Self::Answer> {
		Err(DayError::Unimplemented)
	}
	fn part2(&mut self) -> DayResult<Self::Answer> {
		Err(DayError::Unimplemented)
	}
}

/*
#[test]
fn fuel_calc() {
	let cases = [
		(100756, 33583),
	];
	run_test(|n| DayMe::calc_fuel(*n), &cases);
}
*/

#[cfg(test)]
const TEST_INPUT: &'static str = "
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

#[test]
fn part1() {
	let cases = [
		(TEST_INPUT, 0),
		(daystr!("{{DayNum}}"), 0),
	];
	test_runner::<Day{{DayNum}}, _>(DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		(TEST_INPUT, 0),
		(daystr!("{{DayNum}}"), 0),
	];
	test_runner::<Day{{DayNum}}, _>(DayPart::Part2, &cases);
}
