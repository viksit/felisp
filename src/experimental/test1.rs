extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Deserialize, Serialize)]
struct A(String, u8);

fn example() -> Result<(), serde_json::Error> {
    let input = A(String::from("Einstein"), 80);

    let json = serde_json::to_string(&input)?;

    println!("JSON: {}", json);

    let back: A = serde_json::from_str(&json)?;

    println!("Back to A: {:?}", back);

    Ok(())
}

fn main() {
    example().unwrap();
}
