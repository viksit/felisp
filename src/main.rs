/// Felisp
/// A Simple lisp inspired by Peter Norvig's lispy and risp by @stopachka
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
use std::io::{stdin,stdout,Write};

#[derive(Clone)]
enum FelispExp {
    Symbol(String),
    Number(f64),
    List(Vec<FelispExp>),
    Func(fn(&[FelispExp]) -> Result<FelispExp, FelispErr>) // function evaluations
}
impl fmt::Display for FelispExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            FelispExp::Symbol(s) => s.clone(),
            FelispExp::Number(n) => n.to_string(),
            FelispExp::List(list) => {
                let xs: Vec<String> = list
                    .iter()
                    .map(|x| x.to_string())
                    .collect();
                format!("({})", xs.join(","))
            },
            FelispExp::Func(_) => "Function {}".to_string(),
        };

        write!(f, "{}", str)
    }
}


#[derive(Debug)]
enum FelispErr {
    Reason(String),
}

#[derive(Clone)]
struct FelispEnv {
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

// convert each atom into a number or a symbol
fn parse_atom(token: &str) -> FelispExp {
    let potential_float: Result<f64, ParseFloatError> = token.parse();
    match potential_float {
        Ok(v) => FelispExp::Number(v),
        Err(_) => FelispExp::Symbol(token.to_string().clone())
    }
}

// Set up the environment to control functions and operators
fn default_env() -> FelispEnv {
    let mut data: HashMap<String, FelispExp> = HashMap::new();

    // Addition operators
    data.insert(
        "+".to_string(), FelispExp::Func(
            |args: &[FelispExp]| -> Result<FelispExp, FelispErr> {
                let sum = parse_list_of_floats(args)?.iter().fold(0.0, |sum, a| sum + a);
                Ok(FelispExp::Number(sum))
            }));

    // Subtraction operator
    data.insert(
        "-".to_string(), FelispExp::Func(
            |args: &[FelispExp]| -> Result<FelispExp, FelispErr> {
                let floats = parse_list_of_floats(args)?;
                let first = *floats.first()
                    .ok_or(FelispErr::Reason("expected atleast one number".to_string()))?;
                let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);
                Ok(FelispExp::Number(first - sum_of_rest))
            }));

    FelispEnv {data} // Return expression
}

// Helper function that enforces all FelispExp's that we receive are floats
fn parse_list_of_floats(args: &[FelispExp]) -> Result<Vec<f64>, FelispErr> {
    args
        .iter()
        .map(|x| parse_single_float(x))
        .collect() // no ; since return expression
}

fn parse_single_float(exp: &FelispExp) -> Result<f64, FelispErr> {
    match exp {
        FelispExp::Number(num) => Ok(*num),
        _ => Err(FelispErr::Reason("expected a number".to_string()))
    }
}

// Eval function
fn eval(exp: &FelispExp, env: &mut FelispEnv) -> Result<FelispExp, FelispErr> {
  match exp {
    FelispExp::Symbol(k) =>
        env.data.get(k)
        .ok_or(
          FelispErr::Reason(
            format!("unexpected symbol k='{}'", k)
          )
        )
        .map(|x| x.clone())
    ,
    FelispExp::Number(_a) => Ok(exp.clone()),
    FelispExp::List(list) => {
      let first_form = list
        .first()
        .ok_or(FelispErr::Reason("expected a non-empty list".to_string()))?;
      let arg_forms = &list[1..];
      let first_eval = eval(first_form, env)?;
      match first_eval {
        FelispExp::Func(f) => {
          let args_eval = arg_forms
            .iter()
            .map(|x| eval(x, env))
            .collect::<Result<Vec<FelispExp>, FelispErr>>();
          f(&args_eval?)
        },
        _ => Err(
          FelispErr::Reason("first form must be a function".to_string())
        ),
      }
    },
    FelispExp::Func(_) => Err(
      FelispErr::Reason("unexpected form".to_string())
    ),
  }
}

// Repl function


fn parse_eval(expr: String, env: &mut FelispEnv) -> Result<FelispExp, FelispErr> {
    let (parsed_exp, _) = parse(&tokenize(expr))?;
    let evaled_exp = eval(&parsed_exp, env)?;

    Ok(evaled_exp)
}

fn slurp_expr() -> String {
    let mut expr = String::new();
    io::stdin().read_line(&mut expr)
        .expect("Failed to read line");
    expr
}

fn main() {
    let env = &mut default_env();
    loop {
        println!("risp >");
        let expr = slurp_expr();
        match parse_eval(expr, env) {
            Ok(res) => println!("// 🔥 => {}", res),
            Err(e) => match e {
                FelispErr::Reason(msg) => println!("// 🙀 => {}", msg),
            },
        }
    }
}

// fn main() {
//     println!(" ==== hello Felisp!! ===");
//     //let _prog = "(hello (world (+ a b)))"; // &'static str or string slice
//     let _prog = "(+ 1 2)";
//     println!("{:?}", _prog);
//     println!("{:?}, ", tokenize(_prog.to_string()));
//     println!("{:?}, ", parse(&tokenize(_prog.to_string())));

//     println!(" ==== End felisp ====");

//     /*let mut s=String::new();
//     print!("Please enter some text: ");
//     let _=stdout().flush();
//     stdin().read_line(&mut s).expect("Did not enter a correct string");
//     if let Some('\n')=s.chars().next_back() {
//         s.pop();
//     }
//     if let Some('\r')=s.chars().next_back() {
//         s.pop();
//     }
//     println!("You typed: {}",s);*/

//     // print!("Please enter some text: ");
//     // let _=stdout().flush();
//     // let mut input_text = String::new();
//     // io::stdin()
//     //     .read_line(&mut input_text)
//     //     .expect("failed to read from stdin");

//     // let trimmed = input_text.trim();
//     // match trimmed.parse::<u32>() {
//     //     Ok(i) => println!("your integer input: {}", i),
//     //     Err(..) => println!("this was not an integer: {}", trimmed),
//     // };



// }
