use blisp;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::{env, fs};

fn run_lisp(code: &String) {
    // initialize
    match blisp::init(&code) {
        Ok(exprs) => {
            // typing
            match blisp::typing(&exprs) {
                Ok(ctx) => {
                    println!("{}", code);
                    run_repl(code, &ctx);
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

fn run_repl(code: &String, ctx: &blisp::semantics::Context) {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    println!("CTRL-D to exit");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let result = blisp::eval(&line, ctx);
                match result {
                    Ok(rs) => {
                        for r in &rs {
                            println!("{}", r);
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
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("specify a file as follows:");
        println!("  {} examples/ex1.blisp", args[0]);
        return;
    }

    let content = match fs::read_to_string(&args[1]) {
        Ok(s) => s,
        Err(e) => {
            println!("error: {:?}", e);
            return;
        }
    };

    run_lisp(&content);
}
