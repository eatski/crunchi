pub mod model;
pub mod state;
pub mod function;
pub mod io;
mod settings;
mod util;
use std::panic;

use exprocess::core::ExprocessCore;
use rand::thread_rng;
use settings::AppSetting;
use state::{MissionsNum, PlayingState};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum AppState {
    SetUp,
    Playing(PlayingState)
}

pub struct AppStateRoot {
    pub state: AppState,
    setting: AppSetting
}

#[derive(JsonSchema,Serialize,Deserialize)]
pub enum AppCommand {
    Init(usize,MissionsNum)
}

#[derive(Clone,Serialize,Deserialize)]
pub enum AppResult {
    CreatePlaying(PlayingState)
}

pub struct AppCore;

impl ExprocessCore for AppCore {
    type State = AppStateRoot;

    type Command = AppCommand;

    type Result = AppResult;

    fn init() -> Self::State {
        AppStateRoot {
            setting: AppSetting::new(),
            state: AppState::SetUp
        }
    }

    fn resolve(prev: &Self::State, command: Self::Command) -> Self::Result {
        match (&prev.state,command) {
            (AppState::SetUp, AppCommand::Init(players_num,missions_num)) => {
                AppResult::CreatePlaying(
                    PlayingState::new(players_num,&missions_num,&prev.setting.actions, &mut thread_rng())
                )
            },
            (AppState::Playing(_), AppCommand::Init(_,_)) => panic!(),
        }
    }

    fn reducer(prev: &mut Self::State, result: Self::Result) {
        prev.state = match (&prev.state,result) {
            (AppState::SetUp, AppResult::CreatePlaying(playing)) => {
                 AppState::Playing(playing)
            },
            (AppState::Playing(_), AppResult::CreatePlaying(_)) => panic!(),
        };
    }
}