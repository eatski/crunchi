use crate::model::*;
use std::collections::HashMap;

pub struct PlayingState {
    pub characters: HashMap<Character, CharacterState>,
}

pub enum CharacterState {
    Alive {
        sins: Vec<Sin>,
        prises: Vec<PriseType>,
    },
    Arraival {
        prises: Vec<PriseType>,
    },
    Scrap,
}

pub enum CharacterLastState {
    Arraival { prises: Vec<PriseType> },
    Scrap,
}
