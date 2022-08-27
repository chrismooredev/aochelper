use std::borrow::Cow;
use std::ffi::OsString;

pub mod tree_node; // independent helper data structure

pub mod aocday;
//#[macro_use]
//pub mod error_handling;
pub mod testing;

pub use aocday::{AoCDay, DayPart};
pub use testing::{run_test, test_runner};

pub use aoch_proc::{aoc_inputs, load_days};
pub mod macros {
	#[macro_export]
	macro_rules! daystr {
		($dnum: expr) => {
			include_str!(concat!("../../input/", $dnum, ".txt"))
		};
	}

	#[macro_export]
	macro_rules! aoc_input {
		($daynum: literal) => {
			aoc_input!("input", $daynum)
		};
		($inputdir: literal, $daynum: literal) => {
			include_str!(concat!("../../", $inputdir, "/", $daynum, ".txt"))
			// include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $inputdir, "/", $daynum, ".txt"))
		};
	}
}
pub mod parsing;

pub fn run_day<D: AoCDay>(day: D, inputstr: &str, part: Option<DayPart>) {
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

	run_day_with_input(day, part, &inp);
}

pub fn run_day_with_input<D: AoCDay>(day: D, part: Option<DayPart>, inputstr: &str) {
	let mut data = day.parse(&inputstr);
	if matches!(part, None | Some(DayPart::Part1)) {
		let p1_out = day.part1(&mut data);
		println!("Day {} Part 1: {}", day.day(), p1_out);
	}
	if matches!(part, None | Some(DayPart::Part2)) {
		let p1_out = day.part2(&mut data);
		println!("Day {} Part 2: {}", day.day(), p1_out);
	}
}
