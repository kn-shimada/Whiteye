use anyhow::Result;
use clap::{crate_description, crate_name, crate_version, App, Arg};
use inkwell::context::Context;
use log::{debug, LevelFilter};
use std::fs;
use whiteye::backend::llvm::LLVMBackend;
use whiteye::backend::BackendType;

<<<<<<< HEAD
use whiteye::ast::Ast;
=======
>>>>>>> 9fe28ac (fix(backend llvm): add frontend)
use whiteye::machine::Machine;
use whiteye::parser::parse;
use whiteye::value::Value;

fn main() -> Result<()> {
    let app = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(Arg::with_name("FILE"))
        .arg(Arg::with_name("jit").long("jit"))
        .arg(Arg::with_name("dbg").short("d").long("debug"));

    let matches = app.get_matches();

    let is_debug = matches.occurrences_of("dbg") != 0;
    let is_jit = matches.occurrences_of("jit") != 0;

    let mut logger = env_logger::Builder::new();

    if is_debug {
        logger.filter_level(LevelFilter::Debug);
        logger.init();
    }

    let backend_type = if is_jit {
        BackendType::LLVM
    } else {
        BackendType::None
    };

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
                if is_jit {
                    backend.run_jit(&parsed);
                }
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
