use cargo_edit::{Dependency, RegistryReq};
// use cargo_edit::{Dependency, RegistryReq};
use chrono::Datelike;
use clap::Parser;
use reqwest::Url;
use std::ffi::OsString;
use std::process;
use std::sync::Arc;
use std::{io, path::Path};
use toml_edit::{Array, Entry};

const DAY_TEMPLATE_LIB: &'static str = include_str!("../../templates/lib.rs");
const DAY_TEMPLATE_BIN: &'static str = include_str!("../../templates/main.rs");
const GITIGNORE: &'static str = include_str!("../../templates/gitignore");

/// Provides a Rust framework for organizing Advent of Code (AoC challenges)
#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about=None)]
struct Opts {
	/// Omits common dependencies from Cargo.toml
	#[arg(short, long)]
	omit_deps: bool,

	/// Inserts the dependency into Cargo.toml
	#[arg(short, long)]
	install_dep: Vec<String>,

	#[clap(subcommand)]
	subcmd: SubCmd,
}

#[derive(Debug, clap::Parser)]
enum SubCmd {
	/// cargo aoch new <day num> <day name>
	New(CmdNew),
}

#[derive(clap::Parser)]
struct CmdInit {
}

#[derive(Debug, clap::Parser)]
struct CmdNew {
	/// The day's number. Should be within the range of [1, 25]
	day_num: Option<u8>,
}

/// Returns a Dependency representing this crate
fn this_crate() -> Dependency {
	Dependency::new("aoch").set_git("https://github.com/chrismooredev/aochelper", None)
}

fn download_input(year: i64, day: u8, session: &str) -> Result<(), Box<dyn std::error::Error>> {
	if let Err(e) = std::fs::create_dir("./input") {
		if e.kind() != std::io::ErrorKind::AlreadyExists {
			return Err(e.into());
		}
	}

	let jar = reqwest::cookie::Jar::default();
	let hostname = "adventofcode.com";
	let url = format!("https://{}", hostname).parse::<Url>().unwrap();
	jar.add_cookie_str(&format!("session={}; Domain={}", session, hostname), &url);

	let client = reqwest::blocking::Client::builder()
		.cookie_provider(Arc::new(jar))
		.build().expect("error occured while creating reqwest http client");

	let resp = client.get(format!("https://{}/{}/day/{}/input", hostname, year, day))
		.send()?
		.bytes()?;

	std::fs::write(format!("input/{:>02}.txt", day), resp)?;

	Ok(())
}

fn main() -> io::Result<()> {
	let mut args: Vec<OsString> = std::env::args_os().collect();

	// enable running as `cargo aoch ...` instead of `cargo-aoch ...`
	match args.get(1) {
		Some(sec) if sec == "aoch" => {
			args.remove(1);
		},
		_ => {},
	}

	let opts: Opts = Opts::parse_from(args);

	// can be switched to match if more subcommands are added
	let SubCmd::New(CmdNew { day_num }) = opts.subcmd;
	let day_num = day_num.unwrap_or(chrono::Utc::now().day() as u8);

	let mut year: Option<i64> = None;
	let session_cookie = add_day_to_workspace_toml(&mut year, day_num);
	let act_year = year.unwrap();

	// download the day's input
	if let Some(session) = session_cookie {
		if let Err(e) = download_input(act_year, day_num, &session) {
			eprintln!("error downloading input: {}", e);
		}
	} else {
		eprintln!("no session cookie found. not downloading input.");
	}

	// create and enter the day's specific folder
	let folder_name = format!("day{:0>2}", day_num);
	if let Err(e) = std::fs::create_dir(&folder_name) {
		eprintln!("Unable to create folder `{}`. Exiting.", folder_name);
		eprintln!("{}", e.to_string());
		return Ok(());
	}
	std::env::set_current_dir(&folder_name)?;

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
		vec.push(Dependency::new("thiserror"));
		vec.push(Dependency::new("log"));
		vec.push(Dependency::new("test-log"));
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
		.replace("{{DayNum}}", &format!("{}", day_num));
	std::fs::write("src/main.rs", day_rs_bin)?;

	let day_rs_lib = DAY_TEMPLATE_LIB
		.replace("{{DayNum}}", &format!("{:0>2}", day_num));
	std::fs::write("src/lib.rs", day_rs_lib)?;

	Ok(())
}

