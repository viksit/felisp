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
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::{stdin, stdout, Write};
use std::num::ParseFloatError;
use std::rc::Rc;
use std::process;


#[derive(Clone)]
enum FelispExp {
    Bool(bool),
    Symbol(String),
    Number(f64),
    List(Vec<FelispExp>),
    Func(fn(&[FelispExp]) -> Result<FelispExp, FelispErr>), // function evaluations
    Lambda(FelispLambda),
}


impl fmt::Display for FelispExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            FelispExp::Symbol(s) => s.clone(),
            FelispExp::Number(n) => n.to_string(),
            FelispExp::List(list) => {
                let xs: Vec<String> = list.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(","))
            }
            FelispExp::Func(_) => "Function {}".to_string(),
            FelispExp::Bool(a) => a.to_string(),
            FelispExp::Lambda(_) => "Lambda {}".to_string(),
        };

        write!(f, "{}", str)
    }
}

#[derive(Debug)]
enum FelispErr {
    Reason(String),
}

#[derive(Clone)]
struct FelispEnv<'a> {
    data: HashMap<String, FelispExp>,
    outer: Option<&'a FelispEnv<'a>>,
}

#[derive(Clone)]
struct FelispLambda {
    params_exp: Rc<FelispExp>,
    body_exp: Rc<FelispExp>
}

// Create a tokenizer that takes a felisp expression in string
// and converts it to an AST

fn tokenize(expr: String) -> Vec<String> {
    expr.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|x| x.to_string())
        .collect()
}

// Now, convert this AST expression into a felisp expression

