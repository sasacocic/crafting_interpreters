use core::fmt;
use std::error::Error;
use std::io::stdin;
use std::path::PathBuf;
use std::process::exit;
use std::{env, fs};

use expr::{Binary, Expr, Grouping, Literal, Unary};
use log::{info, trace};

mod expr;
mod parser;
mod playground;
mod scripts;
use parser::{Object, Scanner, Token};
use playground::ASTPrinter;
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
                println!("token: {}", token);
            }
        }
        Err(errors) => {
            for error in errors {
                eprintln!("ERROR: {}", &*error);
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

    run(program)?;
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    trace!("excepting input from prompt");
    loop {
        println!("> ");
        let mut read_input = String::new();
        stdin().read_line(&mut read_input)?; // I have no idea what happens when control-D (or EOF) is sent to this.... read the docs
        trace!("read from input: {:?}", &read_input);
        match run(read_input) {
            Ok(_) => {
                // everything is ok no need to do anything
                info!("prompt ran succesfully");
            }
            Err(error) => {
                // this is a boxed error. I need to figure out the error, and
                // also idk the difference of printing &dyn Error vs Box<dyn Error>
                eprint!("{}", error);
            }
        }
    }
}

fn see_tree() {
    let expr = Binary::<String> {
        left: Box::new(Unary {
            operator: Token {
                token_type: parser::TokenType::MINUS,
                line: 1,
                lexeme: "-".to_string(),
                literal: None,
            },
            right: Box::new(Literal {
                value: Object("123".to_string()),
            }),
        }),
        operator: Token {
            token_type: parser::TokenType::STAR,
            literal: None,
            lexeme: "*".to_string(),
            line: 1,
        },
        right: Box::new(Grouping {
            expression: Box::new(Literal {
                value: Object("45.67".to_string()),
            }),
        }),
    };

    let ast_priter = ASTPrinter {};

    println!("{}", ast_priter.print(Box::new(expr)));
}

fn main() -> Result<(), Box<dyn Error>> {
    see_tree();

    env_logger::init();
    trace!("starting program");

    // I'll need to differenetiate between running lox, and running the helper script

    let mut args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jilox [script]");
        exit(64); // I could also panic, but this isn't an unrecoverable error? Or is it?
    } else if args.len() == 2 {
        //let got = args.get(1);
        //dbg!(&got);
        if args.get(1) == Some(&"generate".to_string()) {
            scripts::generate_exprs()?;
        } else {
            // cloned, because I couldn't move the string out for whatever reason...
            // honestly just wanted to
            let file_name = args
                .pop()
                .ok_or("couldn't pop from args blah".to_string())?;
            run_file(file_name)?
        }
    } else {
        run_prompt()?
    }

    Ok(())
}
