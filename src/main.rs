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
use std::fmt;
use std::io;
use std::num::ParseFloatError;

#[derive(Clone,Debug)]
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

// Create a tokenizer that takes a felisp expression in string
// and converts it to an AST

fn tokenize(expr: String) -> Vec<String> {
    expr
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

// Now, convert this AST expression into a felisp expression

fn parse(tokens: & [String]) -> Result<(FelispExp, & [String]), FelispErr> {
    let (token, rest) = tokens.split_first()
        .ok_or(
            FelispErr::Reason("Could not get token".to_string())
        )?;
    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(FelispErr::Reason("unexpected `)`".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

fn read_seq(tokens: & [String]) -> Result<(FelispExp, & [String]), FelispErr> {
    let mut res: Vec<FelispExp> = vec![];
    let mut xs = tokens;
    loop { // infinite loop here
        let (next_token, rest) = xs
            .split_first()
            .ok_or(FelispErr::Reason("could not find closing `)`".to_string()))
            ?;
        if next_token == ")" {
            return Ok((FelispExp::List(res), rest)) // skip `)`, head to the token after
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

fn parse_atom(token: &str) -> FelispExp {
    let potential_float: Result<f64, ParseFloatError> = token.parse();
    match potential_float {
        Ok(v) => FelispExp::Number(v),
        Err(_) => FelispExp::Symbol(token.to_string().clone())
    }
}

fn main() {
    println!(" ==== hello Felisp!! ===");
    let _prog = "(hello (world (+ a b)))"; // &'static str or string slice
    println!("{:?}", _prog);
    println!("{:?}, ", tokenize(_prog.to_string()));
    println!("{:?}, ", parse(&tokenize(_prog.to_string())));
    println!(" ==== End felisp ====");
}
