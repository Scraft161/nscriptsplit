use std::{
	env,
	fs::{
		self,
		File,
	},
	io::{
		Error,
		ErrorKind,
	},
};

// We could string match, but regex makes this easier to adapt.
use regex::Regex;

// Our set of changable constants
const FILE_EXTENSION: &str = ".ns";
const OUTPUT_DIR: &str = "script";
//static REGEX_BLOCK_START: Result<regex::Regex, regex::Error> = Regex::new(r#"/\*(f|s)\d+"#);

fn main() -> Result<(), std::io::Error> {
	// Create block start regex
	let regex_block_start = Regex::new(r#"/\*(f|s)\d+"#);

	// Get file path from CLI
	let args: Vec<String> = env::args().collect();

	// Only continue if we have the file
	if args.len() != 2 {

		println!("No file given. Exiting!");

		// We return an error to denote something went wrong.
		return Err(Error::new(ErrorKind::Other, format!("Expected 1 argument, but got {} instead", args.len() - 1)));

	}

	// Open file, since this may take a bit we notify the user.
	print!("Reading file from disk, this may take a while... ");

	let source_nscript_file = fs::read_to_string(&args[1])?;

	println!("Done!");

	// Split file and write the blocks, again we notify the user since this can take a bit
	print!("Splitting file into blocks, this may take a while... ");

	split_file_into_blocks(source_nscript_file);

	println!("Done!");

	// Return a status code of 0
	Ok(())
}

// Write each block into it's own file, F-[N] for F-blocks S-[N] for scenes/subroutines
fn split_file_into_blocks(file: String) {
	// Match starting blocks by regex
	// For each block in file
		// write_block_to_file(is_f_block, block_number)

	// Write the current block to file
	write_block_to_file(true, 161, "I have mental issues");

	// Close the file gracefully to prevent the OS from locking the file after the program exits (edge case for windows, but always a good thing to do)
	drop(file);
	// PS: Rust ownership is a bit odd at times, just think of it that `source_nscript_file` became
	// `file` after being passed to this function and that now neither exists anymore.
	todo!();
}

fn write_block_to_file(is_f_block: bool, block_number: i32, block_data: &str) {
	dbg!(is_f_block);
	dbg!(block_number);
	dbg!(FILE_EXTENSION);
	dbg!(OUTPUT_DIR);

	let mut block_file_path = String::new();

	// Rust version of a switch statement
	match is_f_block {
		true => block_file_path = "F-Blocks/".to_string(),
		false => block_file_path = "S-Blocks/".to_string()
	}

	// Create handler to the new file, this also creates the file itself
	let mut block_file = File::create(block_file_path + &block_number.to_string() + FILE_EXTENSION)
		.expect("Could not create file for block {block_file_path}{block_number}{FILE_EXTENSION}");

	dbg!(block_file);
	// Close the file by dropping the handler, it should normally get dropped at the end of this
	// function, but we want to avoid windows locking the file after the program exits.
	//drop(block_file);
	todo!();
}
