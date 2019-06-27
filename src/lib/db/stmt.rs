use crate::lib::data::*;
// TODO(viksit): add a result with success etc to these functions

// (use database)
// this should open a database and load it
// (insert mytable1 value value) should insert this data into this file


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

pub fn execute_select(table: &mut Table) {
    println!("Table: <{}, {} rows, {} pages>", table.name, table.num_rows, table.pages);
    for row in &table.rows {
        for subrow in row.iter() {
            match subrow {
                Some(subrow) => println!("{:?}", subrow),
                None => ()
            }
        }
    }
}

pub fn execute_insert(table: &mut Table, id: i32, username: String, email: String) {
    // which row are we on
    // find which page to add this row to
    let page_num: i32 = (table.num_rows  as i32) / (ROWS_PER_PAGE as i32);
    //println!(">> page_num: {}, num_pages: {}", page_num, table.pages);

    if (page_num > table.pages) {
        // this page doesn't exist. so we
        // println!("********** this page {} doesn't exist lets create", page_num);
        // push a array of 10 rows into the rows vector
        let mut xs: [Option<Row>; 10] = Default::default();
        // increment pages by 1
        table.rows.push(xs);
        table.pages+=1;
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
    table.rows[page_num as usize][row_offset] = Some(row.clone());
    table.num_rows+=1;
}

fn db_open(filename: String) {
    use std::fs::File;
    let mut file = File::open(filename).unwrap();
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_db_open() {
        println!("sfds");
        test_execute_select();
    }

    #[test]
    fn test_write_to_file() {
        // ?
        // for now, we will do a flush at the very end
        // where we read the file on open and store 1 page
        // then when we are done, we flush and rewrite the file page by page
        //


    }


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
    fn test_execute_select() {
        let mut xs: [Option<Row>; 10] = Default::default();
        let mut t = Table {
            name: String::from("mytable1"),
            num_rows: 0,
            pages: 0,
            rows: vec![xs],
        };
        for i in 0..25 {
            execute_insert(&mut t,
                           i,
                           String::from(format!("apple{}", i)),
                           String::from(format!("apple{}@orange{}", i, i)));
        }

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
