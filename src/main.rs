use rand::seq::SliceRandom;
use std::env::args;
use std::process::exit;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

const UNICODE_CHARS: &[&str] = &[
	"a", "b", "c", "d", "e", "f", "g", "h", "i", "g", "k", "l", "m",
	"n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
	"A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
	"N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
	"1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "!", "@", "#",
	"$", "%", "^", "&", "*", "(", ")", "_", "+", "-", "=", "\\", "|",
	"]", "[", ";", "\"", ",", ".", "/", "<", ">", "?", ":", "{", "}",
	"~", "`"
];

fn print_help() {
	println!(concat!(
		"A Rust program that all it does is generate trash\n",
		"Usage: main.exe FILESIZE FILENAME\n",
		"FILESIZE: The size of the trash file in bytes\n",
		"FILENAME: The path of the output trash file"
	));
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
	let filename = &arguments[2];

	let mut output = String::new();
	for _ in 0..filesize {
		output.push_str(UNICODE_CHARS.choose(&mut rand::thread_rng()).unwrap());
	}

	let out_path = Path::new(filename);
	let display = out_path.display();

    let mut file = match File::create(&out_path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write(output.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}