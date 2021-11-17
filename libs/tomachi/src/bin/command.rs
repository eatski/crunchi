

use exprocess::core::ExprocessCore;
use tomachidomain::{self, AppCore, io::{push_result, read_command, read_current_state}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = read_current_state()?;
    let command = read_command()?;
    let result = AppCore::resolve(&state, command.command);
    AppCore::reducer(&mut state, result.clone());
    push_result(result)?;
    match state.state {
        tomachidomain::AppState::SetUp => {
            println!("Setup");
        },
        tomachidomain::AppState::Playing(state) => {

            for (key,value) in state.players {
                println!("{:?}:{:?}",key,value);
            }
        },
    }
    Ok(())
}