/// Adds the specified day to the workspace's Cargo.toml, and returns a session token string, if found
fn add_day_to_workspace_toml(year: &mut Option<i64>, day_num: u8) -> Option<String> {
	let mut session_cookie: Option<String> = None;

	if let Ok(workspace_toml) = std::fs::read_to_string("Cargo.toml") {
		use toml_edit::{Document, Item, Table, Value};
		match workspace_toml.parse::<Document>() {
			Err(_) => eprintln!("Error parsing workspace Cargo.toml"),
			Ok(mut toml_doc) => {
				let doc = toml_doc.as_table_mut();

				if matches!(doc.entry("package"), Entry::Occupied(_)) {
					eprintln!("Not adding new package to current Cargo.toml - this directory's Cargo.toml looks like a crate instead of a workspace.");
					return None;
				}

				let wkspc = doc.entry("workspace").or_insert(Item::Table(Table::new()));
				if let Item::Table(wkspc) = wkspc {
					let members = wkspc
						.entry("members")
						.or_insert(Item::Value(Value::Array(Array::default())));

					if let Item::Value(Value::Array(members)) = members {
						let daystr = format!("day{:0>2}", day_num);
						let included = members.iter()
							.any(|v| {
								let s = v.as_str()
									.expect("non-string item found in members array in workspace Cargo.toml");
								s == daystr
							});

						if ! included {
							members.push(daystr);
						}
					} else {
						eprintln!("[error] workspace::members item in workspace toml is not an array");
					}

					let meta = wkspc.entry("metadata").or_insert(Item::Table(Table::new()));
					if let Item::Table(meta) = meta {
						meta.set_implicit(true);
						let aoch = meta.entry("aoch").or_insert(Item::Table(Table::new()));
						if let Item::Table(aoch) = aoch {
							if let Some(Item::Value(Value::String(session))) = aoch.get("session") {
								session_cookie = Some(session.value().clone());
							}


							// no expected year, populate current year in toml
							// no expected year, already exists year in toml (return in &mut Option)
							// expected year, assert against year in toml
							// expected year, populate year in toml
							let yr = aoch.entry("year");
							let yr = yr.or_insert(Item::Value(year.unwrap_or_else(|| chrono::Utc::now().year() as i64).into()));

							// yr could include actual, expected, or saved
							// year is just expected
							if let Some(doc_yr) = yr.as_integer() {
								if let Some(exp_yr) = year {
									if doc_yr != *exp_yr {
										eprintln!("[error] workspace::metadata::aoch::year was expected to be {}, but was actually {}", exp_yr, doc_yr);
									}
								}

								// 'return' the document's year to the caller, overwriting expected
								let _ = year.insert(doc_yr);
							} else {
								eprintln!("[error] workspace::metadata::aoch::year was not an integer");
							}
						}
					}
				} else {
					eprintln!("[error] workspace item in workspace toml is not a table");
				}

				if let Err(_) = std::fs::write("Cargo.toml", toml_doc.to_string())
				{
					eprintln!("Error writing workspace Cargo.toml");
				}
			}
		}
	} else if day_num == 1 {
		// new repo - init git, etc
		std::fs::write("Cargo.toml", "").expect("unable to create a Cargo.toml");
		add_day_to_workspace_toml(year, day_num);

		match git2::Repository::init(".") {
			Ok(_) => {
				// add .gitignore
				if let Err(e) = std::fs::write(".gitignore", GITIGNORE) {
					eprintln!("[error] error writing workspace .gitignore: {}", e);
				}
			},
			Err(e) => eprintln!("[error] failed to init git repo for workspace: {}", e),
		}
	} else {
		eprintln!("No workspace found, but trying to initialize a non-first day. Exiting.");
	}

	session_cookie
}
