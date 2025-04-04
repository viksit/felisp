use std::num::ParseFloatError;
use std::rc::Rc;

use crate::lib::data::*;

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



// Helper function that enforces all FelispExp's that we receive are floats
pub fn parse_list_of_floats(args: &[FelispExp]) -> Result<Vec<f64>, FelispErr> {
    args.iter().map(|x| parse_single_float(x)).collect() // no ; since return expression
}

pub fn parse_single_float(exp: &FelispExp) -> Result<f64, FelispErr> {
    match exp {
        FelispExp::Number(num) => Ok(*num),
        _ => Err(FelispErr::Reason("expected a number".to_string())),
    }
}

pub fn parse_list_of_symbol_strings(form: Rc<FelispExp>) -> Result<Vec<String>, FelispErr> {
    let list = match form.as_ref() {
        FelispExp::List(s) => Ok(s.clone()),
        _ => Err(FelispErr::Reason(
            "expected args form to be a list".to_string(),
        )),
    }?;
    list.iter()
        .map(|x| match x {
            FelispExp::Symbol(s) => Ok(s.clone()),
            _ => Err(FelispErr::Reason(
                "expected symbols in the argument list".to_string(),
            )),
        })
        .collect()
}
