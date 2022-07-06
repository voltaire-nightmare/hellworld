#[macro_use]
extern crate log;
extern crate simple_logger;

use clap::Parser;
use std::process;

mod fileio;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Activate debug mode
    #[clap(short, long)]
    debug: bool,

    /// Set config filepath
    #[clap(short, long)]
    config_filepath: Option<String>,

    /// Set output string
    #[clap(short, long)]
    output_string: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    // If debug mode, load the logging implementation with debug, otherwise filter until warn
    let logger: simple_logger::SimpleLogger =
        simple_logger::SimpleLogger::new().without_timestamps();
    if args.debug {
        logger.with_level(log::LevelFilter::Debug).init().unwrap();
    } else {
        logger.with_level(log::LevelFilter::Warn).init().unwrap();
    }
    debug!("Logging implementation initialized");



    // Load config file
    let config_filepath: String = match args.config_filepath {
        Some(v) => v,
        None => {
            debug!("No config filepath set; defaulting to \"./config.json\"");
            String::from("./config.json")
        }
    };

    let config: fileio::Config = match fileio::load_config_from_path(config_filepath.clone()) {
        Ok(v) => v,
        Err(_) => {
            error!("ConfigError encountered; the program cannot continue");
            process::exit(1)
        }
    };
    debug!("{config_filepath} loaded as config");



    // Set the output string
    let output_string: String = match args.output_string {
        Some(v) => v,
        None => {
            debug!("No output string set; defaulting to \"{}\"", config.output_string);
            config.output_string
        }
    };

    println!("{output_string}");
}
