use std::io::prelude::*;
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

use crate::lib::data::*;
// TODO(viksit): add a result with success etc to these functions

// (use database)
// this should open a database and load it
// (insert mytable1 value value) should insert this data into this file

pub fn execute_select(table: &mut Table) {
    println!("Table: <{}, {} num_rows, {} num_pages>", table.name, table.num_rows, table.num_pages);
    for page in &table.pages {
        for row in page.iter() {
            match row {
                Some(row) => println!("{:?}", row),
                None => ()
            }
        }
    }
}

pub fn execute_insert(table: &mut Table, id: i32, username: String, email: String) {
    // which row are we on
    // find which page to add this row to
    let page_num: i32 = (table.num_rows  as i32) / (ROWS_PER_PAGE as i32);
    //println!(">> page_num: {}, num_pages: {}", page_num, table.num_pages);

    if (page_num > table.num_pages) {
        // this page doesn't exist. so we
        // println!("********** this page {} doesn't exist lets create", page_num);
        // push a array of 10 rows into the rows vector
        let mut xs: [Option<Row>; 10] = Default::default();
        // increment num_pages by 1
        table.pages.push(xs);
        table.num_pages+=1;
        //println!("\ntable is now {:?}", table);
    }

    // this page exists so we're ok
    //println!("this page {} exists already", page_num);
    let mut row = Row {
        id: id,
        email: email,
        username: username
    };
    let row_offset: usize = table.num_rows as usize % ROWS_PER_PAGE as usize;
    //println!("pagenum: {}, numrows {}, row_offset: {}", page_num, table.num_rows, row_offset);
    table.pages[page_num as usize][row_offset] = Some(row.clone());
    table.num_rows+=1;
}


fn create_dummy_table () -> Table {
    let mut xs: [Option<Row>; 10] = Default::default();
    let mut t = Table {
        name: String::from("mytable1"),
        num_rows: 0,
        num_pages: 0,
        pages: vec![xs],
    };
    for i in 0..21 {
        execute_insert(&mut t,
                       i,
                       String::from(format!("apple{}", i)),
                       String::from(format!("apple{}@orange{}", i, i)));
    }
    println!("dummy table: rows: {}, num_pages: {}", t.num_rows, t.num_pages);
    t
}

fn db_open(filename: String) {
    let mut file = File::open(filename).unwrap();
}

fn write_table_to_file(filename: String, table: &Table) {
    let path = Path::new(&filename);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // First write meta into the file
    let encoded_name = bincode::serialize(&table.name).unwrap();
    file.write_all(&encoded_name.len().to_ne_bytes()).expect("oops");
    println!("bytes for {} are {:?}", &table.name, &encoded_name.len().to_ne_bytes());
    file.write_all(&encoded_name).expect("couldn't write data");

    let encoded_num_rows = bincode::serialize(&table.num_rows).unwrap();
    file.write_all(&encoded_num_rows.len().to_ne_bytes()).expect("oops");
    file.write_all(&encoded_num_rows).expect("couldn't write data");

    let encoded_num_pages = bincode::serialize(&table.num_pages).unwrap();
    file.write_all(&encoded_num_pages.len().to_ne_bytes()).expect("oops");
    file.write_all(&encoded_num_pages).expect("couldn't write data");

    println!("encoded len: {}, {}, {}", encoded_name.len(), encoded_num_rows.len(), encoded_num_pages.len());

    // Since we plan to write to file only when we flush
    // lets write all pages together

    for i in 0..table.num_pages {
        println!("page {}", i);
        let tp = &table.pages[i as usize];
        println!("serializing {:?}", table.pages[i as usize]);
        let encoded_page = bincode::serialize(&tp).unwrap();
        println!("encode page size: {}", encoded_page.len());
        //file.write_all(&encoded_page).expect("couldn't write page");
    }

    /*
    In order for us to store data in this file, we need to figure out how big a page is
    - unless we specify a max limit for a page, we can't efficiently retrieve it since
    each array of row structs can have a different size.
    - So, we can do something interesting here.

    (1)
    - Lets specify a byte buffer of 4k
    - Copy whatever serialized version of the page looks like into this buffer
    - Make the first item in the buffer the total size of the actual contents and the rest is none?
    - when someone wants to read this, they first allocate a buffer of 4k
    - then they read buf[0] to figure how many bytes to read. If this is a i32
    - then seek to start + 4 bytes
    - and read that into a page buffer

    (2)
    - second approach is to keep size and bytes
    - dynamically create the buffer to read into it

     */




}
use std::fmt;

enum MyResult {
    I(i32),
    S(&'static str)
}
impl fmt::Display for MyResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            MyResult::I(a) => format!("{}", a),
            MyResult::S(a) => format!("{}", a)
        };
        write!(f, "{}", str)
    }
}

