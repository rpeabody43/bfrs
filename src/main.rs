mod bf;

use colored::Colorize;
use std::env;
use std::fs;

fn main() {
    let mut instance = bf::Instance::new();
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let contents: String = match fs::read_to_string(file_path) {
        Ok(x) => x,
        Err(e) => {
            err(format!("FnF: {}", e), false);
            return;
        }
    };

    let output = match instance.interpret_to_ascii(&contents) {
        Ok(x) => x,
        Err(e) => {
            err(e, true);
            return;
        }
    };
    println!("{}", output);
}

fn err(msg: String, runtime: bool) {
    let total_msg = format!(
        "{}{}: {}",
        if runtime { "Runtime " } else { "" },
        "Err",
        msg
    );
    println!("🟥 {}", total_msg.red());
}
