use std::error::Error;
use std::path::Path;
use std::fs::File;

use std::io::BufWriter;
use std::io::BufReader;

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
    fn test_1() {
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
}
