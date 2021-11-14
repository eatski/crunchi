use std::{fmt::Debug, iter::repeat};
pub enum Card {
    Action(Action),
}

#[derive(Clone,Debug)]
pub enum PriseKey {
    Key,Bag,Chest
}

#[derive(Clone,Debug)]
pub enum Action {
    Draw,
    Offer(Offering),
    Prise(PriseKey,Offering),
    Clean,
    Robbery,
    Contrarian
}

// pub enum Insanity {
//     /// 沈黙
//     Silence,
//     /// 金縛り
//     Paralysis,
//     /// どっかから手
//     Hand,
//     /// 同調圧力
//     PeerPressure,
//     /// 神隠し
//     SpiritedAway,
// }

#[derive(Clone,Debug)]
pub enum OfferingNum {
    ON1,ON2,ON3,
}

impl OfferingNum {
    pub fn usize(&self) -> usize {
        match self {
            OfferingNum::ON1 => 1,
            OfferingNum::ON2 => 2,
            OfferingNum::ON3 => 3,
        }
    }
}

#[derive(Clone)]
pub struct Offering(pub OfferingTarget,pub OfferingNum);

impl Debug for Offering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mark = match &self.0 {
            OfferingTarget::Specific(chara) => chara.debug_str(),
            OfferingTarget::Either => "◎",
        };
        let str : String = repeat(mark).take(self.1.usize()).collect();
        f.write_str(str.as_str())
    }
}

#[derive(Clone,Debug)]
pub enum OfferingTarget {
    Specific(Character),
    Either
}

pub enum GoalType {
    Arrival(Character),
    Matching,
    Scrap(Character),
    Prise,
}

#[derive(PartialEq,Eq,Hash,Clone,Debug)]
pub enum Character {
    Adam,
    Eve,
}


impl Character {
    pub fn debug_str(&self) -> &str {
        match self {
            Character::Adam => "●",
            Character::Eve => "○",
        }
    }
}

pub enum Sin {
    Prise(PriseKey),
    FailedOffering(OfferingNum),
    Turn
}

impl Sin {
    pub fn count(&self) -> usize {
        match self {
            Sin::Prise(_) => 2,
            Sin::FailedOffering(num) => num.usize(),
            Sin::Turn => 1,
        }
    }
}

pub type Player = usize;
