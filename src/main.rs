extern crate libc;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use parser::Parser;

mod interpreter;
mod command;
mod parser;
mod optimizer;
mod errors;

fn main() {
    let args : Vec<String> = env::args().skip(1).collect();

    if args.len() == 0{
        println!("Brainf*ck interpreter written by Rust.");
        println!("usage: ./cheetah_bf code.bf");
        return;
    }

    let mut f = match File::open(args[0].clone()){
        Ok(v) => v,
        Err(_)=> {
            println!("Error: Cannot open source code.");
            return;
        }
    };

    let mut code = String::new();
    if f.read_to_string(&mut code).is_err(){
        println!("Error: Cannot read source code.");
        return;
    }

    let mut p = Parser::new(code);
    let ast = match p.parse() {
        Ok(ast) => ast,
        Err(e) => {
            println!("Parse error: {:?}",e);
            return;
        }
    };

    let mut bf = interpreter::Interpreter::new();
    match bf.run(&ast){
        Ok(()) => {},
        Err(e) => println!("Runtime error: {:?}",e),
    };
}

