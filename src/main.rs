mod constants;
mod parser;
mod types;

use parser::parse_eof_bytecode;
use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut input;
    if args.len() > 1 {
        if args[1].contains(".") {
            let filepath = args[1].clone();

            // Some Examples EOF bytecode can be found in ../inputs.json
            input = fs::read_to_string(filepath)
                .expect("Should have been able to read the file")
                .to_lowercase();
        } else {
            input = args[1].clone().to_lowercase();
        }
    } else {
        println!("Expected a filepath or hex bytecode input");
        exit(0);
    }

    // cleanse, remove whitespaces if any and remove leading 0x if any
    input = input.replace(" ", "");
    input = input.replace("0x", "");

    let bytecode = hex::decode(input).unwrap_or_else(|x| {
        println!("Err: {}", x);
        exit(0);
    });

    let eof_container = parse_eof_bytecode(&bytecode);
    println!("{}", eof_container);
}
