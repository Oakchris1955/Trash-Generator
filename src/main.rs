use rand::seq::SliceRandom;
use std::env::args;
use std::process::exit;
use std::path::Path;
use std::fs::{self, File};
use std::io::prelude::*;

const CHARS_FILE_NAME: &str = "char.txt";

fn print_help() {
	println!(concat!(
		"A Rust program that all it does is generate trash\n",
		"Usage: main.exe FILESIZE FILENAME\n",
		"FILESIZE: The size of the trash file in bytes\n",
		"FILENAME: The path of the output trash file"
	));
}

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
    let arguments: Vec<String> = args().collect();
	if arguments.len() < 3 {
		print_help();
		eprintln!("\nError. Expected at least 2 items. Exiting...");
		exit(1);
	}
	match arguments.iter().find(|item| item == &&String::from("--help") || item == &&String::from("-h")) {
		Some(_) => {print_help();exit(0);},
		None => ()
	}
	let filesize: u64 = arguments[1].parse().unwrap_or_else(|_err| {
		eprintln!("Error. Expected an integer below 2^64-1 on second argument");
		exit(1);
	});

	let unicode_option = load_chars(&CHARS_FILE_NAME.to_string());
	let unicode_chars = match unicode_option {
		Some(chars) => chars,
		None => exit_with_error(format!("Couldn't load character vector from {}", &CHARS_FILE_NAME))
	};
	let mut output: Vec<&char> = Vec::new();

	for _ in 0..filesize {
		output.push(unicode_chars.choose(&mut rand::thread_rng()).unwrap_or_else(|| {
			exit_with_error(format!("Couldn't choose a random character from `unicode_chars` vector. Maybe \"char.txt\" is empty?"))
		}));
	}

	write_to_file(&arguments[2], output.iter().cloned().collect::<String>());
}