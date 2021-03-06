use std::cmp::Ordering;
use std::env;
use std::fs;
use dont_disappear;

struct Instruction {
	from_char: char,
	from_state: usize,
	to_char: char,
	to_state: String,
	direction: char,
}

fn main() {
	println!("Turing machine converter by Vitalii Hurianov (@not_holar).");

	let args: Vec<String> = env::args().collect();

	let path = match args.len().cmp(&2) {
		Ordering::Less => "input.txt",
		_ => &args[1],
	};

	let input = match fs::read_to_string(path) {
		Ok(input) => input,
		Err(_) => {
			println!("Failed to read the file ({}).", path);
			println!();
			println!("Usage:");
			println!("   a) drag and drop a table TM .txt file on the executable");
			println!("   b) CLI: \"converter [PATH]\" e.g. \"converter input.txt\"");
			println!("   c) put a file named input.txt in the same directory as the executable and run it");
			println!();

			dont_disappear::any_key_to_continue::default();

			panic!();
		}
	};

	let mut instructions: Vec<Instruction> = Vec::new();

	for line in input.lines() {
		let mut line: Vec<char> = line.chars().collect();

		line.push('\t');

		let from_char = line[0];
		let mut from_state = 0;
		let mut to_char = ' ';
		let mut to_state = String::new();
		let mut direction = 'S';

		let mut on_state: bool = false;

		for ch in &line[2..] {
			match ch {
				'\t' => {
					from_state += 1;

					if on_state {
						let to_state: usize = to_state.parse().unwrap();

						let to_state = match to_state {
							0 => String::from("!"),
							_ => format!("q{}", to_state),
						};

						instructions.push(Instruction {
							from_char,
							from_state,
							to_char,
							to_state,
							direction,
						});
					}

					to_state = String::new();
					on_state = false;
				}
				'<' | '>' | '.' => {
					on_state = true;

					direction = match ch {
						'<' => 'L',
						'>' => 'R',
						_ => 'S',
					}
				}
				_ => {
					if on_state {
						to_state.push(*ch);
					} else {
						to_char = match ch {
							'_' => ' ',
							_ => *ch,
						};
					}
				}
			}
		}
	}

	let mut output = String::new();

	for instruction in instructions {
		output.push_str(&format!(
			"q{},{}:{},{},{};\n",
			instruction.from_state,
			instruction.from_char,
			instruction.to_state,
			instruction.to_char,
			instruction.direction
		));
	}

	match fs::write("output.txt", output) {
		Ok(_) => println!("Successfully written the result to a file."),
		Err(_) => println!("Failed to write the result to a file."),
	}
}
