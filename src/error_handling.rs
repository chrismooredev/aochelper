use std::borrow::Cow;
use std::error;
use std::fmt;
use std::io;
use std::num::{ParseFloatError, ParseIntError};

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

macro_rules! impl_from_error {
	($mem: ident, $t: ty) => {
		impl From<$t> for DayError {
			fn from(e: $t) -> Self {
				DayError::$mem(e.into())
			}
		}
	};
}

// be sure to update the error::Error impl for new variants, if applicable (wrapped data is also an error::Error)
#[derive(Debug)]
pub enum DayError {
	/// A day, or part of a day, is not yet implemented.
	Unimplemented,
	IOError(io::Error),
	ParseInt(ParseIntError),
	ParseFloat(ParseFloatError),
	Wrapped(Box<dyn error::Error>),
	Generic(Cow<'static, str>),
}
impl DayError {
	pub fn generic<S: Into<Cow<'static, str>>>(msg: S) -> DayError {
		DayError::Generic(msg.into())
	}
	pub fn from_debug<E: fmt::Debug>(e: E) -> DayError {
		DayError::Generic(format!("{:?}", e).into())
	}
}

impl_from_error!(IOError, io::Error);
impl_from_error!(ParseInt, ParseIntError);
impl_from_error!(ParseFloat, ParseFloatError);
impl_from_error!(Wrapped, Box<dyn error::Error>);

impl_from_error!(Generic, Cow<'static, str>);
impl_from_error!(Generic, &'static str);
impl_from_error!(Generic, String);

// Forward Display impl to Debug impl
impl fmt::Display for DayError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use DayError::*;
		match self {
			Generic(s) => fmt::Display::fmt(s, f),
			_ => fmt::Debug::fmt(self, f),
		}
	}
}
impl error::Error for DayError {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		use DayError::*;
		match &self {
			IOError(e) => Some(e),
			ParseInt(e) => Some(e),
			ParseFloat(e) => Some(e),
			Wrapped(e) => Some(e.as_ref()),
			_ => None,
		}
	}
}
