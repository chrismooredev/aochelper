
use std::fmt;
use std::panic;

#[allow(unused_imports)]
use colored::Colorize;

use crate::{AoCDay, DayPart};

//TODO: See about consolidating the logic of these two functions

// Used to test a day's specific part
// Note: should refactor to use run_test ?
pub fn test_runner<D: AoCDay, TS: ToString + fmt::Debug + PartialEq<D::Answer> + Eq>(part: DayPart, cases: &[(&str, TS)]) {
	for (input, ans) in cases {
		match &mut D::parse(input) {
			Ok(day_struct) => {
				// Catch any panics each part may throw
				// TODO: implement chain_err or something
				let result: D::Answer = match part {
					DayPart::Part1 => day_struct.part1().expect(&format!("Error executing Day {}, Part {}", D::day(), part)),
					DayPart::Part2 => day_struct.part2().expect(&format!("Error executing Day {}, Part {}", D::day(), part)),
				};

				let calcd = result;
				if ans.to_string() != calcd.to_string() {
					// limit input string to 10 lines
					let short_input: String = {
						const LINES: usize = 10;
						let lines: Vec<&str> = input.lines().collect();
						if lines.len() < LINES {
							input.to_string()
						} else {
							let lines_first_few = lines.split_at(LINES).0;
							let mut s = String::new();
							for line in lines_first_few {
								if s.len() > 0 { s += "\n"; }
								s += line;
							}
							if lines_first_few.len() == LINES {
								s += "\n...";
							}
							s
						}
					};

					panic!("input '{}': expected `{}` got `{}`",
						short_input.bold(),
						format!("{}", ans.to_string().green()),
						format!("{}", calcd.to_string().red())
					);
				} else {
					//println!("yay")
				};
			},
			Err(e) => panic!("parsing input `{}` produces error `{}`", input.bold(), format!("{}", e).red()),
		}
	}
	// No return value - implicit success if here
}

// Used to test a specific function in a day
pub fn run_test<I, O, F>(func: F, cases: &[(I, O)]) where
	I: fmt::Debug + Sized,
	O: PartialEq<O> + Eq + fmt::Debug,
	F: for<'a> Fn(&'a I) -> O {
	for (case, expected) in cases {
		let generated = func(case);
		if *expected != generated {
			panic!("for input '{}': expected `{}` got `{}`",
				format!("{:?}", case).bold(),
				format!("{:?}", expected).green(),
				format!("{:?}", generated).red()
			);
		};
	};
}

