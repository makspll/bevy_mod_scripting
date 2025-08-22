#![allow(missing_docs)]
use std::{env, fs::File, io, process::exit};

use clap::{Arg, Command};
use env_logger::Builder;
use log::LevelFilter;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook_lad_preprocessor::LADPreprocessor;

// use mdbook_lad_preprocessor::LADPreprocessor;

fn init_logger() {
    let mut builder = Builder::new();

    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse_filters(&var);
    } else {
        builder.filter(None, LevelFilter::Info);
    }

    // target lad.log file in current directory
    // print pwd
    if let Ok(file) = File::create("./lad.log") {
        let target = Box::new(file);
        builder.target(env_logger::Target::Pipe(target));
    }

    builder.init();

    log::debug!("Debug logging enabled");
}

pub fn make_app() -> Command {
    Command::new("nop-preprocessor")
        .about("A mdbook preprocessor which does precisely nothing")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    let matches = make_app().get_matches();
    if let Some(sub_args) = matches.subcommand_matches("supports") {
        let renderer = match sub_args.get_one::<String>("renderer") {
            Some(r) => r,
            None => {
                log::error!("No renderer specified");
                exit(1)
            }
        };

        if LADPreprocessor.supports_renderer(renderer) {
            exit(0)
        } else {
            exit(1)
        }
    } else {
        let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;
        let processed_book = LADPreprocessor.run(&ctx, book)?;
        serde_json::to_writer(io::stdout(), &processed_book)?;
        exit(0)
    }
}
