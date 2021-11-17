use rand::{Rng, prelude::SliceRandom};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{model::*, settings::ActionList};
use std::{collections::{HashMap}};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PlayingState {
    pub characters: HashMap<Character, CharacterState>,
    pub players: HashMap<Player,PlayerState>
}

#[derive(Debug,JsonSchema,Serialize,Deserialize)]
pub enum MissionType {
    Arrival,Scrap,Others
}

impl MissionType {
    pub fn missions(&self) -> Vec<Mission> {
        match self {
            MissionType::Arrival => Character::ALL.map(Mission::Arrival).into(),
            MissionType::Others => [Mission::Matching,Mission::Prise].into(),
            MissionType::Scrap => Character::ALL.map(Mission::Scrap).into(),
        }
    }
}

pub type MissionsNum = Vec<(MissionType,usize)>;

impl PlayingState {
    pub fn new<R : Rng + Clone>(
        player_num: usize,
        mussions_num: &MissionsNum,
        list: &ActionList,
        rng: &mut R
    ) -> Self{
        let missions = Self::assign_missions(
            player_num,
            mussions_num,
            rng
        );
        Self {
            characters: Character::ALL.map(|chara| (chara,CharacterState::new())).into(),
            players: (0..player_num)
                .map(|player| (player,PlayerState::new(missions.get(player).unwrap().clone(), list, rng) ))
                .collect()
        }
    }

    fn assign_missions<R : Rng + Clone>(player_num: usize,missions_num: &MissionsNum,rng: &mut R) -> Vec<Mission> {
        let mut candidate : Vec<_> = missions_num
            .iter()
            .flat_map(|(mission,num)| Self::a(mission.missions(),*num,rng))
            .collect();

        candidate.shuffle(rng);
        let (taken,_) = candidate.split_at(player_num);
        taken.into()
    }

    fn a<T: Clone,R: Rng>(mut list: Vec<T>,num: usize,mut rng: &mut R) -> Vec<T>{
        list.shuffle(&mut rng);
        list.into_iter().cycle().take(num).collect()
    }
}



#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum CharacterState {
    Alive {
        sins: Vec<Sin>,
        position: usize,
        prise: Prise 
    },
    Decisions(CharacterDecisions)
}

impl CharacterState {
    pub fn new() -> Self{
        CharacterState::Alive {
            sins: Vec::new(),
            position: 0,
            prise: Prise::new()
        }
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum CharacterDecisions {
    Arraival { prises: bool },
    Scrap,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct PlayerState {
    pub mission: Mission,
    pub contribution: Contribution,
    pub cards: Vec<Action>
}

impl PlayerState {
    pub fn new<R : Rng + Clone>(mission: Mission, list: &ActionList,rng: &R) -> PlayerState {
        Self {
            mission,
            contribution: 0,
            cards: list.pick(4, rng)
        }
    }
}