use core::fmt;
use std::error::Error;
use std::io::stdin;
use std::path::PathBuf;
use std::process::exit;
use std::{env, fs};

use log::{info, trace};

mod parser;
mod playground;
use parser::Scanner;
/*
god damn it rust how the fuck should I actually return real errors
*/

/*

*/

/*
obviously need better errors, and need to play with them to see what I can do
 */
#[derive(Debug)]
enum LoxErrors {
    GeneralError(usize, String),
}

impl fmt::Display for LoxErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxErrors::GeneralError(line_numebr, error_message) => {
                write!(f, "[line {}] Error: {}", line_numebr, error_message)
            }
        }
    }
}

impl Error for LoxErrors {}

fn run(raw_program: String) -> Result<(), Box<dyn Error>> {
    // this isn't going to be useful if I pass a whole program, but is useful for
    // when I'm running things from the prompt
    trace!("running program: {:?}", &raw_program);
    let mut scanner = Scanner::new(raw_program);
    trace!("new scanner created: {:?}", &scanner);
    let scanning_result = scanner.scan_tokens();

    match scanning_result {
        Ok(tokens) => {
            for token in tokens {
                info!("token: {}", token);
            }
        }
        Err(errors) => {
            for error in errors {
                eprintln!("{}", &*error);
            }
        }
    }
    Ok(())
}

// file_name should probably be a path because that's what it is actually
fn run_file(file_name: String) -> Result<(), Box<dyn Error>> {
    let file_path = PathBuf::from(&file_name);
    trace!("run_file - running with {}", &file_name);
    trace!("file_path - running with {:?}", &file_path);
    let program = fs::read_to_string(file_path)?;

    run(program);
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    trace!("excepting input from prompt");
    loop {
        println!("> ");
        let mut read_input = String::new();
        stdin().read_line(&mut read_input)?; // I have no idea what happens when control-D (or EOF) is sent to this.... read the docs
        trace!("read from input: {:?}", &read_input);
        let trimmed_input = read_input.trim();
        if trimmed_input.is_empty() {
            break;
        }
        match run(trimmed_input.to_string()) {
            Ok(_) => {
                // everything is ok no need to do anything
            }
            Err(error) => {
                // this is a boxed error. I need to figure out the error, and
                // also idk the difference of printing &dyn Error vs Box<dyn Error>
                eprint!("{}", error);
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    trace!("starting program");

    let mut args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jilox [script]");
        exit(64); // I could also panic, but this isn't an unrecoverable error? Or is it?
    } else if args.len() == 2 {
        // cloned, because I couldn't move the string out for whatever reason...
        // honestly just wanted to
        let file_name = args
            .pop()
            .ok_or("couldn't pop from args blah".to_string())?;
        run_file(file_name)?
    } else {
        run_prompt()?
    }

    Ok(())
}
