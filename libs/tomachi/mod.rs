pub mod model;
pub mod state;
pub mod function;
mod settings;
mod util;
use std::panic;

use exprocess::core::ExprocessCore;
use rand::thread_rng;
use settings::AppSetting;
use state::{MissionsNum, PlayingState};

#[derive(Debug)]
pub enum AppState {
    Blank,
    Playing(Playing)
}

#[derive(Debug)]
pub struct Playing {
    pub state:PlayingState,
    pub setting:AppSetting
}

pub enum AppCommand {
    Init(usize,MissionsNum)
}

pub enum AppResult {
    CreatePlaying(Playing)
}

pub struct AppCore;

impl ExprocessCore for AppCore {
    type State = AppState;

    type Command = AppCommand;

    type Result = AppResult;

    fn init() -> Self::State {
        AppState::Blank
    }

    fn resolve(prev: &Self::State, command: &Self::Command) -> Self::Result {
        match (prev,command) {
            (AppState::Blank, AppCommand::Init(players_num,missions_num)) => {
                let setting = AppSetting::new();
                AppResult::CreatePlaying(Playing {
                    state: PlayingState::new(*players_num,missions_num,&setting.actions, &mut thread_rng()),
                    setting,
                })
            },
            (AppState::Playing(_), AppCommand::Init(_,_)) => panic!(),
        }
    }

    fn reducer(prev: &mut Self::State, result: &Self::Result) {
        todo!()
    }
}