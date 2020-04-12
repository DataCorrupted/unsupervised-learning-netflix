use std::error::Error;
use std::process;
use std::fs::File;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Transaction {
    movie_id: u64,
    customer_id: u64,
    rating: u8,
    date: String,
}

fn example() -> Result<(), Box<dyn Error>> {
    let file = File::open("./data/train.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let Transaction: Transaction = result?;
        println!("{:?}", Transaction);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}