use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use clap::{crate_description, crate_name, crate_version, App, Arg};

use whiteye::machine::Machine;
use whiteye::parser::parse;

fn main() -> Result<()> {
    let app = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(Arg::with_name("FILE"));

    let matches = app.get_matches();

    if let Some(path) = matches.value_of("FILE") {
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
            println!("machine state: {:?}", machine);
        }
    }

    Ok(())
}
