use rand::seq::SliceRandom;
use std::env::args;
use std::process::exit;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

const UNICODE_CHARS: &[char] = &[
	'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'g', 'k', 'l', 'm',
	'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
	'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
	'1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '!', '@', '#',
	'$', '%', '^', '&', '*', '(', ')', '_', '+', '-', '=', '\\', '|',
	']', '[', ';', '\'', ',', '.', '/', '<', '>', '?', ':', '{', '}',
	'~', '`'
];

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

	let mut output: Vec<&char> = Vec::new();
	for _ in 0..filesize {
		output.push(UNICODE_CHARS.choose(&mut rand::thread_rng()).unwrap());
	}

	write_to_file(&arguments[2], output.iter().cloned().collect::<String>());
}