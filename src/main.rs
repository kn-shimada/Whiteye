use anyhow::Result;
use clap::{crate_description, crate_name, crate_version, App, Arg};
use inkwell::context::Context;
use log::{debug, LevelFilter};
use std::fs;
use std::str::FromStr;
use whiteye::backend::llvm::LLVMBackend;
use whiteye::backend::{BackendType, BACKEND_TYPES};

use whiteye::machine::Machine;
use whiteye::parser::parse;

fn main() -> Result<()> {
    let app = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(Arg::with_name("FILE").required(true))
        .arg(
            Arg::with_name("backend")
                .help("Backend type")
                .possible_values(&BACKEND_TYPES)
                .default_value("none")
                .short("B")
                .long("backend"),
        )
        .arg(Arg::with_name("verbose").short("v").long("verbose"));

    let matches = app.get_matches();

    let is_verbose = matches.occurrences_of("verbose") != 0;

    let mut logger = env_logger::Builder::new();

    if is_verbose {
        logger.filter_level(LevelFilter::Debug);
        logger.init();
    }

    let backend_type = BackendType::from_str(matches.value_of("backend").unwrap()).unwrap();

    if let Some(path) = matches.value_of("FILE") {
        let input = fs::read_to_string(path)?;

        let mut machine = Machine::new();

        debug!("Raw: \n{}", input);

        let parsed = parse(&input).unwrap_or_else(|e| panic!("{}", e));
        debug!("AST: {:?}", parsed);

        match backend_type {
            BackendType::LLVM => {
                let context = Context::create();
                let mut backend = LLVMBackend::new(&context);
                backend.run_jit(&parsed);
            }
            BackendType::None => {
                for ast in parsed {
                    machine.run(ast).unwrap_or_else(|e| panic!("{}", e));
                    debug!("machine state: {:?}", machine);
                }
            }
        };

        /*
        if !input.is_empty() {
            eprintln!("parsing error, input remaining {:?}", input);
            exit(1);
        }
        */
    }

    Ok(())
}
