use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::{stdin, stdout, Write};
use std::num::ParseFloatError;
use std::process;
use std::rc::Rc;

use std::error::Error;
use std::path::Path;
use std::fs::File;

use std::io::BufWriter;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;
use std::io::SeekFrom;
use std::io::Seek;
use std::io::BufRead;

use serde::{Serialize, Deserialize};
use bincode; // serialize_into will be useful


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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Row {
    pub id: i32,
    pub username: String,
    pub email: String,
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct Table {
//     pub name: String,
//     pub num_rows: i32,
//     pub pages: i32,
//     pub rows: Vec<Row>,
// }


pub const PAGE_SIZE: u32 = 4096;
pub const TABLE_MAX_PAGES: usize = 100;
pub const ROWS_PER_PAGE: usize = 10; // arbitrary for now
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Table {
    pub name: String,
    pub num_rows: i32,
    pub pages: i32,
    pub rows: Vec<[Option<Row>; ROWS_PER_PAGE]>
}
