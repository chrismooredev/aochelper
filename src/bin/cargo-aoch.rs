use cargo_edit::{Dependency, RegistryReq};
use clap::Parser;
use std::process;
use std::{io, path::Path};
use toml_edit::Array;

const DAY_TEMPLATE_LIB: &'static str = include_str!("../../templates/lib.rs");
const DAY_TEMPLATE_BIN: &'static str = include_str!("../../templates/main.rs");

#[derive(Parser)]
#[clap(
	version = "0.1.1",
	author = "Chris M.",
	about = "Provides a Rust framework for organizing Advent of Code (AoC) challenges"
)]
struct Opts {
	#[clap(
		short = 'o',
		long = "omit-deps",
		about = "Omits common dependencies from Cargo.toml"
	)]
	omit_deps: bool,

	#[clap(
		short = 'i',
		long = "install-dep",
		about = "Inserts the dependency into Cargo.toml"
	)]
	install_dep: Vec<String>,

	#[clap(subcommand)]
	subcmd: SubCmd,
}

#[derive(Parser)]
enum SubCmd {
	/// cargo aoch new <day num> <day name>
	New(CmdNew),
}

#[derive(Parser)]
struct CmdInit {
	day_name: String,
}

#[derive(Parser)]
struct CmdNew {
	#[clap(about = "The day's number. Should be within the range of [1, 25]")]
	day_num: u8,
	#[clap(about = "The textual name/theme of the day")]
	day_name: String,
}

/// Returns a Dependency representing this crate
fn this_crate() -> Dependency {
	Dependency::new("aoch").set_git("https://github.com/csm123199/aochelper", None)
}

fn main() -> io::Result<()> {
	let opts: Opts = Opts::parse();

	let (day_num, day_name): (u8, String) = match opts.subcmd {
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

	if !cinit.status.success() {
		eprintln!("`cargo init` has failed. Exiting early.");
	}

	// CARGO PROJECT INITIALIZED - EDIT Cargo.toml, INSERT TEMPLATE

	// append args to end of Cargo.toml

	let mut vec: Vec<Dependency> = Vec::new();
	if !opts.omit_deps {
		// TODO: look for custom Cargo.toml workspace option for common deps
		vec.push(Dependency::new("itertools"));
	}
	for dep in &opts.install_dep {
		vec.push(Dependency::new(&dep));
	}

	// use cargo_edit to get dependency versions
	vec.iter_mut()
		.try_for_each(|dep| -> cargo_edit::Result<()> {
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
		let mut manifest = cargo_edit::Manifest::open(&Some("./Cargo.toml".into()))
			.expect("Unable to open Cargo.toml");

		manifest.add_deps(&["dependencies".into()], &vec).unwrap();

		manifest
			.write_to_file(
				&mut std::fs::File::create("Cargo.toml").expect("Unable to open Cargo.toml"),
			)
			.expect("Error writing Cargo.toml");
	}

	// replace stuff on our template and write it out
	let day_rs_bin = DAY_TEMPLATE_BIN
		.clone()
		.replace("{{DayNum}}", &format!("{:0>2}", day_num))
		.replace("{{DayName}}", &day_name);
	std::fs::write("src/main.rs", day_rs_bin)?;

	let day_rs_lib = DAY_TEMPLATE_LIB
		.clone()
		.replace("{{DayNum}}", &format!("{:0>2}", day_num))
		.replace("{{DayName}}", &day_name);
	std::fs::write("src/lib.rs", day_rs_lib)?;

	// go back to the directory above this new crate
	std::env::set_current_dir(std::env::current_dir()?.parent().unwrap())?;
	add_day_to_workspace_toml(day_num);

	Ok(())
}

fn add_day_to_workspace_toml(day_num: u8) {
	if let Ok(workspace_toml) = std::fs::read_to_string("Cargo.toml") {
		use toml_edit::{Document, Item, Table, Value};
		match workspace_toml.parse::<Document>() {
			Err(_) => eprintln!("Error parsing workspace Cargo.toml"),
			Ok(mut toml_doc) => {
				let doc = toml_doc.as_table_mut();

				if !doc.entry("package").is_none() {
					eprintln!("Not adding new package to current Cargo.toml - this directory's Cargo.toml looks like a crate instead of a workspace.");
					return;
				}

				let wkspc = doc.entry("workspace").or_insert(Item::Table(Table::new()));
				if let Item::Table(wkspc) = wkspc {
					let members = wkspc
						.entry("members")
						.or_insert(Item::Value(Value::Array(Array::default())));
					if let Item::Value(Value::Array(members)) = members {
						if let Err(_) = members.push(format!("day{:0>2}", day_num)) {
							eprintln!("workspace::members array does not contain strings");
						}
					} else {
						eprintln!("workspace::members item in workspace toml is not an array");
					}
				} else {
					eprintln!("workspace item in workspace toml is not a table");
				}

				if let Err(_) = std::fs::write("Cargo.toml", toml_doc.to_string_in_original_order())
				{
					eprintln!("Error writing workspace Cargo.toml");
				}
			}
		}
	} else if day_num == 1 {
		// make it
		std::fs::write("Cargo.toml", "").expect("unable to create a Cargo.toml");
		add_day_to_workspace_toml(day_num);
	}
}
