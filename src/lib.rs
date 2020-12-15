
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
