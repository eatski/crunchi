
use std::{fs::{File}, io::{BufReader, Error, Write}, path::Path};

use exprocess::core::ExprocessCore;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{AppCommand, AppCore, AppResult, AppStateRoot};

#[derive(JsonSchema,Serialize,Deserialize)]
pub struct CommandIO {
    pub command: AppCommand
}

const PATH_TO_HISTORY : &str = "./work/history.json";
const PATH_TO_COMMAND : &str = "./work/command.json";

pub fn read_current_state() -> Result<AppStateRoot,Error>  {
    let mut state = AppCore::init();
    let history: Vec<AppResult> = if Path::new(PATH_TO_HISTORY).exists() {
        let file = File::open(PATH_TO_HISTORY)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)?
    }  else {
        Vec::new()
    };
    for result in history.iter() {
        AppCore::reducer(&mut state, result.clone());
    }
    Ok(state)
}

pub fn push_result(result: AppResult) -> Result<(),Error>{
    let mut history  = if Path::new(PATH_TO_HISTORY).exists() {
        let file = File::open(PATH_TO_HISTORY)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader)?
    }  else {
        Vec::new()
    };
    history.push(result);
    let json = serde_json::to_vec(&history)?;
    let mut file = File::create(PATH_TO_HISTORY)?;
    file.write_all(&json)?;
    file.flush()?;
    Ok(())
}

pub fn read_command() -> Result<CommandIO,Error> {
    let file = BufReader::new(File::open(PATH_TO_COMMAND)?);
    Ok(serde_json::from_reader(file)?)
}