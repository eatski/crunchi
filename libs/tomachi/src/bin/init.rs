use std::vec;

use exprocess::core::ExprocessCore;
use tomachidomain::{self, AppCommand, AppCore, state::MissionType};

fn main() {
    let state  = exec::<AppCore>(AppCommand::Init(
        6,
        vec![
            (MissionType::Arrival,3),
            (MissionType::Others,2),
            (MissionType::Scrap,3)
        
        ]
    ));
    match state {
        tomachidomain::AppState::Blank => todo!(),
        tomachidomain::AppState::Playing(playing) => {
            let players = playing.state.players;
            for (p,state) in players.iter() {
                println!("{:?}: {:?}",p,&state);
            }
            
        },
    }
}

fn exec<Core: ExprocessCore>(command: Core::Command) -> Core::State{
    let mut state = Core::init();
    let result = Core::resolve(&state, command);
    Core::reducer(&mut state, result);
    state
}