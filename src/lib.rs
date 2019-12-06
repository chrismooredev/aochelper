
#![feature(termination_trait_lib)]
#![feature(associated_type_defaults)]

extern crate colored;
extern crate atty;

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

pub fn run_day<D: aocday::AoCDay>(inputstr: &'static str) {
	let dynread: Box<dyn std::io::Read> = if atty::is(atty::Stream::Stdin) {
		Box::new(inputstr.as_bytes())
	} else {
		eprintln!("Note: Using stdin for puzzle input, since it's not a TTY");
		let stdin: std::io::Stdin = std::io::stdin();
		Box::new(stdin)
	};
	let mut parsed = D::parse(dynread).unwrap();
	println!("Part 1: {}", parsed.part1());
	println!("Part 2: {}", parsed.part2());
}