use crate::settlements::AnnualSettlement;

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Result};

const SETTLEMENTS_FILE: &str = "settlements.json";

pub fn serialize_to_json_file(data: &HashMap<u32, AnnualSettlement>) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(SETTLEMENTS_FILE)?;

    serde_json::to_writer_pretty(file, data)?;
    Ok(())
}

pub fn deserialize_from_json_file() -> Result<HashMap<u32, AnnualSettlement>> {
    let mut file = match File::open(SETTLEMENTS_FILE) {
        Ok(file) => file,
        Err(_) => return Ok(HashMap::new()),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: HashMap<u32, AnnualSettlement> = serde_json::from_str(&contents)?;
    Ok(data)
}
