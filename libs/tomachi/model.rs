use std::{collections::HashMap, fmt::Debug, iter::repeat};
pub enum PriseType {
    A,
    B,
}

#[derive(Clone)]
pub enum Action {
    None,
    Offer(Offering),
    Protect(Character),
    Discard,
}

impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "-"),
            Self::Offer(map) => {
                let text: String = map.iter().flat_map(|(chara,num)| match chara {
                    Character::Adam => repeat("●").take(num.usize()),
                    Character::Eve => repeat("○").take(num.usize()),
                }).collect();
                f.write_str(text.as_str())
            },
            Self::Protect(arg0) => f.debug_tuple("Protect").field(arg0).finish(),
            Self::Discard => write!(f, "Discard"),
        }
    }
}

#[derive(Clone,Debug)]
pub enum OfferingNum {
    ON0,
    ON1,
    ON2,
    ON3,
}

impl OfferingNum {
    pub fn usize(&self) -> usize {
        match self {
            OfferingNum::ON0 => 0,
            OfferingNum::ON1 => 1,
            OfferingNum::ON2 => 2,
            OfferingNum::ON3 => 3,
        }
    }
}

pub type Offering = HashMap<Character, OfferingNum>;

pub enum GoalType {
    Arrival(Character),
    Matching,
    Scrap(Character),
    Prise(PriseType),
    Seizure,
}

#[derive(PartialEq,Eq,Hash,Clone,Debug)]
pub enum Character {
    Adam,
    Eve,
}

pub enum Sin {
    Prise,
    FailedAction,
}

pub type Player = usize;
