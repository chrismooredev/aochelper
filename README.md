
# Advent of Code Helper Library & Cargo Subcommand
This library is meant to assist in the structuring, running, and creation of tests for Advent of Code challenges.

## Usage
Install the command/library globally:
```
cargo install aochelper
```
Then in a folder containing your AoC days, you can run the following to init the project:
```
cargo-aoc new <day_num> <day_name>
```
(Alternatively run `cargo-aoc init <day_name>` in an already existing folder)

## Why this one?
(over the many other AoC crates)
* Simple day creation
* Mutable day struct
  * Easy caching between parts 1 and 2 when run as a binary
* Provides a basic N-Tree implementation with [`ptree`](https://crates.io/crates/ptree)
* Runs the program as proper tests
  * Run the program with `cargo run --release` for a fast run, once the algorithm is stabilized.
* Inclusion of typical helper libraries (data structures, itertools, etc)

This crate allows you to easily implement your solutions with the right amount of skeleton code to focus on just your projec

## Day Input
In the same folder, save your personal puzzle input to `input/00.txt`, replacing `00` with your 2-digit day number. This will be passed to your puzzle as a `&str` for further parsing.

## Running your Day Inputs
Day problems are implemented as tests, so you can run the regular test command to check your programs. (`cargo test`) Tests are auto-populated with your puzzle input, checking it as equal to zero - this can be changed once the correct answer is found.

Running the crate as a binary runs the program against the large puzzle input - this allows easy usage of release compilation, after the algorithm is solidified.

Input `&str`'s can be compared to any data that implements `Eq` - meaning puzzle output can be `String`s or even a custom data type.

## Contributing

Any improvements are welcome as GitHub Pull Requests.
## Current Todos
* Get command to run properly as "`cargo aoc ...`" (Currently [`clap`](https://crates.io/crates/clap) isn't configured for this)
* Cleanup test runner code?
