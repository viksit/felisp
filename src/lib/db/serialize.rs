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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_examples() {
        let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);
        let encoded: Vec<u8> = bincode::serialize(&world).unwrap();
        // 8 bytes for the length of the vector, 4 bytes per float.
        assert_eq!(encoded.len(), 8 + 4 * 4);
        println!("encoded: {:?}", encoded);
        let decoded: World = bincode::deserialize(&encoded[..]).unwrap();
        println!("decoded: {:?}", decoded);
        assert_eq!(world, decoded);
    }

    #[test]
    fn test_file_serialize() {
        let path = "/tmp/foo.bar";
        let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);
        let mut writer = BufWriter::new(File::create(path).unwrap());
        bincode::serialize_into(&mut writer, &world).unwrap();
    }

    #[test]
    fn test_file_deserialize() {
        let path = "/tmp/foo.bar";
        let mut reader = BufReader::new(File::open(path).unwrap());
        let decoded: World = bincode::deserialize_from(&mut reader).unwrap();
        println!("decoded {:?}", decoded);
    }

    #[test]
    #[ignore]
    fn test_cursor_seek() {
        // let path = "/tmp/foo.bar";
        // let any_offset: u64 = 0;
        // // let mut file = File::open(path).unwrap();

        // let mut contents = Vec::new();
        // // file.read_to_end(&mut contents);


        // let mut cursor = Cursor::new(&mut contents);
        // cursor.seek(SeekFrom::Start(any_offset)).unwrap();
        // println!("cursor position: {:?}", cursor.position());
        // println!("file contents: {:?}", contents);

        // Make a cursor into a file first
        let path = "/tmp/foo.bar";
        let offset: u64 = 0;
        // let mut cursor = Cursor::new();
        // let mut file = File::open(path).unwrap();

    }

    #[test]
    #[ignore]
    fn test_so() {
        use std::io::{Cursor, Read, Seek, SeekFrom, Write};

        // Create fake "file"
        let mut c = Cursor::new(Vec::new());

        // Write into the "file" and seek to the beginning
        c.write_all(&[1, 2, 3, 4, 5]).unwrap();
        c.seek(SeekFrom::Start(0)).unwrap();

        // Read the "file's" contents into a vector
        let mut out = Vec::new();
        c.read_to_end(&mut out).unwrap();

        println!("{:?}", out);
    }

    #[test]
    #[ignore]
    fn test_seek() {
        use std::str;

        let any_offset: u64 = 42;
        let mut file = File::open("/tmp/foo.txt").unwrap();
        let new_position = file.seek(SeekFrom::Start(any_offset)).unwrap();
        println!("1>>>> {:?}", new_position);

        let any_offset: u64 = 42;
        let mut file = File::open("/tmp/foo.txt").unwrap();
        let mut contents = Vec::new();

        file.read_to_end(&mut contents).unwrap();

        let mut cursor = Cursor::new(contents);
        cursor.seek(SeekFrom::Start(0)).unwrap();
        //println!("2>>> {:?}", cursor.position());
        //println!("file: {:?}", file);
        println!("ref: {:?}", str::from_utf8(cursor.get_ref()));
    }

    #[test]
    #[ignore]
    fn test_seek2() {
        use std::str;
        let mut file = File::open("/tmp/foo.txt").unwrap();
        let mut buf=[0u8;4];
        file.read(&mut buf).unwrap();
        println!("{:?}", str::from_utf8(&buf));
    }

    #[test]
    fn test_seek_binary() {
        let mut file = File::open("/tmp/foo.bar").unwrap();
        let mut buf=[0u8;24];
        file.read(&mut buf).unwrap();
        // println!("{:?}", &buf);
        let decoded: World = bincode::deserialize(&mut buf).unwrap();
        println!("decoded {:?}", decoded);
    }


}
