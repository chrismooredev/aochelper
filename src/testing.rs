use std::fmt;
use std::panic;

use colored::Colorize;

use crate::{AoCDay, DayPart};

/// Lines shown if parsing/logic panics
const LINES: usize = 10;

// Used to test a day's specific part
pub fn test_runner<'a, Day, Ans>(day: Day, part: DayPart, cases: &[(&str, Ans)])
where
	Day: AoCDay,
	Ans: ToString + fmt::Debug + PartialEq<Day::Answer> + Eq,
{
	run_test(|&input| {
		let mut data: Day::Data = match panic::catch_unwind(move || day.parse(input)) {
			Ok(ds) => ds,
			Err(e) => {
				let input = (input.len() <= 200)
					.then(|| input).unwrap_or("<too long to display>");
				panic!(
					"error parsing input: `{}` (input = {:?})",
					format!("{:?}", e).red(),
					input.bold(),
				)
			}
		};

		let result: Day::Answer = match part {
			DayPart::Part1 => day.part1(&mut data),
			DayPart::Part2 => day.part2(&mut data),
		};

		result
	}, cases);
}

// Used to test a specific function in a day
pub fn run_test_pretty<I, E, O, F>(func: F, cases: &[(I, E)])
where
	I: fmt::Debug + Sized + ToString,
	E: fmt::Debug + PartialEq<O> + Eq,
	O: fmt::Debug,
	F: for<'a> Fn(&'a I) -> O,
{
	for (i, (case, expected)) in cases.iter().enumerate() {
		let generated = func(case);
		if *expected != generated {
			// limit input string to 10 lines
			let short_input: String = {
				
				let input = case.to_string();
				let mut newlines = input.char_indices()
					.filter(|&(_, c)| c == '\n')
					.skip(LINES);
		
				match newlines.next() {
					None => input,
					Some((i, _)) => {
						let rest = newlines.count();
						let mut sinput = input[..i].to_string();
						sinput += &format!("\n...<{} more lines>...", rest);
						sinput
					}
				}
			};

			panic!(
				"input #{} - '{}': expected `{}` got `{}` for input #{}",
				i,
				short_input.bold(),
				format!("{:?}", expected).green(),
				format!("{:?}", generated).red(),
				i,
			);
		};
	}
}

pub fn run_test<I, E, O, F>(func: F, cases: &[(I, E)])
where
	I: fmt::Debug + Sized,
	E: fmt::Debug + PartialEq<O> + Eq,
	O: fmt::Debug,
	F: for<'a> Fn(&'a I) -> O,
{
	for (i, (case, expected)) in cases.iter().enumerate() {
		let generated = func(case);
		if *expected != generated {
			// limit input string to 10 lines
			let short_input: String = {
				
				let input = format!("{:?}", case);
				let mut newlines = input.char_indices()
					.filter(|&(_, c)| c == '\n')
					.skip(LINES);
		
				match newlines.next() {
					None => input,
					Some((i, _)) => {
						let rest = newlines.count();
						let mut sinput = input[..i].to_string();
						sinput += &format!("\n...<{} more lines>...", rest);
						sinput
					}
				}
			};

			panic!(
				"input #{} - '{}': expected `{}` got `{}` for input #{}",
				i,
				short_input.bold(),
				format!("{:?}", expected).green(),
				format!("{:?}", generated).red(),
				i,
			);
		};
	}
}
