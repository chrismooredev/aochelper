use std::borrow::Cow;

pub mod tree_node; // independent helper data structure

pub mod aocday;
#[macro_use]
pub mod error_handling;
pub mod testing;

pub use error_handling::DayError;
pub use aocday::{AoCDay, DayResult, DayPart};
pub use testing::{run_test, test_runner};

pub mod macros {
	#[macro_export]
	macro_rules! daystr {
		($dnum: expr) => { include_str!(concat!("../../input/", $dnum, ".txt")) }
	}
}
pub mod parsing {
	use std::str::FromStr;

	/// Trims the string, and returns it if the length is greater than zero.
	pub fn trimmed<'a>(s: &'a str) -> Option<&'a str> {
		let trimmed = s.trim();
		if trimmed.len() > 0 {
			Some(trimmed)
		} else {
			None
		}
	}

	/// Returns a vector of the specified type, parsed from strings using [`std::str::parse`].
	///
	/// Trims whitespace and skips empty strings
	pub fn from_iter<'a, T: FromStr, I: Iterator<Item = &'a str>>(iter: I) -> Result<Vec<T>, T::Err> {
		iter
			.filter_map(trimmed)
			.map(str::parse)
			.collect::<Result<Vec<T>, T::Err>>()
	}

	/// Returns a vector of the specified type, parsed from lines using [`std::str::parse`].
	///
	/// Trims whitespace and skips empty lines
	pub fn from_lines<T: FromStr>(input: &str) -> Result<Vec<T>, T::Err> {
		input.lines()
			.filter_map(trimmed)
			.map(str::parse)
			.collect::<Result<Vec<T>, T::Err>>()
	}

	pub fn from_grouped_lines<T: FromStr>(input: &str) -> Result<Vec<Vec<T>>, T::Err> {
		input.split_terminator("\n\n")
			.filter_map(trimmed)
			.map(|s| {
				s.split_terminator("\n")
					.filter_map(trimmed)
					.map(str::parse)
					.collect::<Result<Vec<T>, T::Err>>()
			})
			.collect::<Result<Vec<Vec<T>>, T::Err>>()
	}
}

pub fn run_day<D>(inputstr: &str) where D: AoCDay {
	let inp: Cow<'_, str> = if atty::is(atty::Stream::Stdin) {
		Cow::Borrowed(inputstr)
	} else {
		use std::io::Read;

		eprintln!("Note: Using stdin for puzzle input, since it's not a TTY");
		let stdin: std::io::Stdin = std::io::stdin();
		let mut input = String::new();
		stdin.lock().read_to_string(&mut input).expect("io error reading stdin");
		
		Cow::Owned(input)
	};
	
		let mut parsed = D::parse(inp.as_ref()).unwrap();
		println!("Part 1: {:?}", parsed.part1());
		println!("Part 2: {:?}", parsed.part2());
}
