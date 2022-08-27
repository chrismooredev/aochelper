use clap::Parser;

/// cargo-aoch2 init [year=current]
///     sets up project from pre-existing Cargo.toml file
///     inject aoch dependency
///     add package.metadata.aoch.year key
/// cargo-aoch2 new [day=current]
///     if no day number used, current day/year is attempted
///     creates a new src/dayXX.rs file
///     downloads day's input file
/// cargo-aoch2 download
///     downloads all* available input files for the given session token
///     *skips days that are not implemented

#[derive(Parser)]
#[clap(
	version = clap::crate_version!(),
	author = "Chris Moore",
	about = "Easily create Rust templates for Advent of Code day implementations"
)]
struct Opts {

    /// The session token used for downloading inputs. If not provided, a session.txt file will be used.
    session: Option<String>
}

fn main() {
    // create src/dayXX.rs
    // download input (session.txt and advent year stored in Cargo.toml)
}
