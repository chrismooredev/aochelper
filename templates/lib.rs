#![allow(unused_imports)]
use std::str::FromStr;
use std::fmt::Debug;
use itertools::Itertools;
use aoch::AoCDay;
#[cfg(test)] #[allow(unused_imports)]
use aoch::{DayPart, run_test, test_runner, daystr};

#[derive(Debug)]
pub struct Day{{DayNum}};

impl AoCDay for Day{{DayNum}} {
	type Data<'i> = Vec<String>;
	type Answer = usize;

	fn day(&self) -> u8 { {{DayNum}} }

	fn parse<'i>(&self, input: &'i str) -> Self::Data<'i> {
		aoch::parsing::from_lines(input).unwrap()
	}
	fn part1(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		todo!("Day {{DayNum}} Part 1")
	}
	fn part2(&self, _data: &mut Self::Data<'_>) -> Self::Answer {
		todo!("Day {{DayNum}} Part 2")
	}
}

#[cfg(test)]
const TEST_INPUT: &'static str = "
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

/*
#[test]
fn fuel_calc() {
	let cases = [
		(100756, 33583),
	];
	run_test(|n| DayMe::calc_fuel(*n), &cases);
}
*/

#[test]
fn part1() {
	let cases = [
		(TEST_INPUT, 0),
		(daystr!("{{DayNum}}"), 0),
	];
	test_runner::<Day{{DayNum}}, _>(Day{{DayNum}}, DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		(TEST_INPUT, 0),
		(daystr!("{{DayNum}}"), 0),
	];
	test_runner::<Day{{DayNum}}, _>(Day{{DayNum}}, DayPart::Part2, &cases);
}
