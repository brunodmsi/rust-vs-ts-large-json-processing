#![allow(non_snake_case)]

use std::fs::File;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct DataResult {
    id: String,
    isActive: bool,
    registered: String,
    tags: Vec<String> 
}

fn main() -> std::io::Result<()> {
    let file = File::open("../giant-json.json")
        .expect("File should open");

    let data_result: Vec<DataResult> = serde_json::from_reader(file)
        .expect("File should be proper JSON");

    let mut ids = Vec::<&String>::new();

    for result in data_result.iter() {
        if !result.isActive {
            continue;
        }

        ids.push(&result.id);
    }

    println!("{:?}", ids);

    Ok(())
}
