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
	#[arg(short, long, required = true)]
	paths: Vec<PathBuf>,

	/// Exit with a code of 0 instead of how many duplicates were detected
	#[arg(short, long, default_value_t = true)]
	donterror: bool,

	/// If a file is encountered that can't be read, output a warning
	#[arg(short, long, default_value_t = false)]
	warn_read: bool,
}

/// Will continue a loop if the passed expr is an error and warn with a message
macro_rules! skip_fail_yell {
	($res:expr, $err:literal, $pth:expr, $loud:expr) => {
		match $res {
			Ok(val) => val,
			Err(e) => {
				if ($loud) {
					println!($err, $pth, e);
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
			let path = skip_fail_yell!(
				potential_file,
				"Couldn't read path:{}{}",
				"",
				args.warn_read
			);

			let file = skip_fail_yell!(
				File::open(path),
				"Couldn't read file at {} : {}",
				path.to_string_lossy(),
				args.warn_read
			);

			let dmi = skip_fail_yell!(
				Icon::load(file),
				"Couldn't read DMI at {} : {}",
				path.to_string_lossy(),
				args.warn_read
			);

			check_dmi(dmi, path, &mut error_count);
		}
	}
	println!("Complete, {} duplicates found.", error_count);

	if !args.donterror {
		std::process::exit(error_count);
	}
}

/// Checks that a given DMI at a given path has no duplicate states
fn check_dmi(dmi: Icon, path: &Path, error_count: &mut i32) {
	let mut state_names = Vec::with_capacity(dmi.states.len());

	for state in dmi.states {
		if state_names
			.iter()
			.any(|(name, movement)| (name == &state.name) && (movement == &state.movement))
		{
			*error_count += 1;
			println!(
				"Duplicate state in {} : {}",
				path.to_string_lossy(),
				state.name
			);
		} else {
			state_names.push((state.name, state.movement));
		}
	}
}
