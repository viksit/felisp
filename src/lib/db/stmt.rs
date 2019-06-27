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
    for i in 0..22 {
        execute_insert(&mut t,
                       i+1,
                       String::from(format!("apple{}", i+1)),
                       String::from(format!("apple{}@orange{}", i+1, i+1)));
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
    file.write_all(&encoded_name).expect("couldn't write data");

    let encoded_num_rows = bincode::serialize(&table.num_rows).unwrap();
    file.write_all(&encoded_num_rows).expect("couldn't write data");

    let encoded_num_pages = bincode::serialize(&table.num_pages).unwrap();
    file.write_all(&encoded_num_pages).expect("couldn't write data");

    println!("encoded len: {}, {}, {}", encoded_name.len(), encoded_num_rows.len(), encoded_num_pages.len());

    // Since we plan to write to file only when we flush
    // lets write all pages together

    for i in 0..table.num_pages {

    }



}

fn read_table_from_file(filename: String) {
    let path = Path::new(&filename);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut namebuf = [0u8;16];
    let mut rowbuf = [0u8;4];
    let mut pagebuf = [0u8;4];
    file.read(&mut namebuf).unwrap();
    let decodedname: String = bincode::deserialize(&mut namebuf).unwrap();
    println!("decoded name: {:?}", decodedname);
    file.seek(SeekFrom::Start(16)).unwrap();
    file.read(&mut rowbuf).unwrap();
    let decodedrow: i32 = bincode::deserialize(&mut rowbuf).unwrap();
    println!("decoded row: {:?}", decodedrow);
    file.seek(SeekFrom::Start(20)).unwrap();
    file.read(&mut pagebuf).unwrap();
    let decodedpage: i32 = bincode::deserialize(&mut pagebuf).unwrap();
    println!("decoded page: {:?}", decodedpage);
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
            num_pages: 0,
            pages: vec![xs],
        };

        println!("t.rows: {:?}", t.pages[0]);

        for i in 0..10 {
            t.pages[0][i] = Some(row.clone());
            println!("i: {} row: {:?}", i, t.pages[0][i]);
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
