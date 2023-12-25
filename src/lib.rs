#![feature(min_specialization)]

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
		};
	}
}
pub mod parsing;

#[derive(clap::Parser)]
struct DayRunCmd {
	/// The day's input provided as a filename, or '-' for stdin.
	#[arg(short, long)]
	input_file: Option<OsString>,

	#[arg(short, long, default_value_t = false)]
	quiet: bool,

	#[arg(short, long, default_value_t = 1)]
	repeat: usize,

	#[arg(short, long, default_value_t = false)]
	parse_per_run: bool,

	#[arg(short, long, default_value_t = false)]
	one: bool,

	#[arg(short, long, default_value_t = false)]
	two: bool,
}

pub fn run_day<D: AoCDay>(day: D, inputstr: &str, part: Option<DayPart>) {
	use std::io::Read;
	use clap::Parser;

	let args = DayRunCmd::parse();

	let inp: Cow<'_, str> = match args.input_file {
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
		},
	};

	let mut parsed_input = (!args.parse_per_run).then(|| day.parse(&inp));

	for _ in 0..args.repeat {
		if args.parse_per_run {
			parsed_input = Some(day.parse(&inp));
		}
		let mut data = parsed_input.as_mut().unwrap();
		let exclusive = args.one ^ args.two;
		let one = !exclusive || args.one || part == Some(DayPart::Part1);
		let two = !exclusive || args.two || part == Some(DayPart::Part2);
		if one {
			let r = day.part1(&mut data);
			if ! args.quiet {
				println!("Day {} Part 1: {}", day.day(), r);
			}
		}
		if two {
			let r = day.part2(&mut data);
			if ! args.quiet {
				println!("Day {} Part 2: {}", day.day(), r);
			}
		}
	}

	// run_day_with_input(day, part, &inp, false);
}

pub fn run_day_with_input<D: AoCDay>(day: D, part: Option<DayPart>, inputstr: &str, quiet: bool) {
	let mut data = day.parse(inputstr);
	if matches!(part, None | Some(DayPart::Part1)) {
		let p1_out = day.part1(&mut data);
		if ! quiet {
			println!("Day {} Part 1: {}", day.day(), p1_out);
		}
	}
	if matches!(part, None | Some(DayPart::Part2)) {
		let p1_out = day.part2(&mut data);
		if ! quiet {
			println!("Day {} Part 2: {}", day.day(), p1_out);
		}
	}
}
