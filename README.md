
# Advent of Code Helper Library & Cargo Subcommand
This library is meant to assist in the structuring, running, and creation of tests for Advent of Code challenges.

## Usage
Install the command/library globally:
```
cargo install aoch
```
Then in a folder containing your AoC days, you can run the following to init the project:
```
cargo-aoch new <day_num> <day_name>
```
(Alternatively run `cargo-aoch init <day_name>` in an already existing folder)

## Why this one? (aka: notable features)
(over the many other AoC crates)
* Simple day creation
  * Clear seperation of parse/part1/part2 phases
* Mutable day struct
  * Can easily store any data within a custom struct
  * Day structs are managed via an `AoCDay` trait, which has been stubbed out for you.
  * Easy caching between parts 1 and 2 when run as a binary
* Provides a basic N-Tree implementation with [`ptree`](https://crates.io/crates/ptree)
* Development is test-oriented, while exposing your impl to a day binary
  * Run the program with `cargo run --release` for a fast run, once the algorithm is stabilized.
* Easily organizes your days within a cargo workspace
* Inclusion of typical helper libraries (data structures, itertools, etc)

This crate allows you to easily implement your solutions with the right amount of skeleton code to focus on just your projec

## Day Input
In the same folder, save your personal puzzle input to `input/00.txt`, replacing `00` with your 2-digit day number. This will be passed to your puzzle as a `&str` for further parsing.

## Running your Day Inputs
Day problems are implemented as tests, so you can run the regular test command to check your programs. (`cargo test`) Tests are auto-populated with your puzzle input, checking it as equal to zero - this can be changed once the correct answer is found.

Running the crate as a binary runs the program against the large puzzle input - this allows easy usage of release compilation, after the algorithm is solidified.

Day outputs can be any type implementing `Eq` - meaning puzzle outputs can be nearly any type (`usize`, `String`, a custom type, etc)

## Contributing

Any improvements are welcome as GitHub Pull Requests.
## Current Todos
* Get command to run properly as "`cargo aoch ...`" (Currently my [`clap`](https://crates.io/crates/clap) code isn't configured for this)
* Cleanup test runner code?
* Be able to display multi-line inputs/outlines properly (use different colors on dedicated lines?)
* Create subcommand (`update`?) to auto-create days up-to current day
  * w/ auto-downloading inputs?
