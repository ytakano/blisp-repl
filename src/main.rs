//! # BLisp
//!
//! BLisp is a well typed Lisp like programming language which adopts effect
//! system for no_std environments.
//! BLisp supports higher order RPCs like higher order functions
//! of functional programing languages.
//!
//! This repository provides only a library crate.
//! Please see [blisp](https://github.com/ytakano/blisp),
//! or [baremetalisp](https://github.com/ytakano/baremetalisp) which is a toy OS.
//!
//! [Homepage](https://ytakano.github.io/blisp/) is here.
//!
//! ## Examples
//!
//! ```lisp
//! "Hello, World!" ; string
//! (+ 0x10 0x20)   ; 48
//! (+ 0b111 0b101) ; 12
//! (+ 0o777 0o444) ; 803
//! (car '(1 2 3))  ; (Some 1)
//! (cdr '(1 2 3))  ; '(2 3)
//! (map (lambda (x) (* x 2)) '(8 9 10)) ; '(16 18 20)
//! (fold + 0 '(1 2 3 4 5 6 7 8 9))      ; 45
//! (reverse '(1 2 3 4 5 6 7 8 9))       ; '(9 8 7 6 5 4 3 2 1)
//! (filter (lambda (x) (= (% x 2) 0)) '(1 2 3 4 5 6 7 8 9)) ; '(2 4 6 8)
//! ```
//!
//! ## Features
//!
//! - Algebraic data type
//! - Generics
//! - Hindley–Milner based type inference
//! - Effect system to separate side effects from pure functions
//! - Big integer
//! - Supporting no_std environments

use num_bigint::BigInt;
use rustyline::error::ReadlineError;
use std::{env, error::Error, fs};

fn run_lisp(code: &str) {
    // initialize
    match blisp::init(code, vec![]) {
        Ok(exprs) => {
            // typing
            match blisp::typing(exprs) {
                Ok(mut ctx) => {
                    // set callback function
                    let fun = |x: &BigInt, y: &BigInt, z: &BigInt| {
                        let n = x * y * z;
                        println!("Rust's function is called: n = {}", n);
                        Some(n)
                    };
                    ctx.set_callback(Box::new(fun));

                    println!("{}", code);
                    let _ = run_repl(code, &ctx);
                }
                Err(e) => {
                    println!("{}:{}: {}", e.pos.line + 1, e.pos.column + 1, e.msg);
                }
            }
        }
        Err(e) => {
            println!("{}:{}: {}", e.pos.line + 1, e.pos.column + 1, e.msg);
        }
    }
}

fn run_repl(code: &str, ctx: &blisp::semantics::Context) -> Result<(), Box<dyn Error>> {
    let mut rl = rustyline::DefaultEditor::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    println!("CTRL-D to exit");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                let result = blisp::eval(&line, ctx);
                match result {
                    Ok(rs) => {
                        for r in &rs {
                            match r {
                                Ok(msg) => {
                                    println!("{}", msg);
                                }
                                Err(msg) => {
                                    println!("error: {}", msg);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("{}:{}: {}", e.pos.line + 1, e.pos.column + 1, e.msg);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", code);
                println!("CTRL-D to exit");
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = if args.len() < 2 {
        "".to_owned()
    } else {
        match fs::read_to_string(&args[1]) {
            Ok(s) => s,
            Err(e) => {
                println!("error: {:?}", e);
                return;
            }
        }
    };

    run_lisp(&content);
}
