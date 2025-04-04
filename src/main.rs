/// Felisp
/// A Simple lisp inspired by Peter Norvig's lispy and risp by @stopachka
///
/// First the felisp calculator
/*
Symbol = str
Number = (int, float)
Atom   = (Symbol, Number)
List   = list
Exp    = (Atom, List)
Env    = dict
# is a mapping of {variable: value}
*/

use std::fmt;
use std::io;
use std::io::{stdin, stdout, Write};

mod lib;
use lib::data::{FelispExp, FelispEnv, FelispErr, FelispLambda, Table, Row};

use lib::db::stmt::{execute_select, execute_insert};

mod lisp_core;
use lisp_core::tokenizer::tokenize;
use lisp_core::parser::*;
use lisp_core::env::default_env;
use lisp_core::eval::*;


// Repl function
fn parse_eval(expr: String, env: &mut FelispEnv) -> Result<FelispExp, FelispErr> {
    let (parsed_exp, _) = parse(&tokenize(expr))?;
    let evaled_exp = eval(&parsed_exp, env)?;
    Ok(evaled_exp)
}

fn slurp_expr() -> String {
    let mut expr = String::new();
    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");
    expr
}

fn main() {

    // Lisp layer
    let env = &mut default_env();
    loop {
        println!("Felisp> ");
        let expr = slurp_expr();
        match parse_eval(expr, env) {
            Ok(res) => println!("// 🔥 => {}", res),
            Err(e) => match e {
                FelispErr::Reason(msg) => println!("// 🙀 => {}", msg),
            },
        }
    }

}
