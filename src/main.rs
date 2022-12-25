use rand::seq::SliceRandom;
use std::process::exit;
use std::path::Path;
use std::fs::{self, File};
use std::io::prelude::*;
use clap::{arg, Command, value_parser, crate_version, crate_authors};

fn exit_with_error(err_msg: String) -> ! {
	eprintln!("{}", err_msg);
	exit(1);
}

fn load_chars(filename: &String) -> Option<Vec<char>> {
	let contents_result = fs::read_to_string(filename);
	match contents_result {
		Ok(content) => return Some(content.trim().chars().collect::<Vec<char>>()),
		Err(_) => return None
	}
}

fn write_to_file(filename: &String, output: String) {
	let out_path = Path::new(filename);
	let display = out_path.display();

    let mut file = match File::create(&out_path) {
        Err(why) => exit_with_error(format!("Couldn't create new file \"{}\": {}", display, why)),
        Ok(file) => file,
    };

    match file.write(output.as_bytes()) {
        Err(why) => exit_with_error(format!("Couldn't write to file \"{}\": {}", display, why)),
        Ok(_) => println!("Successfully wrote trash to file \"{}\"", display),
    }
}

fn main() {
	let args = Command::new("MyApp")
		.version(crate_version!())
		.author(crate_authors!())
		.about("Just a Rust program that helps you create files that contain, well, random characters (AKA trash)")
		.arg(arg!([filename] "The path of the file to write into").required(true))
		.arg(arg!([filesize] "The number of total characters in the output file").value_parser(value_parser!(u64)).required(true))
		.arg(arg!(-c --chars <chars_path> "The path of the file containing the characters to be randomly selected (defaults to \"char.txt\")").default_value("char.txt").required(false))
		.get_matches();

	let filesize: &u64 = args.get_one::<u64>("filesize").unwrap_or_else(|| {
		exit_with_error("Error. Expected an integer below 2^64-1 on second argument".to_string());
	});

	let chars_path: &str = args.get_one::<String>("chars").unwrap_or_else(|| {
		exit_with_error("Couldn't load the path of the file containing the characters to randomly select. Exiting...".to_string());
	});

	let unicode_option = load_chars(&chars_path.to_string());
	let unicode_chars = match unicode_option {
		Some(chars) => chars,
		None => exit_with_error(format!("Couldn't load character vector from \"{}\"", &chars_path))
	};
	let mut output: Vec<&char> = Vec::new();

	for _ in 0..*filesize {
		output.push(unicode_chars.choose(&mut rand::thread_rng()).unwrap_or_else(|| {
			exit_with_error("Couldn't choose a random character from `unicode_chars` vector. Maybe \"char.txt\" is empty?".to_string())
		}));
	}

	write_to_file(
		args.get_one("filename").unwrap_or_else(|| {
			exit_with_error("Expected a filename as main input argument".to_string())
		}),
		output.iter().cloned().collect::<String>()
	);
}