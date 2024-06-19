use std::env;
use std::io::{self, BufRead};

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Read lines from stdin
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut code_blocks = Vec::new();
    let mut in_code_block = false;
    let mut current_block = String::new();

    // Parse input lines
    for line in lines {
        let line = line.unwrap();
        
        if line.starts_with("```") {
            if in_code_block {
                // End of code block, add to list
                code_blocks.push(current_block);
                current_block = String::new();
            } 
            in_code_block = !in_code_block;
        } else if in_code_block {
            // Inside a code block, append line
            current_block.push_str(&line);
            current_block.push('\n');
        }
    }

    // Handle command line arguments
    match args.len() {
        1 => {
            // No arguments, print number and first line of each code block 
            for (i, block) in code_blocks.iter().enumerate() {
                let first_line = block.lines().next().unwrap_or("");
                println!("{}: {}", i+1, first_line);
            }
        },
        2 => {
            if args[1] == "count" {
                println!("{}", code_blocks.len());
            } else {
                eprintln!("Unknown argument: {}", args[1]);
            }
        },
        3 => {
            if args[1] == "get" {
                if let Ok(num) = args[2].parse::<usize>() {
                    if num > 0 && num <= code_blocks.len() {
                        println!("{}", code_blocks[num-1]);
                    } else {
                        eprintln!("Invalid code block number: {}", num);
                    }
                } else {
                    eprintln!("Invalid number: {}", args[2]);
                }
            } else {
                eprintln!("Unknown arguments: {} {}", args[1], args[2]);
            }
        },
        _ => eprintln!("Too many arguments provided"),
    }
}
