#![warn(clippy::all, clippy::nursery)]

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
	#[arg(short, long, default_value_t = false)]
	donterror: bool,

	/// If a file is encountered that can't be read, output a warning
	#[arg(short, long, default_value_t = true)]
	warn_read: bool,

	/// Message formatting is in the correct format for GitHub Actions error reporting
	#[arg(long, default_value_t = false)]
	actions_fmt: bool,
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

			let dmi = match Icon::load(file) {
				Ok(val) => val,
				Err(e) => {
					if args.warn_read {
						if args.actions_fmt {
							println!(
								"::error file={},title=Weird DMI format::Couldn't read DMI at {} : {}",
								path.to_string_lossy(),
								path.to_string_lossy(),
								e
							)
						} else {
							println!("Couldn't read DMI at {} : {}", path.to_string_lossy(), e)
						}
					}
					continue;
				}
			};

			check_dmi(dmi, path, &mut error_count, args.actions_fmt);
		}
	}
	println!("Complete, {error_count} duplicates found.");

	if !args.donterror {
		std::process::exit(error_count);
	}
}

/// Checks that a given DMI at a given path has no duplicate states
fn check_dmi(dmi: Icon, path: &Path, error_count: &mut i32, actions_fmt: bool) {
	let mut state_names = Vec::with_capacity(dmi.states.len());

	for state in dmi.states {
		if state_names
			.iter()
			.any(|(name, movement)| (name == &state.name) && (movement == &state.movement))
		{
			*error_count += 1;
			if actions_fmt {
				println!(
					"::error file={},title=Duplicate Icon State::Duplicate state in {} : {}",
					path.to_string_lossy(),
					path.to_string_lossy(),
					state.name
				)
			} else {
				println!(
					"Duplicate state in {} : {}",
					path.to_string_lossy(),
					state.name
				)
			}
		} else {
			state_names.push((state.name, state.movement));
		}
	}
}
