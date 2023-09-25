use crate::bf;
use crate::err;

use colored::Colorize;
use std::io::{stdin, stdout, Write};

pub fn session() {
    let mut instance = bf::Instance::new();
    println!(">> bfrs interpreter <<");
    loop {
        let input = read_input(">>> ");
        match input.trim().to_lowercase().as_str() {
            "exit" => return,
            "out" => {
                let output = instance.get_ascii();
                println!("{}", output);
            }
            "new" => {
                instance = bf::Instance::new();
            }
            "input" => {
                let input_str = read_input("ENTER INPUT STRING: ");
                instance.set_input(&input_str);
            }
            "view" => preview(&instance),
            _ => match instance.update(&input) {
                Ok(()) => {}
                Err(e) => {
                    err(e, true);
                }
            },
        }
    }
}

fn read_input(msg: &'static str) -> String {
    print!("{}", msg);
    let _ = stdout().flush();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error getting user input");
    input
}

fn preview(instance: &bf::Instance) {
    let pointer = *instance.pointer();

    let mut ret = String::new();
    let offset;
    if pointer > 29995 {
        offset = 29995;
    } else if pointer > 5 {
        offset = pointer - 5;
    } else {
        offset = 0;
    }

    let start = offset;
    let end = offset + 10;
    for i in start..end {
        let val = instance.at(i);
        let mut wrapped_idx = format!("[{}]", val);
        if i == pointer {
            wrapped_idx = format!("{}", wrapped_idx.green());
        }
        ret.push_str(&wrapped_idx);
    }
    println!("{}", ret);
}
