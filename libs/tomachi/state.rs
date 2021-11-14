use crate::model::*;
use std::collections::HashMap;

pub struct PlayingState {
    pub characters: HashMap<Character, CharacterState>,
}

pub enum CharacterState {
    Alive {
        sins: Vec<Sin>,
        prise: bool 
    },
    Arraival {
        prises: bool,
    },
    Scrap,
}

pub enum CharacterLastState {
    Arraival { prises: bool },
    Scrap,
}
