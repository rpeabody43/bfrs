mod bf;

use clap::Parser;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Parser)]
struct Cli {
    filepath: std::path::PathBuf,
}

fn main(){
    // let args = Cli::parse();
    // let file = File::open(args.filepath)?;
    // let mut reader = BufReader::new(file);
    //
    // for line in reader.lines()? {
    //
    // }
    let mut instance = bf::Instance::new();
    let test_str = ">>++[<+++++>-]<+[<++++++>-]<.>>+++[<+++>-]<[<+++++>-]<.++++.+.-----.-.>>+++[<++++>-]<+[<------>-]<.>>+++[<++++>-]<+[<++++++>-]+++[<+++>-]<.>++++[<------>-]<++.>+++++[<++>-]<.>+++[<-->-]<.>>++[<+++++>-]<[<------->-]<+.>>++++[<++++>-]<+[<+++++>-]<.-----.";
    let output = match instance.interpret_to_ascii(test_str) {
        Ok(x) => x,
        Err(e) => panic!("{}", e)
    };
    println!("{}", output);
}


