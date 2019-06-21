use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::{stdin, stdout, Write};
use std::num::ParseFloatError;
use std::process;
use std::rc::Rc;

#[derive(Clone)]
pub enum FelispExp {
    Bool(bool),
    Symbol(String),
    Number(f64),
    List(Vec<FelispExp>),
    Func(fn(&[FelispExp]) -> Result<FelispExp, FelispErr>), // function evaluations
    Lambda(FelispLambda),
    Table(Table),
}

#[derive(Debug)]
pub enum FelispErr {
    Reason(String),

}
#[derive(Clone)]
pub struct FelispEnv<'a> {
    pub data: HashMap<String, FelispExp>,
    pub outer: Option<&'a FelispEnv<'a>>,
}

#[derive(Clone)]
pub struct FelispLambda {
    pub params_exp: Rc<FelispExp>,
    pub body_exp: Rc<FelispExp>,
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
            FelispExp::Table(a) => {
                format!("Table: Name: {} Rows: {}", a.name.to_string(), a.num_rows)
            }
        };

        write!(f, "{}", str)
    }
}

/* Database layer */
#[derive(Debug, Clone)]
pub struct Row {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub num_rows: i32,
    pub pages: i32,
    pub rows: Vec<Row>,
}
