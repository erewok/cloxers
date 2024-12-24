use clap::Parser;
use std::io::{self, Write};

use cloxers::chunk::Chunk;
use cloxers::opcodes::OpCode;
use cloxers::vm::VM;
use cloxers::value::Value;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Lox program to run (if not provided, runs in REPL mode)
    #[arg(short, long, default_missing_value = "")]
    filename: Option<String>,
}

fn run_prompt() {
    let mut interpreter = Interpreter::new();
    loop {
        // If there's an error, we want to keep running the REPL
        interpreter.reset();
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.is_empty() {
            break;
        }
        let _ = interpreter.run(&line);
    }
}

fn run_file(filename: &str) {
    let source = std::fs::read_to_string(filename).unwrap();
    todo!("Run file: {}", source);
    let mut interpreter = Interpreter::new();
    match interpreter.run(&source) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            e.exit();
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut chunk = Chunk::new();
    let _ = chunk.write_constant(Value::Number(1.2), 1);
    let _ = chunk.write_constant(Value::Number(3.4), 1);
    let _ = chunk.write(OpCode::Add.into(), 1);
    let _ = chunk.write_constant(Value::Number(5.6), 2);
    let _ = chunk.write(OpCode::Divide.into(), 4);
    chunk.write(OpCode::Return.into(),2);
    VM::new(&chunk).run().unwrap();

    if args.filename.is_none() {
        run_prompt();
    } else {
        run_file(&args.filename.unwrap_or_default());
    }
}
