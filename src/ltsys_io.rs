use std::fs;
use std::io;
use std::vec;

use crate::ltsys::LeitnerSystem;

pub fn create_system() -> Result<(), String> {
    // ask file name
    let filename: String = ask_string("filename", "default.ltsys")?;

    // ask number of boxes
    let mut boxes_number = String::new();
    print!("Number of boxes (let empty for default 7) :\n");
    io::stdin()
        .read_line(&mut boxes_number)
        .or(Err("error when reading number of boxes".to_string()))?;

    // create new system
    let new_system = LeitnerSystem {
        cards: vec![],
        boxes_number: boxes_number.trim().parse().unwrap_or(7),
    };

    write_to_disk(&new_system, &filename)
}

pub fn open_ltsys(filename: &String) -> Result<LeitnerSystem, String> {
    let content =
        fs::read_to_string(filename).or(Err(format!("error opening file {}", filename)))?;

    let ltsys: Result<LeitnerSystem, _> = serde_json::from_str(&content);
    ltsys.or(Err("error while deserialization".to_string()))
}

pub fn ask_string(name: &str, default: &str) -> Result<String, String> {
    print!("Enter {name} (leave empty for default):\n");

    let mut string = String::new();
    io::stdin().read_line(&mut string).or(Err("error"))?;

    let trimed_string = string.trim();
    Ok((if trimed_string.is_empty() {
        default
    } else {
        trimed_string
    })
    .to_string())
}

pub fn write_to_disk(ltsys: &LeitnerSystem, filename: &String) -> Result<(), String> {
    fs::write(filename, serde_json::to_string(&ltsys).unwrap())
        .or(Err("error writing file".to_string()))
}
