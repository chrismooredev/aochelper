use std::str::FromStr;
use std::fmt::Debug;

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
pub fn try_from_lines_with<T, E: Debug, F: FnMut(&str) -> Result<T, E>>(input: &str, mut map: F) -> Vec<T> {
    input
        .lines()
        .enumerate()
        .filter_map(|(i, itm)| trimmed(itm).map(|itm| (i, itm)))
        .map(|(i, itm)| map(itm).expect(&format!("error parsing input line {}", i)))
        .collect::<Vec<T>>()
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
