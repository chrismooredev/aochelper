use std::io;
use aochelper::{AoCDay, DayResult};
use aochelper::{run_day, daystr};
#[cfg(test)]
use aochelper::{DayPart, run_test, test_runner};

fn main() {
	run_day::<Day{{DayNum}}>(daystr!("{{DayNum}}"));
}

struct Day{{DayNum}} {
}
impl Day{{DayNum}} {
}

impl AoCDay for Day{{DayNum}} {
	type Answer = usize;

	fn day() -> u8 { {{DayNum}} }
	fn name() -> &'static str { "{{DayName}}" }
	fn parse(mut input: Box<dyn io::Read>) -> DayResult<Self> {
		let mut s = String::new();
		input.read_to_string(&mut s)?;
		let lines_as_nums: Result<Vec<usize>, std::num::ParseIntError> = s.lines()
			.map(|l| l.parse::<usize>()).collect();

		unimplemented!();
	}
	fn part1(&mut self) -> Self::Answer {
		unimplemented!();
	}
	fn part2(&mut self) -> Self::Answer {
		unimplemented!();
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

#[test]
fn part1() {
	let cases = [
		(daystr!("{{DayNum}}"), 0),
	];
	test_runner::<Day{{DayNum}}, _>(DayPart::Part1, &cases);
}
#[test]
fn part2() {
	let cases = [
		(daystr!("{{DayNum}}"), 0),
	];
	test_runner::<Day{{DayNum}}, _>(DayPart::Part2, &cases);
}