fn parse(tokens: &[String]) -> Result<(FelispExp, &[String]), FelispErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(FelispErr::Reason("Could not get token".to_string()))?;
    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(FelispErr::Reason("unexpected `)`".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

fn read_seq(tokens: &[String]) -> Result<(FelispExp, &[String]), FelispErr> {
    let mut res: Vec<FelispExp> = vec![];
    let mut xs = tokens;
    loop {
        // infinite loop here
        let (next_token, rest) = xs
            .split_first()
            .ok_or(FelispErr::Reason("could not find closing `)`".to_string()))?;
        if next_token == ")" {
            return Ok((FelispExp::List(res), rest)); // skip `)`, head to the token after
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

// convert each atom into a number or a symbol
fn parse_atom(token: &str) -> FelispExp {
    match token.as_ref() {
        "true" => FelispExp::Bool(true),
        "false" => FelispExp::Bool(false),
        _ => {
            let potential_float: Result<f64, ParseFloatError> = token.parse();
            match potential_float {
                Ok(v) => FelispExp::Number(v),
                Err(_) => FelispExp::Symbol(token.to_string().clone())
            }
        }
    }
}
macro_rules! ensure_tonicity {
    ($check_fn:expr) => {{
        |args: &[FelispExp]| -> Result<FelispExp, FelispErr> {
            let floats = parse_list_of_floats(args)?;
            let first = floats.first().ok_or(FelispErr::Reason("expected at least one number".to_string()))?;
            let rest = &floats[1..];
            fn f (prev: &f64, xs: &[f64]) -> bool {
                match xs.first() {
                    Some(x) => $check_fn(prev, x) && f(x, &xs[1..]),
                    None => true,
                }
            };
            Ok(FelispExp::Bool(f(first, rest)))
        }
    }};
}

// Set up the environment to control functions and operators
fn default_env<'a>() -> FelispEnv<'a> {
    let mut data: HashMap<String, FelispExp> = HashMap::new();

    // Addition operators
    data.insert(
        "+".to_string(),
        FelispExp::Func(|args: &[FelispExp]| -> Result<FelispExp, FelispErr> {
            let sum = parse_list_of_floats(args)?
                .iter()
                .fold(0.0, |sum, a| sum + a);
            Ok(FelispExp::Number(sum))
        }),
    );

    // Subtraction operator
    data.insert(
        "-".to_string(),
        FelispExp::Func(|args: &[FelispExp]| -> Result<FelispExp, FelispErr> {
            let floats = parse_list_of_floats(args)?;
            let first = *floats
                .first()
                .ok_or(FelispErr::Reason("expected atleast one number".to_string()))?;
            let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);
            Ok(FelispExp::Number(first - sum_of_rest))
        }),
    );

    data.insert(
        "=".to_string(),
        FelispExp::Func(ensure_tonicity!(|a, b| a == b))
    );
    data.insert(
        ">".to_string(),
        FelispExp::Func(ensure_tonicity!(|a, b| a > b))
    );
    data.insert(
        ">=".to_string(),
        FelispExp::Func(ensure_tonicity!(|a, b| a >= b))
    );
    data.insert(
        "<".to_string(),
        FelispExp::Func(ensure_tonicity!(|a, b| a < b))
    );
    data.insert(
        "<=".to_string(),
        FelispExp::Func(ensure_tonicity!(|a, b| a <= b))
    );

    FelispEnv { data, outer: None } // Return expression
}

// Helper function that enforces all FelispExp's that we receive are floats
fn parse_list_of_floats(args: &[FelispExp]) -> Result<Vec<f64>, FelispErr> {
    args.iter().map(|x| parse_single_float(x)).collect() // no ; since return expression
}

fn parse_single_float(exp: &FelispExp) -> Result<f64, FelispErr> {
    match exp {
        FelispExp::Number(num) => Ok(*num),
        _ => Err(FelispErr::Reason("expected a number".to_string())),
    }
}

fn eval_if_args(arg_forms: &[FelispExp], env: &mut FelispEnv) -> Result<FelispExp, FelispErr> {
    let test_form = arg_forms.first().ok_or(
        FelispErr::Reason(
            "expected test form".to_string(),
        )
    )?;
    let test_eval = eval(test_form, env)?;
    match test_eval {
        FelispExp::Bool(b) => {
            let form_idx = if b { 1 } else { 2 };
            let res_form = arg_forms.get(form_idx)
                .ok_or(FelispErr::Reason(
                    format!("expected form idx={}", form_idx)
                ))?;
            let res_eval = eval(res_form, env);

            res_eval
        },
        _ => Err(
            FelispErr::Reason(format!("unexpected test form='{}'", test_form.to_string()))
        )
    }
}


fn eval_defn_args(arg_forms: &[FelispExp], env: &mut FelispEnv) -> Result<FelispExp, FelispErr> {
    let first_form = arg_forms.first().ok_or(
        FelispErr::Reason(
            "expected first form".to_string(),
        )
    )?;
    let first_str = match first_form {
        FelispExp::Symbol(s) => Ok(s.clone()),
        _ => Err(FelispErr::Reason(
            "expected first form to be a symbol".to_string(),
        ))
    }?;
    let second_form = arg_forms.get(1).ok_or(
        FelispErr::Reason(
            "expected second form".to_string(),
        )
    )?;
    if arg_forms.len() > 2 {
        return Err(
            FelispErr::Reason(
                "defn can only have two forms ".to_string(),
            )
        )
    }
    let second_eval = eval(second_form, env)?;
    env.data.insert(first_str, second_eval);

    Ok(first_form.clone())
}

fn eval_select_args(arg_forms: &[FelispExp], env: &mut FelispEnv) -> Result<FelispExp, FelispErr> {
    let first_form = arg_forms.first().ok_or(
        FelispErr::Reason(
            "expected first form".to_string(),
        )
    )?;
    println!(">> Called Select");
    Ok(first_form.clone())
}

fn eval_insert_args(arg_forms: &[FelispExp], env: &mut FelispEnv) -> Result<FelispExp, FelispErr> {
    let first_form = arg_forms.first().ok_or(
        FelispErr::Reason(
            "expected first form".to_string(),
        )
    )?;
    let second_form = arg_forms.get(1).ok_or(
        FelispErr::Reason(
            "expected second form".to_string(),
        )
    )?;
    let third_form = arg_forms.get(2).ok_or(
        FelispErr::Reason(
            "expected second form".to_string(),
        )
    )?;
    println!("Called insert {} {} {}", first_form, second_form, third_form);
    Ok(first_form.clone())
}

fn eval_exit_args(arg_forms: &[FelispExp], env: &mut FelispEnv) -> Result<FelispExp, FelispErr> {
    println!("Called exit");
    process::exit(0x0100);
}


fn eval_lambda_args(arg_forms: &[FelispExp]) -> Result<FelispExp, FelispErr> {
    let params_exp = arg_forms.first().ok_or(
        FelispErr::Reason(
            "expected args form".to_string(),
        )
    )?;
    let body_exp = arg_forms.get(1).ok_or(
        FelispErr::Reason(
            "expected second form".to_string(),
        )
    )?;

    if arg_forms.len() > 2 {
        return Err(
            FelispErr::Reason(
                "fn definition can only have two forms".to_string()
            )
        )
    }

    Ok(
        FelispExp::Lambda(
            FelispLambda {
                body_exp: Rc::new(body_exp.clone()),
                params_exp: Rc::new(params_exp.clone()),
            }
        )
    )
}

fn eval_built_in_form(
    exp: &FelispExp, arg_forms: &[FelispExp], env: &mut FelispEnv
) -> Option<Result<FelispExp, FelispErr>> {
    match exp {
        FelispExp::Symbol(s) =>
            match s.as_ref() {
                "if" => Some(eval_if_args(arg_forms, env)),
                "defn" => Some(eval_defn_args(arg_forms, env)),
                "fn" => Some(eval_lambda_args(arg_forms)),
                "select" => Some(eval_select_args(arg_forms, env)),
                "insert" => Some(eval_insert_args(arg_forms, env)),
                "exit" => Some(eval_exit_args(arg_forms, env)),
                _ => None,
            }
        ,
        _ => None,
    }
}

fn env_get(k: &str, env: &FelispEnv) -> Option<FelispExp> {
    match env.data.get(k) {
        Some(exp) => Some(exp.clone()),
        None => {
            match &env.outer {
                Some(outer_env) => env_get(k, &outer_env),
                None => None
            }
        }
    }
}


fn eval_forms(arg_forms: &[FelispExp], env: &mut FelispEnv) -> Result<Vec<FelispExp>, FelispErr> {
    arg_forms
        .iter()
        .map(|x| eval(x, env))
        .collect()
}

fn parse_list_of_symbol_strings(form: Rc<FelispExp>) -> Result<Vec<String>, FelispErr> {
    let list = match form.as_ref() {
        FelispExp::List(s) => Ok(s.clone()),
        _ => Err(FelispErr::Reason(
            "expected args form to be a list".to_string(),
        ))
    }?;
    list
        .iter()
        .map(
            |x| {
                match x {
                    FelispExp::Symbol(s) => Ok(s.clone()),
                    _ => Err(FelispErr::Reason(
                        "expected symbols in the argument list".to_string(),
                    ))
                }
            }
        ).collect()
}

fn env_for_lambda<'a>(
    params: Rc<FelispExp>,
    arg_forms: &[FelispExp],
    outer_env: &'a mut FelispEnv,
) -> Result<FelispEnv<'a>, FelispErr> {

    let ks = parse_list_of_symbol_strings(params)?;
    if ks.len() != arg_forms.len() {
        return Err(
            FelispErr::Reason(
                format!("expected {} arguments, got {}", ks.len(), arg_forms.len())
            )
        );
    }
    let vs = eval_forms(arg_forms, outer_env)?;
    let mut data: HashMap<String, FelispExp> = HashMap::new();
    for (k, v) in ks.iter().zip(vs.iter()) {
        data.insert(k.clone(), v.clone());
    }
    Ok(
        FelispEnv {
            data,
            outer: Some(outer_env),
        }
    )

}

// Eval function
fn eval(exp: &FelispExp, env: &mut FelispEnv) -> Result<FelispExp, FelispErr> {
    match exp {
        FelispExp::Number(_a) => Ok(exp.clone()),
        FelispExp::Func(_) => Err(FelispErr::Reason("unexpected form".to_string())),
        FelispExp::Bool(_a) => Ok(exp.clone()),
        FelispExp::List(list) => {
            let first_form = list
                .first()
                .ok_or(FelispErr::Reason("expected a non-empty list".to_string()))?;
            let arg_forms = &list[1..];
            match eval_built_in_form(first_form, arg_forms, env) {
                Some(res) => res,
                None => {
                    let first_eval = eval(first_form, env)?;
                    match first_eval {
                        FelispExp::Func(f) => {
                            let args_eval = arg_forms
                                .iter()
                                .map(|x| eval(x, env))
                                .collect::<Result<Vec<FelispExp>, FelispErr>>();
                            return f(&args_eval?);
                        },
                        FelispExp::Lambda(lambda) => {
                            let new_env = &mut env_for_lambda(lambda.params_exp, arg_forms, env)?;
                            eval(&lambda.body_exp, new_env)
                        },
                        _ => Err(
                            FelispErr::Reason("first form must be a function".to_string())
                        ),
                    }
                }
            }
        }
        FelispExp::Lambda(_) => Err(FelispErr::Reason("unexpected form in lambda".to_string())),
        FelispExp::Symbol(k) =>
            env_get(k, env)
            .ok_or(
                FelispErr::Reason(
                    format!("<< unexpected symbol k='{}'", k)
                )
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
    io::stdin()
        .read_line(&mut expr)
        .expect("Failed to read line");
    expr
}


/* Database layer */
#[derive(Debug)]
struct Row {
    id: i32,
    username: String,
    email: String
}

#[derive(Debug)]
struct Table {
    name: String,
    num_rows: i32,
    pages: i32,
    rows: Vec<Row>
}

// TODO(viksit): add a result with success etc to these functions

fn execute_insert(
    table: &mut Table,
    id: i32,
    username: String,
    email: String) {

    // insert data from a source data structure into a row
    let mut row = Row {
        id: id,
        email: email,
        username: username
    };
    table.rows.push(row);
    table.num_rows += 1;
}

fn execute_select(table: &mut Table) {
    println!("Table: <{}>", table.name);
    for row in &table.rows {
        println!("{:?}", row);
    }
}

fn test_insert1() {
    let mut rows: Vec<Row> = Vec::new(); // or Vec::new()
    let mut t = Table {
        name: String::from("mytable1"),
        num_rows: 0,
        pages: 0,
        rows: rows
    };
    execute_insert(
        &mut t,
        1,
        String::from("akriti"),
        String::from("akritigaur@gmail.com")
    );
    execute_insert(
        &mut t,
        10,
        String::from("viksit"),
        String::from("viksit@gmail.com")
    );
    // println!("Table: {:?}", t);
    execute_select(&mut t);
}



fn main() {
    test_insert1();
}

// fn main() {

//     let env = &mut default_env();
//     loop {
//         println!("risp >");
//         let expr = slurp_expr();
//         match parse_eval(expr, env) {
//             Ok(res) => println!("// 🔥 => {}", res),
//             Err(e) => match e {
//                 FelispErr::Reason(msg) => println!("// 🙀 => {}", msg),
//             },
//         }
//     }

// }

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
