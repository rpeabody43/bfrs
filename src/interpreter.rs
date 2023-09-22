use crate::bf;
use crate::err;

use std::io::{stdin, stdout, Write};

pub fn session() {
    let mut instance = bf::Instance::new();
    println!("-- BFRS Interpreter --");
    loop {
        print!(">>> ");
        let _ = stdout().flush();
        let input = read_input();
        match input.as_str().trim() {
            "exit" => return,
            "out" => {
                let output = instance.get_ascii();
                println!("{}", output);
            }
            _ => match instance.update(&input) {
                Ok(()) => {}
                Err(e) => {
                    err(e, true);
                }
            },
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error getting user input");
    input
}
