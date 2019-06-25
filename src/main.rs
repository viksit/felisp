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

mod db;
use db::stmt::{execute_select, execute_insert};

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

// fn main() {

//     // Lisp layer
//     let env = &mut default_env();
//     loop {
//         println!("Felisp> ");
//         let expr = slurp_expr();
//         match parse_eval(expr, env) {
//             Ok(res) => println!("// 🔥 => {}", res),
//             Err(e) => match e {
//                 FelispErr::Reason(msg) => println!("// 🙀 => {}", msg),
//             },
//         }
//     }

// }


use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);

fn main() {
    let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);
    let encoded: Vec<u8> = bincode::serialize(&world).unwrap();

    // 8 bytes for the length of the vector, 4 bytes per float.
    assert_eq!(encoded.len(), 8 + 4 * 4);
    println!("encoded: {:?}", encoded);
    let decoded: World = bincode::deserialize(&encoded[..]).unwrap();
    println!("decoded: {:?}", decoded);
    assert_eq!(world, decoded);
}
