
use std::ffi::OsStr;
use std::path::Path;
use std::io;
use syn::{parse_macro_input, LitStr};
use quote::{quote, format_ident};
use proc_macro2::TokenStream as PM2TokenStream;

extern crate proc_macro;
use proc_macro::TokenStream as PMTokenStream;

#[proc_macro]
pub fn aoc_inputs(name: PMTokenStream) -> PMTokenStream {
	// load all files in "inputs/XX.txt"
	// emit as string array

	let mani_dir = std::env::var("CARGO_MANIFEST_DIR")
		.expect("CARGO_MANIFEST_DIR environment variable not defined");

	let input: Option<LitStr> = parse_macro_input!(name as Option<LitStr>);
	let input_dir = input.map(|s| s.value()).unwrap_or_else(|| "input".to_string());

	let input_dir = Path::new(&mani_dir).join(input_dir);

	let inputs = get_directory_contents(&input_dir).expect(&format!("unable to read day input file from {:?}", input_dir));

	// emit [Option<&'static str>; 25]

	let elements: PM2TokenStream = inputs.into_iter()
		.map(|opt| match opt {
			None => quote! { None, },
			Some(s) => quote! { Some(#s), }
		})
		.collect();
	
	quote! { [ #elements ] }.into()
}

fn get_directory_contents(input_dir: &Path) -> io::Result<[Option<String>; 25]> {	
	let mut found: [Option<String>; 25] = vec![None; 25].try_into().unwrap();

	for ent in input_dir.read_dir()? {
		let dirent = ent?;
		let name = dirent.file_name();
		let name = Path::new(&name);

		if name.extension() != Some(OsStr::new("txt")) {
			continue;
		}

		if let Some(fs) = name.file_stem().and_then(OsStr::to_str) {
			if let Ok(day) = fs.parse::<u8>() {
				let day_ind = if 1 <= day && day <= 25 {
					(day as usize) - 1
				} else {
					panic!("input directory contains numbered day file that is out of range (exected range [1, 25]): {:?}", name);
				};
				if found[day_ind].is_some() {
					panic!("input directory contains multiple text files for day {}", day);
				}
				
				let contents = std::fs::read_to_string(dirent.path())?;
				
				found[day_ind] = Some(contents);
			}
		}
	}

	Ok(found)
}

#[proc_macro]
pub fn load_days(name: PMTokenStream) -> PMTokenStream {
	let input: Option<LitStr> = parse_macro_input!(name as Option<LitStr>);
	let input_dir = input.map(|s| s.value()).unwrap_or_else(|| "../input".to_string());

	let mut day_mods = PM2TokenStream::new();
	let mut runners = PM2TokenStream::new();
	let max: usize = 25;
	for i in  1..=max {
		let n = format!("{:02}", i);
		let smolday = format_ident!("day{}", n);
		let bigday = format_ident!("Day{}", n);
		day_mods.extend(quote! {
			pub mod #smolday;
		});
		runners.extend(quote! {
			(include_str!(concat!(#input_dir, "/", #n,".txt")), &|p, i, q| ::aoch::run_day_with_input(crate::days::#smolday::#bigday, p, q, i)),
		});
	}

	quote! {
		pub mod days {
			#day_mods
		}
		pub const RUNNERS: [(&'static str, &dyn Fn(::std::option::Option<::aoch::DayPart>, bool, &str) -> ()); #max] = [
			#runners
		];
	}.into()
}
