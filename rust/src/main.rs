#![allow(non_snake_case)]

use std::collections::HashMap;
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

    let mut tag_users: HashMap<&String, Vec<&String>> = HashMap::new();

    for user in data_result.iter() {
        for tag in user.tags.iter() {
            tag_users.entry(tag).or_insert(Vec::new()).push(&user.id);
        }
    }

    //println!("{:?}", tag_users);
    println!("total users -> {}", data_result.len());
    println!("total tags -> {}", tag_users.keys().len());

    println!("total users by tags");
    for tag in tag_users.keys() {
        println!("tag {} -> {} users", tag, tag_users[tag].len());
    }

    Ok(())
}
