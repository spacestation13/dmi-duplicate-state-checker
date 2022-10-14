use clap::Parser;
use dmi::icon::Icon;
use std::{
	fs::File,
	path::{Path, PathBuf},
};

/// Arguments for the program
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
	/// List of paths to check for duplicate states within DMI files
	#[arg(short, long)]
	paths: Vec<PathBuf>,

	/// Exits with an error code corresponding to how many duplicates were detected
	#[arg(long, default_value_t = true)]
	error: bool,

	/// If we encounter a file we can't read, output a warning
	#[arg(long, default_value_t = false)]
	read_errors: bool,
}

/// Will continue a loop if the passed expr is an error and warn with a message
macro_rules! skip_fail_yell {
	($res:expr, $err:literal, $loud:expr) => {
		match $res {
			Ok(val) => val,
			Err(e) => {
				if ($loud) {
					println!($err, e);
				}
				continue;
			}
		}
	};
}

fn main() {
	let args = Args::parse();

	let mut error_count = 0;

	for search_path in args.paths {
		let builder = globmatch::Builder::new("**/*.dmi").build(search_path);
		let globpaths: Vec<_> = builder.into_iter().flatten().collect();

		for potential_file in globpaths.iter() {
			let path = skip_fail_yell!(potential_file, "Couldn't read path: {}", args.read_errors);

			let file = skip_fail_yell!(File::open(path), "Couldn't read file: {}", args.read_errors);

			let dmi = skip_fail_yell!(Icon::load(file), "Couldn't read DMI: {}", args.read_errors);

			check_dmi(dmi, path, &mut error_count);
		}
	}
	println!("Complete!");

	if args.error {
		std::process::exit(error_count);
	}
}

/// Checks that a given DMI at a given path has no duplicate states
fn check_dmi(dmi: Icon, path: &Path, error_count: &mut i32) {
	let mut state_names = Vec::with_capacity(dmi.states.len());

	for state in dmi.states {
		if state_names.iter().any(|i| i == &state.name) {
			*error_count += 1;
			println!(
				"Duplicate state in {} : {}",
				path.to_string_lossy(),
				state.name
			);
		} else {
			state_names.push(state.name);
		}
	}
}
