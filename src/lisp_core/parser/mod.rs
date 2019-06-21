use std::num::ParseFloatError;
use crate::lib::data::{FelispExp, FelispErr, FelispEnv, FelispLambda, Table, Row};

// Now, convert this AST expression into a felisp expression

pub fn parse(tokens: &[String]) -> Result<(FelispExp, &[String]), FelispErr> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(FelispErr::Reason("Could not get token".to_string()))?;
    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(FelispErr::Reason("unexpected `)`".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}

pub fn read_seq(tokens: &[String]) -> Result<(FelispExp, &[String]), FelispErr> {
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
pub fn parse_atom(token: &str) -> FelispExp {
    match token.as_ref() {
        "true" => FelispExp::Bool(true),
        "false" => FelispExp::Bool(false),
        _ => {
            let potential_float: Result<f64, ParseFloatError> = token.parse();
            match potential_float {
                Ok(v) => FelispExp::Number(v),
                Err(_) => FelispExp::Symbol(token.to_string().clone()),
            }
        }
    }
}

#[macro_export]
macro_rules! ensure_tonicity {
    ($check_fn:expr) => {{
        |args: &[FelispExp]| -> Result<FelispExp, FelispErr> {
            let floats = parse_list_of_floats(args)?;
            let first = floats.first().ok_or(FelispErr::Reason(
                "expected at least one number".to_string(),
            ))?;
            let rest = &floats[1..];
            fn f(prev: &f64, xs: &[f64]) -> bool {
                match xs.first() {
                    Some(x) => $check_fn(prev, x) && f(x, &xs[1..]),
                    None => true,
                }
            };
            Ok(FelispExp::Bool(f(first, rest)))
        }
    }};
}
