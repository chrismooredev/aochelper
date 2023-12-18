use std::fmt;
use std::panic::UnwindSafe;
use std::str;

pub trait AoCDay: fmt::Debug + Sized + UnwindSafe + Clone + Copy {
	type Data<'i>: fmt::Debug;
	type Answer: fmt::Debug + fmt::Display + PartialEq + Eq;

	/// The day's number.
	fn day(&self) -> u8;

	/// Parses the raw input into a useable format. Input validation is recommended, and should be clonable for multiple uses.
	fn parse<'i>(&self, input: &'i str) -> Self::Data<'i>;

	/// Part 1 implementation.
	fn part1(&self, data: &mut Self::Data<'_>) -> Self::Answer;

	/// Part 2 implementation.
	///
	/// If the result of part1 is needed for part2, then it should be recomputed, or stored in the Data struct.
	fn part2(&self, data: &mut Self::Data<'_>) -> Self::Answer;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DayPart {
	Part1,
	Part2,
}
impl fmt::Display for DayPart {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			DayPart::Part1 => write!(f, "Part 1"),
			DayPart::Part2 => write!(f, "Part 2"),
		}
	}
}
