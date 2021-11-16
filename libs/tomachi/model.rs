use std::{collections::{HashSet}, fmt::Debug, iter::repeat};

use rand::{Rng, prelude::{SliceRandom}};
pub enum Card {
    Action(Action),
}

#[derive(Debug)]
pub struct Prise {
    pub keys: HashSet<PriseKey>
}

impl Prise {
    pub fn new() -> Self {
        Self {
            keys: HashSet::new()
        }
    }
}

#[derive(Clone,Debug)]
pub enum PriseKey {
    Key,Bag,Chest
}

#[derive(Clone,Debug)]
pub enum Action {
    Rest,
    Offer(Offering),
    Prise(PriseKey,Offering),
    Judgment,
    Twist,
    Levy,
    Meditation,
    Restraint,
    Reveal
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Strangeness {
    /// 沈黙
    Silence,
    /// 金縛り
    Paralysis,
    /// 同調圧力
    PeerPressure,
    /// 物忘れ
    Forgetful,
    /// 入れ替わり
    Swap,
    /// 神隠し
    SpiritedAway,
}

impl Strangeness {
    fn incidence<R: Rng>(&self,mut rng: R) -> bool {
        rng.gen_bool(1.0 / 3.0)
    }
    fn cost(&self) -> usize {
        match self {
            Strangeness::Silence => 5,
            Strangeness::Paralysis => 5,
            Strangeness::Forgetful => 11,
            Strangeness::SpiritedAway => 11,
            Strangeness::Swap => 16,
            Strangeness::PeerPressure => 16,
        }
    }
    //FIXME
    pub fn check<R: Rng + Clone>(insanity: usize,mut rng: R) -> Self {
        let list = [
            Strangeness::Silence,
            Strangeness::Paralysis,
            Strangeness::PeerPressure,
            Strangeness::Forgetful,
            Strangeness::Swap,
            Strangeness::SpiritedAway
        ];

        list.choose(&mut rng).unwrap().clone()
    }
}

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

#[derive(Clone,Debug)]
pub enum Mission {
    Arrival(Character),
    Matching,
    Scrap(Character),
    Prise,
}

pub type Contribution = i32;

#[derive(PartialEq,Eq,Hash,Clone,Debug)]
pub enum Character {
    Adam,
    Eve,
}

impl Character {
    pub const ALL: [Self; 2] = [Character::Adam,Character::Eve];
}

impl Character {
    pub fn debug_str(&self) -> &str {
        match self {
            Character::Adam => "●",
            Character::Eve => "○",
        }
    }
}

#[derive(Debug)]
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
