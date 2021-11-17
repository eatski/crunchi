use std::vec;

use exprocess::core::ExprocessCore;
use tomachidomain::{self, AppCommand, AppCore, state::MissionType};

fn main() {
    let state  = exec::<AppCore>(AppCommand::Init(4,vec![(MissionType::Arrival,2),(MissionType::Others,2),(MissionType::Scrap,2)]));
    println!("{:?}",state);
}

fn exec<Core: ExprocessCore>(command: Core::Command) -> Core::State{
    let mut state = Core::init();
    let result = Core::resolve(&state, command);
    Core::reducer(&mut state, result);
    state
}