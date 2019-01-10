extern crate libc;

use std::env;
use std::fs::File;
use std::io::prelude::*;

mod interpreter;
mod command;
mod parser;
mod optimizer;

fn main() {
    let args : Vec<String> = env::args().skip(1).collect();

    if args.len() == 0{
        println!("./brainfuck code.bf");
        return;
    }

    let mut f = match File::open(args[0].clone()){
        Ok(v) => v,
        Err(_)=> {
            println!("file not found");
            return;
        }
    };

    let mut code = String::new();
    if f.read_to_string(&mut code).is_err(){
        println!("cannot read file");
        return;
    }

    let mut bf = interpreter::Interpreter::new();
    match bf.run(code){
        Ok(()) => {},
        Err(s) => println!("{}",s),
    };
}

