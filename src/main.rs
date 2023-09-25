mod bf;
mod interpreter;

use colored::Colorize;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_path = &args[1];
        let input: Option<&String> = match args.len() > 2 {
            true => Some(&args[2]),
            false => None
        };
        read_from_file(file_path, input);
    } else {
        interpreter::session();
    }
}

fn read_from_file(file_path: &String, input: Option<&String>) {
    let mut instance = bf::Instance::new();
    if let Some(input_str) = input {
        instance.set_input(input_str)
    }

    let contents: String = match fs::read_to_string(file_path) {
        Ok(x) => x,
        Err(e) => {
            err(format!("FnF: {}", e), false);
            return;
        }
    };

    match instance.update(&contents) {
        Ok(()) => {},
        Err(e) => {
            err(e, true);
            return;
        }
    };
    
    let output = instance.get_ascii();
    println!("{}", output);
}

pub fn err(msg: String, runtime: bool) {
    let total_msg = format!(
        "{}{}: {}",
        if runtime { "Runtime " } else { "" },
        "Err",
        msg
    );
    println!("🟥 {}", total_msg.red());
}
