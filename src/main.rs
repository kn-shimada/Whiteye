use anyhow::Result;
use clap::{crate_description, crate_name, crate_version, App, Arg};
use log::{debug, LevelFilter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use whiteye::machine::Machine;
use whiteye::parser::parse;

fn main() -> Result<()> {
    let app = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(Arg::with_name("FILE"))
        .arg(Arg::with_name("dbg").short("d").long("debug"));

    let matches = app.get_matches();

    let is_debug = matches.occurrences_of("dbg") != 0;

    let mut logger = env_logger::Builder::new();

    if is_debug {
        logger.filter_level(LevelFilter::Debug);
    } else {
        logger.filter_level(LevelFilter::Info);
    }

    logger.init();

    if let Some(path) = matches.value_of("FILE") {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);

        let mut machine = Machine::new();

        for line in reader.lines() {
            let line = line.unwrap();

            debug!("Raw: {}", line);

            let (input, parsed) = parse(&line).unwrap();
            if !input.is_empty() {
                eprintln!("parsing error, input remaining {:?}", input);
                exit(1);
            }

            debug!("AST: {:?}", parsed);

            machine.run(parsed);

            debug!("machine state: {:?}", machine);
        }
    }

    Ok(())
}
