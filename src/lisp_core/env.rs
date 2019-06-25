use std::collections::HashMap;

use crate::lib::data::*;
use crate::lisp_core::parser::*;

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

// Set up the environment to control functions and operators and data
pub fn default_env<'a>() -> FelispEnv<'a> {
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
        FelispExp::Func(ensure_tonicity!(|a, b| a == b)),
    );
    data.insert(
        ">".to_string(),
        FelispExp::Func(ensure_tonicity!(|a, b| a > b)),
    );
    data.insert(
        ">=".to_string(),
        FelispExp::Func(ensure_tonicity!(|a, b| a >= b)),
    );
    data.insert(
        "<".to_string(),
        FelispExp::Func(ensure_tonicity!(|a, b| a < b)),
    );
    data.insert(
        "<=".to_string(),
        FelispExp::Func(ensure_tonicity!(|a, b| a <= b)),
    );

    // Database layer
    let mut rows: Vec<Row> = Vec::new(); // or Vec::new()
    let mut t = Table {
        name: String::from("mytable1"),
        num_rows: 0,
        pages: 0,
        rows: rows,
    };

    data.insert("mytable1".to_string(), FelispExp::Table(t));

    FelispEnv { data, outer: None } // Return expression
}


#[test]
fn test_env_cccc() {
    assert_eq!(2, 2);
    println!("yay test11 ++++++++++");
}
