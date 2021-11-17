use std::{fs::File, io::{Error, Write}};
use tomachidomain::{io::CommandIO};

fn main() -> Result<(),Error>{
    let schema = schemars::schema_for!(CommandIO);
    let json = serde_json::to_string_pretty(&schema).unwrap();
    let mut file = File::create("./work/schema.json")?;
    file.write_all(json.as_bytes())?;
    file.flush()?;
    Ok(())
}