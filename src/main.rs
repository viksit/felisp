/// Felisp
/// A Simple lisp inspired by Peter Norvig's lispy
///
/// First the rutth calculator
/*
    Symbol = str
    Number = (int, float)
    Atom   = (Symbol, Number)
    List   = list
    Exp    = (Atom, List)
    Env    = dict
    # is a mapping of {variable: value}
 */

use std::collections::HashMap;
//use std::fmt;
//use std::io;


#[derive(Clone)]
enum FelispExp{
    Symbol(String),
    Number(f64),
    List(Vec<FelispExp>)
}

#[derive(Debug)]
enum FelispErr {
    Reason(String),
}

#[derive(Clone)]
struct RispEnv {
    data: HashMap<String, FelispExp>
}

// Create a tokenizer that takes a felisp expression
// and converts it to an AST

fn tokenize(expr: String) -> Vec<String> {
    expr
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}




fn main() {
    println!(" ==== hello Felisp!! ===");
    let _prog = "(hello (world (+ a b)))"; // &'static str or string slice
    println!("{:?}", _prog);
    println!("{:?}, ", tokenize(_prog.to_string()));
    println!(" ==== End felisp ====");
}
