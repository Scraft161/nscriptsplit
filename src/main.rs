use std::{
	env,
	fs::{
		self,
		File,
	},
	io::{
		self,
		prelude::*,
		BufReader,
		Error,
		ErrorKind,
		Write,
	},
	path::Path,
};

// We could string match, but regex makes this easier to adapt.
use regex::Regex;

// Our set of changable constants
const FILE_EXTENSION: &str = ".ns";
const OUTPUT_DIR: &str = "script";

fn main() -> Result<(), std::io::Error> {
	// Get file path from CLI
	let args: Vec<String> = env::args().collect();

	// Only continue if we have the file
	if args.len() != 2 {

		println!("No file given. Exiting!");

		// We return an error to denote something went wrong.
		return Err(Error::new(ErrorKind::Other, format!("Expected 1 argument, but got {} instead", args.len() - 1)));

	}

	// Open file, since this may take a bit we notify the user.
	print!("Reading file from disk... ");
	io::stdout().flush().unwrap();

	let source_nscript_file = File::open(&args[1])?;

	println!("Done!");

	// Split file and write the blocks, again we notify the user since this can take a bit
	print!("Splitting file into blocks... ");
	io::stdout().flush().unwrap();

	split_file_into_blocks(source_nscript_file);

	println!("Done!");

	// Return a status code of 0
	Ok(())
}

// Write each block into it's own file, F-[N] for F-blocks S-[N] for scenes/subroutines
fn split_file_into_blocks(file: File) {
	// Create top directory
	fs::create_dir_all(OUTPUT_DIR).unwrap();

	let regex_block_start = Regex::new(r#"^\*(s|f)([\d]+)$"#).unwrap();

	let reader = BufReader::new(file);

	// Values used for the current block
	let mut is_f_block: bool = false;
	let mut block_number: i32 = -1;	// Default value so we can catch malformed block identifiers
	let mut block_data = String::new();

	for line in reader.lines() {
		let ln = line.unwrap().trim_end().to_string();
		if regex_block_start.is_match(&ln) {
			// Write last block if it contains data
			if block_data != String::new() {
				// Write file
				write_block_to_file(is_f_block, block_number, &block_data);
			}
			// Get block type and id from match
			for capture in regex_block_start.captures(&ln) {
				//println!("block type: {}\nblock num:  {}", &capture[1], &capture[2]);

				if &capture[1] == "f" {
					is_f_block = true;
				}

				// get the block number as integer
				block_number = capture[2].parse::<i32>().unwrap();
			}

			// Reset block data
			block_data = String::new() + &(ln.to_string() + "\n");

		} else {
			block_data += &(ln.to_string() + "\n");
		}
	}
}

fn write_block_to_file(is_f_block: bool, block_number: i32, block_data: &str) {
	let mut block_dir_path = OUTPUT_DIR.to_string() + "/";

	// Rust version of a switch statement, we just use this to append in a readable way.
	match is_f_block {
		true => block_dir_path += "F-Blocks/",
		false => block_dir_path += "S-Blocks/",
	}

	// Create direcory if it doesn't exist
	if ! Path::new(&block_dir_path).is_dir() {
		fs::create_dir_all(&block_dir_path).unwrap();
	}

	let block_file_path = block_dir_path + &block_number.to_string() + FILE_EXTENSION;

	// Create handler to the new file, this also creates the file itself if it doesn't exist yet.
	let mut block_file = File::create(&block_file_path)
		.expect("Could not create file for block {block_file_path}{block_number}{FILE_EXTENSION}");

	// Finally write the current block to the file
	block_file.write(block_data.as_bytes()).unwrap();

	/*println!("[Debug]:
	F-Block:      {is_f_block}
	Block number: {block_number}
	Block path:   {block_file_path}");*/
}
