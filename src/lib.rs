use std::borrow::Cow;
use std::ffi::OsString;

pub mod tree_node; // independent helper data structure

pub mod aocday;
#[macro_use]
pub mod error_handling;
pub mod testing;

pub use aocday::{AoCDay, DayPart, DayResult};
pub use error_handling::DayError;
pub use testing::{run_test, test_runner};

pub mod macros {
	#[macro_export]
	macro_rules! daystr {
		($dnum: expr) => {
			include_str!(concat!("../../input/", $dnum, ".txt"))
		};
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

	/// Takes each line with content, and passes them pre-trimmed to the mapping function
	pub fn try_from_lines_with<T, E, F: FnMut(&str) -> Result<T, E>>(input: &str, map: F) -> Result<Vec<T>, E> {
		input
			.lines()
			.filter_map(trimmed)
			.map(map)
			.collect::<Result<Vec<T>, E>>()
	}

	/// Takes each line with content, and passes them pre-trimmed to the mapping function
	pub fn from_lines_with<T, F: FnMut(&str) -> T>(input: &str, map: F) -> Vec<T> {
		input
			.lines()
			.filter_map(trimmed)
			.map(map)
			.collect::<Vec<T>>()
	}


	/// Returns a vector of the specified type, parsed from strings using [`std::str::parse`].
	///
	/// Trims whitespace and skips empty strings
	pub fn from_iter<'a, T: FromStr, I: Iterator<Item = &'a str>>(
		iter: I,
	) -> Result<Vec<T>, T::Err> {
		iter.filter_map(trimmed)
			.map(str::parse)
			.collect::<Result<Vec<T>, T::Err>>()
	}

	/// Returns a vector of the specified type, parsed from lines using [`std::str::parse`].
	///
	/// Trims whitespace and skips empty lines
	pub fn from_lines<T: FromStr>(input: &str) -> Result<Vec<T>, T::Err> {
		input
			.lines()
			.filter_map(trimmed)
			.map(str::parse)
			.collect::<Result<Vec<T>, T::Err>>()
	}

	pub fn from_grouped_lines<T: FromStr>(input: &str) -> Result<Vec<Vec<T>>, T::Err> {
		input
			.split_terminator("\n\n")
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

pub fn run_day<D: AoCDay>(inputstr: &str) {
	use std::io::Read;

	let args: Vec<OsString> = std::env::args_os().collect();
	if args.len() > 2 {
		eprintln!("Usage: {:?} [input file]", args[0]);
		eprintln!("If no input file is provided, then an embedded input will be used instead.");
		eprintln!("If input is '-' then input is read from stdin.");
		return;
	}

	let inp: Cow<'_, str> = match args.get(1) {
		None => Cow::Borrowed(inputstr),
		Some(s) if s == "-" => {
			let mut input = String::new();
			let stdin: std::io::Stdin = std::io::stdin();
		
			stdin
				.lock()
				.read_to_string(&mut input)
				.expect("io error reading stdin");

			Cow::Owned(input)
		},
		Some(s) => {
			Cow::Owned(std::fs::read_to_string(s).expect("io error reading from file"))
		}
	};

	let mut parsed = D::parse(inp.as_ref()).unwrap();
	println!("Part 1: {:?}", parsed.part1());
	println!("Part 2: {:?}", parsed.part2());
}
