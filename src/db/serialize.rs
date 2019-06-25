use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_1() {

//         let world = World(vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]);
//         let encoded: Vec<u8> = bincode::serialize(&world).unwrap();

//         // 8 bytes for the length of the vector, 4 bytes per float.
//         assert_eq!(encoded.len(), 8 + 4 * 4);
//         println!("encoded: {:?}", encoded);
//         let decoded: World = bincode::deserialize(&encoded[..]).unwrap();
//         println!("decoded: {:?}", decoded);
//         assert_eq!(world, decoded);

//     }

// }
