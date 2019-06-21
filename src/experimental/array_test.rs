fn main() {
    let array_main: [Vec<u8>; 3] = [vec![1], vec![2, 4], vec![]];
    print(array_main);
    println!("{:?}", array_main[0])
}

fn print(array: [Vec<u8>; 3]) {
    for e in array.iter() {
        println!("{:?}", e)
    }
}
