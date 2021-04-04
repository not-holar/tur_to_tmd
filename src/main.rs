use dont_disappear;
use std::env;
use std::fs;

enum Direction {
    Left,
    Right,
    Stay,
}
struct Instruction {
    current_char: Option<char>,
    current_state: usize,
    new_char: Option<char>,
    new_state: Option<usize>,
    new_direction: Direction,
}

fn main() {
    println!("Turing machine converter by Vitalii Hurianov (@not_holar).");

    let path = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "input.txt".to_string());

    let input = fs::read_to_string(&path).unwrap_or_else(|_| {
        println!("Failed to read the file ({}).", &path);
        println!();
        println!("Usage:");
        println!("   a) drag and drop a table TM .txt file on the executable");
        println!("   b) CLI: \"converter [PATH]\" e.g. \"converter input.txt\"");
        println!(
            "   c) put a file named input.txt in the same directory as the executable and run it"
        );
        println!();

        dont_disappear::any_key_to_continue::default();

        panic!();
    });

    let parsed_instructions: Vec<Instruction> = input
        .lines()
        .filter(|line| line.len() != 0)
        .map(|line| {
            let current_char = match line.chars().next().unwrap() {
                ' ' => None,
                x => Some(x),
            };

            line.split('\t')
                .skip(1)
                .enumerate()
                .filter(|(_, instruction)| instruction.len() > 2)
                .map(|(current_state, instruction)| {
                    let current_state = current_state + 1;

                    let new_char = match instruction.chars().nth(0).unwrap() {
                        '_' => None,
                        x => Some(x),
                    };

                    let new_direction = match instruction.chars().nth(1).unwrap() {
                        '<' => Direction::Left,
                        '>' => Direction::Right,
                        _ => Direction::Stay,
                    };

                    let new_state = match instruction[2..].parse().unwrap() {
                        0 => None,
                        x => Some(x),
                    };

                    Instruction {
                        current_char,
                        current_state,
                        new_char,
                        new_state,
                        new_direction,
                    }
                })
                .collect::<Vec<Instruction>>()
        })
        .flatten()
        .collect();

    let output = parsed_instructions
        .iter()
        .map(|instruction| {
            format!(
                "q{},{}:{},{},{};",
                instruction.current_state,
                instruction.current_char.unwrap_or(' '),
                match instruction.new_state {
                    Some(x) => format!("q{}", x),
                    None => String::from("!"),
                },
                instruction.new_char.unwrap_or(' '),
                match instruction.new_direction {
                    Direction::Left => 'L',
                    Direction::Right => 'R',
                    Direction::Stay => 'S',
                }
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    match fs::write("output.txt", output) {
        Ok(_) => println!("Successfully written the result to a file."),
        Err(_) => println!("Failed to write the result to a file."),
    }
}
