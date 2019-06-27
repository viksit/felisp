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


const PAGE_SIZE: u32 = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROWS_PER_PAGE: usize = 10; // arbitrary for now
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Table {
    pub name: String,
    pub num_rows: i32,
    pub pages: i32,
    pub rows: Vec<[Option<Row>; ROWS_PER_PAGE]>
}


// pub fn execute_insert(table: &mut Table, id: i32, username: String, email: String) {
//     // insert data from a source data structure into a row
//     let mut row = Row {
//         id: id,
//         email: email,
//         username: username,
//     };
//     //table.rows.push([Some(row)]);
//     //table.num_rows += 1;
// }


fn execute_insert(table: &mut Table, id: i32, username: String, email: String) {
    // which row are we on
    let row_num = table.num_rows;

    // find which page to add this row to
    let page_num: i32 = (row_num as i32 + 1) / ROWS_PER_PAGE as i32;

    if (page_num <= table.pages) {
        // this page exists so we're ok
        println!("this page {} exists already", page_num);
        table.num_rows+=1;
        let mut row = Row {
            id: id,
            email: email,
            username: username
        };
        let row_offset: usize = table.num_rows as usize % ROWS_PER_PAGE as usize;
        table.rows[page_num as usize][row_offset] = Some(row.clone());
        println!("table is {:?}", table);

    } else {
        // this page doesn't exist. so we
        println!("this page {} doesn't exist lets create", page_num);
        // push a array of 10 rows into the rows vector
        let mut xs: [Option<Row>; 10] = Default::default();
        // increment pages by 1
        table.rows.push(xs);
        table.pages+=1;
        println!("\ntable is now {:?}", table);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_execute_insert() {
        let mut xs: [Option<Row>; 10] = Default::default();
        let mut t = Table {
            name: String::from("mytable1"),
            num_rows: 0,
            pages: 0,
            rows: vec![xs],
        };
        for i in 0..22 {
            execute_insert(&mut t,
                           i+1,
                           String::from(format!("apple{}", i+1)),
                           String::from(format!("apple{}@orange{}", i+1, i+1)));
        }

    }
    #[test]
    fn basic_vec() {
        let mut row = Row {
            id: 10,
            email: String::from("email1"),
            username: String::from("user1")
        };

        let mut xs: [Option<Row>; 10] = Default::default();
        let mut t = Table {
            name: String::from("mytable1"),
            num_rows: 0,
            pages: 0,
            rows: vec![xs],
        };

        println!("t.rows: {:?}", t.rows[0]);

        for i in 0..10 {
            t.rows[0][i] = Some(row.clone());
            println!("i: {} row: {:?}", i, t.rows[0][i]);
        }

    }

    #[test]
    fn test_serialize_row() {
        // compact repr - take structure and serialize into bytes
        // rather than use the bytes system as in the sqlite tutorial
        // we'll use bincode to serialize each row object

    }

    #[test]
    fn test_deserialize_row_from_bytes() {
        // given bytes, deserialize via bincode
    }

    #[test]
    fn test_insert_row() {
    }

}
