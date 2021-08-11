use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use whiteye::machine::Machine;
use whiteye::parser::parse;

fn main() -> Result<()> {
    let path = "program.txt";
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);

    let mut machine = Machine::new();

    for line in reader.lines() {
        let line = line.unwrap();
        println!("Raw: {}", line);
        let (input, parsed) = parse(&line).unwrap();
        if !input.is_empty() {
            eprintln!("parsing error, input remaining {:?}", input);
            exit(1);
        }
        println!("AST: {:?}", parsed);

        machine.run(parsed);
        println!("Current machine state: {:?}", machine);
    }

    println!("Final: {:?}", machine);

    Ok(())
}
