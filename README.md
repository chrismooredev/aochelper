
# Advent of Code Helper Library
This library is meant to assist in the structuring, running, and creation of tests for Advent of Code challenges.

## Usage
Create a new crate, and add this one as a dependency. I recommend copying the `./day_template.rs`, `./create-template.sh`, `./test` (for bash) and `./test.ps1` (for powershell) files.

I recommend each day as a seperate binary (ex: each day in `./src/bin/dayXX.rs`) with their own tests.

## Utility Files
If you run `./create-template.sh <day_number> <day_name>` it will automatically copy the template to `./src/bin/dayXX.rs` and fill the day number and name into the file so you can get going!

If you are going to use the template file, it is highly recommended to put each day's input data in `./input/XX.txt`. This will be fed to each day so you can parse it as a `&str`.

The day template file has some tests in it already, so you should just have to adapt them for each day. Once you finish a part of a day, you can replace the `0` in the test so it passes. Most days provide shorter inputs and expected answers to add to this section as well.

## Day Binaries
If you use the day template, then you will get a `fn main() { ... }` for each day. If this is ran from a TTY, it will automatically use the `input/XX.txt` file built into the binary at compilation, otherwise it will obtain the day input via stdin.

You can test each one seperately using `cargo test --bin dayXX`, or just `./test XX`, which is an alias for that (which also means you can use `./test XX $test_name` for individual tests, just like with Cargo)

## Testing
The day template shows how I've been testing my programs. I've tried to work the test functions to allow a good number of type inputs/outputs as Rust's type system allows. If you have any improvements, I welcome you to submit PRs for review.
