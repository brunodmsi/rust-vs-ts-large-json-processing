use std::fs::File;

fn main() -> std::io::Result<()> {
    let file = File::open("../giant-json.json")
        .expect("File should open");

    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("File should be proper JSON");

    println!("{}", json["is_giant"]);

    Ok(())
}
