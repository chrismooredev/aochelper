
use std::{path::Path, io};
use std::process;
use std::borrow::Cow;
use clap::Clap;
use cargo_edit::{Dependency, RegistryReq};

const DAY_TEMPLATE_LIB: &'static str = include_str!("../../templates/lib.rs");
const DAY_TEMPLATE_BIN: &'static str = include_str!("../../templates/main.rs");

#[derive(Clap)]
#[clap(version = "0.1.1", author = "Chris M.", about = "Provides a Rust framework for organizing Advent of Code (AoC) challenges")]
struct Opts {
	#[clap(short = 'o', long = "omit-deps", about = "Omits common dependencies from Cargo.toml")]
	omit_deps: bool,

	#[clap(short = 'i', long = "install-dep", about = "Inserts the dependency into Cargo.toml")]
	install_dep: Vec<String>,

	#[clap(subcommand)]
	subcmd: SubCmd,
}

#[derive(Clap)]
enum SubCmd {
	/// cargo aoch init <day name>
	Init(CmdInit),
	/// cargo aoch new <day num> <day name>
	New(CmdNew),
}

#[derive(Clap)]
struct CmdInit {
	day_name: String,
}

#[derive(Clap)]
struct CmdNew {
	#[clap(about = "The day's number. Should be within the range of [1, 25]")]
	day_num: u8,
	#[clap(about = "The textual name/theme of the day")]
	day_name: String,
}

/// Returns a Dependency representing this crate
fn this_crate() -> Dependency {
	Dependency::new("aoch")
		.set_git("https://github.com/csm123199/aochelper", None)
}

fn main() -> io::Result<()> {
	let opts: Opts = Opts::parse();

	let (day_num, day_name): (u8, String) = match opts.subcmd {
		SubCmd::Init(CmdInit { day_name }) => {
			// get day num from folder name

			let wholepath = std::env::current_dir()?;
			let dirname = wholepath
				.components().last()
				.expect("unable to get current process directory");

			let dirname_str: Cow<'_, str> = dirname.as_os_str().to_string_lossy();

			let nums: String = dirname_str.chars()
				.map(|c| if !c.is_numeric() { ' ' } else { c })
				.collect();

			let trimmed = nums.trim();

			// parse will error if it cannot consume the entire string
			// eg) string has whitespace between numbers
			match trimmed.parse() {
				Ok(n) => (n, day_name),
				Err(_) => {
					eprintln!("Unable to retrieve day number from folder name `{}`. Exiting.", nums);
					eprintln!("\t(are there non-contiguous numbers?)");
					return Ok(());
				}
			}
		},
		SubCmd::New(CmdNew { day_num, day_name }) => {
			// make the folder - cd into it
			let folder_name = format!("day{:0>2}", day_num);
			if let Err(e) = std::fs::create_dir(&folder_name) {
				eprintln!("Unable to create folder `{}`. Exiting.", folder_name);
				eprintln!("{}", e.to_string());
				return Ok(());
			}
			std::env::set_current_dir(&folder_name)?;

			(day_num, day_name)
		}
	};

	// NOW INSIDE DAY'S FOLDER

	let cinit = process::Command::new("cargo")
		.args(&["init"])
		.output()
		.expect("failed to execute process. Cargo not on path?");
	
	if ! cinit.status.success() {
		eprintln!("`cargo init` has failed. Exiting early.");
	}

	// CARGO PROJECT INITIALIZED - EDIT Cargo.toml, INSERT TEMPLATE

	// append args to end of Cargo.toml

	let mut vec: Vec<Dependency> = Vec::new();
	if ! opts.omit_deps {
		// TODO: look for custom Cargo.toml workspace option for common deps
		vec.push(Dependency::new("itertools"));
	}
	for dep in &opts.install_dep {
		vec.push(Dependency::new(&dep));
	}

	// use cargo_edit to get dependency versions
	vec.iter_mut()
		.try_for_each(|dep| -> cargo_edit::Result<()>{
			// use registries as specified at/above the destination crate's location
			*dep = cargo_edit::get_latest_dependency(
				&dep.name,
				false,
				RegistryReq::project(None, Path::new(".")),
			)?;
			Ok(())
		})
		.expect("Unable to determine latest version of a crate");

	// add ourselves - not yet on crates.io so will fail above
	vec.push(this_crate());

	{
		let mut manifest = cargo_edit::Manifest::open(&Some("./Cargo.toml".into())).expect("Unable to open Cargo.toml");

		manifest.add_deps(&["dependencies".into()], &vec).unwrap();

		manifest.write_to_file(&mut std::fs::File::create("Cargo.toml").expect("Unable to open Cargo.toml")).expect("Error writing Cargo.toml");
	}
	
	// replace stuff on our template and write it out
	let day_rs_bin = DAY_TEMPLATE_BIN.clone()
		.replace("{{DayNum}}", &format!("{:0>2}", day_num))
		.replace("{{DayName}}", &day_name);
	std::fs::write("src/main.rs", day_rs_bin)?;

	let day_rs_lib = DAY_TEMPLATE_LIB.clone()
		.replace("{{DayNum}}", &format!("{:0>2}", day_num))
		.replace("{{DayName}}", &day_name);
	std::fs::write("src/lib.rs", day_rs_lib)?;

	Ok(())
}
