use crate::lib::data::Row;
use crate::lib::data::Table;

// TODO(viksit): add a result with success etc to these functions

// (use database)
// this should open a database and load it
// (insert mytable1 value value) should insert this data into this file


pub fn execute_insert(table: &mut Table, id: i32, username: String, email: String) {
    // insert data from a source data structure into a row
    let mut row = Row {
        id: id,
        email: email,
        username: username,
    };
    //table.rows.push([Some(row)]);
    //table.num_rows += 1;
}

pub fn execute_select(table: &mut Table) {
    println!("Table: <{}>", table.name);
    for row in &table.rows {
        println!("{:?}", row);
    }
}
