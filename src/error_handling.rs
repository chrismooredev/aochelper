
use std::io;
use std::fmt;
use std::error;

// https://stackoverflow.com/a/31749071 - Macros within crates
// These are inspired by various third-party crate libs


/// Exits a function early with an `Error`.
///
/// The `bail!` macro provides an easy way to exit a function. `bail!(X)` is
/// equivalent to writing:
///
/// ```rust,ignore
/// return Err(format_err!(X))
/// ```
#[macro_export]
macro_rules! bail {
	($e:expr) => {
		return Err(crate::aocday::DayError::new($e));
	};
	($fmt:expr, $($arg:tt)*) => {
		return Err(crate::aocday::DayError::new(format!($fmt, $($arg)*)));
	};
}

/// Exits a function early with an `Error` if the condition is not satisfied.
///
/// Similar to `assert!`, `ensure!` takes a condition and exits the function
/// if the condition fails. Unlike `assert!`, `ensure!` returns an `Error`,
/// it does not panic.
#[macro_export]
macro_rules! ensure {
	($cond:expr) => {
		if !($cond) {
			bail!("{}", stringify!($cond));
		}
	};
	($cond:expr, $e:expr) => {
		if !($cond) {
			bail!($e);
		}
	};
	($cond:expr, $fmt:expr, $($arg:tt)*) => {
		if !($cond) {
			bail!($fmt, $($arg)*);
		}
	};
}

#[derive(Debug)]
pub enum DayError {
	IOError(io::Error),
	Wrapped(Box<dyn error::Error>),
	ParsingError(String),
	Simple(String),
	Numbered(i32),
}
impl DayError {
	pub fn new<S: Into<String>>(msg: S) -> DayError {
		DayError::Simple(msg.into())
	}
}
impl From<io::Error> for DayError {
	fn from(e: io::Error) -> Self {
		DayError::IOError(e)
	}
}
impl From<std::num::ParseIntError> for DayError {
	fn from(e: std::num::ParseIntError) -> Self {
		DayError::ParsingError(format!("{}", e))
	}
}
impl From<std::num::ParseFloatError> for DayError {
	fn from(e: std::num::ParseFloatError) -> Self {
		DayError::ParsingError(format!("{}", e))
	}
}
impl From<Box<dyn error::Error>> for DayError {
	fn from(e: Box<dyn error::Error>) -> Self {
		DayError::Wrapped(e)
	}
}
impl From<&str> for DayError {
	fn from(e: &str) -> Self {
		DayError::Simple(e.to_string())
	}
}
impl fmt::Display for DayError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self)
	}
}
impl error::Error for DayError {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		match &self {
			DayError::IOError(e) => Some(e),
			DayError::Wrapped(e) => Some(e.as_ref()),
			_ => None,
		}
	}
}
