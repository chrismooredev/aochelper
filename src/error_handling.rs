use std::borrow::Cow;
use std::error;
use std::fmt;
use std::io;

// https://stackoverflow.com/a/31749071 - Macros within crates
// These are inspired by various third-party crate libs

// be sure to update the error::Error impl for new variants, if applicable (wrapped data is also an error::Error)
#[derive(Debug, thiserror::Error)]
pub enum DayError {
	IOError(#[from] io::Error),
	Wrapped(#[from] Box<dyn error::Error>),
	Generic(Cow<'static, str>),
}
impl DayError {
	pub fn generic<S: Into<Cow<'static, str>>>(msg: S) -> DayError {
		DayError::Generic(msg.into())
	}
	pub fn from_debug<E: fmt::Debug>(e: E) -> DayError {
		DayError::Generic(format!("{:?}", e).into())
	}
	pub fn boxed<E: 'static + error::Error>(e: E) -> DayError {
		DayError::Wrapped(Box::new(e))
	}
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