fn read_field_from_file<'a>(mut file: &File, typename: &'a str) -> MyResult {
    //println!("file2 : {:?}", file);
    let mut field_size_buf = [0u8;8];
    file.read(&mut field_size_buf).unwrap();
    let field_size = i64::from_ne_bytes(field_size_buf);
    println!("field size: {}", field_size);
    let mut field_buf = vec![0; field_size as usize];
    file.read(&mut field_buf).unwrap();
    println!("field buf: {:?}", field_buf);
    match typename {
        "String" =>  {
            let decoded_field:String  = bincode::deserialize(&mut field_buf).unwrap();
            let s: &str = Box::leak(decoded_field.into_boxed_str());
            MyResult::S(s)
        },
        "i32" =>  {
            let decoded_field:i32  = bincode::deserialize(&mut field_buf).unwrap();
            MyResult::I(decoded_field)
        },
        _ => {
            println!("Unsupported type");
            MyResult::S("")
        }

    }
    //println!("decoded field: {:?}", decoded_field);
}

fn read_table_from_file(filename: String) {
    let path = Path::new(&filename);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    println!(">>> file1: {:?}", file);
    let field1 = read_field_from_file(&file, "String");
    println!("field1: {}", field1);
    // file.seek(SeekFrom::Start()).unwrap();
    let field2 = read_field_from_file(&file, "i32");
    println!("field2: {}", field2);

    let field3 = read_field_from_file(&file, "i32");
    println!("field3: {}", field3);

    // let mut rowbuf = [0u8;8];
    // let mut pagebuf = [0u8;4];

    // let mut namesizebuf = [0u8;8];
    // file.read(&mut namesizebuf).unwrap();
    // let namesize = i64::from_ne_bytes(namesizebuf);
    // let mut namebuf = vec![0; namesize as usize];
    // file.read(&mut namebuf).unwrap();
    // let decodedname: String = bincode::deserialize(&mut namebuf).unwrap();
    // println!("decoded name: {:?}", decodedname);


    //     file.read(&mut namebuf).unwrap();
    //     let decodedname: String = bincode::deserialize(&mut namebuf).unwrap();
    //     println!("decoded name: {:?}", decodedname);
    //     file.seek(SeekFrom::Start(16)).unwrap();
    //     file.read(&mut rowbuf).unwrap();
    //     let decodedrow: i32 = bincode::deserialize(&mut rowbuf).unwrap();
    //     println!("decoded row: {:?}", decodedrow);
    //     file.seek(SeekFrom::Start(20)).unwrap();
    //     file.read(&mut pagebuf).unwrap();
    //     let decodedpage: i32 = bincode::deserialize(&mut pagebuf).unwrap();
//     println!("decoded page: {:?}", decodedpage);
}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_dummy_table() {
        let t = create_dummy_table();
        println!("{:?}", t);
    }

    #[test]
    fn test_db_open() {
        // db open means we find a file and in a struct, store the open file descriptor in our
        // struct
        // ill
    }

    #[test]
    fn test_write_table_to_file() {
        // for now, we will do a flush at the very end
        // where we read the file on open and store 1 page
        // then when we are done, we flush and rewrite the file page by page
        let mut t = create_dummy_table();
        write_table_to_file("/tmp/mytable1.bar".to_string(), &t);
    }

    #[test]
    fn test_read_table_from_file() {
        read_table_from_file("/tmp/mytable1.bar".to_string());
    }

    #[test]
    fn test_execute_insert() {
        let mut xs: [Option<Row>; 10] = Default::default();
        let mut t = Table {
            name: String::from("mytable1"),
            num_rows: 0,
            num_pages: 0,
            pages: vec![xs],
        };
        for i in 0..22 {
            execute_insert(&mut t,
                           i+1,
                           String::from(format!("apple{}", i+1)),
                           String::from(format!("apple{}@orange{}", i+1, i+1)));
        }

    }

    #[test]
    fn test_execute_select() {
        let mut t = create_dummy_table();
        execute_select(&mut t);
    }


    #[test]
    fn test_basic_paging() {
        let mut row = Row {
            id: 10,
            email: String::from("email1"),
            username: String::from("user1")
        };

        let mut xs: [Option<Row>; 10] = Default::default();
        let mut t = Table {
            name: String::from("mytable1"),
            num_rows: 0,
            num_pages: 0,
            pages: vec![xs],
        };
        println!("t.rows: {:?}", t.pages[0]);
        for i in 0..10 {
            t.pages[0][i] = Some(row.clone());
            println!("i: {} row: {:?}", i, t.pages[0][i]);
        }

    }
}
