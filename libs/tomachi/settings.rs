use rand::Rng;

use crate::{model::*, util::pick};

pub type ActionAndNum = (Action,usize);

pub trait ClonableRng: rand::Rng + std::clone::Clone {}

#[derive(Debug)]
pub struct AppSetting {
    pub actions: ActionList,
}
#[derive(Debug)]
pub struct ActionList {
    actions: Vec<ActionAndNum>,
}

impl ActionList {
    pub fn new() -> Self {
        let nums  = [
            (OfferingNum::ON1,(8,3)),
            (OfferingNum::ON2,(8,3)),
            (OfferingNum::ON3,(6,2))
        ];
    
        let others = [
            (Action::Rest,3),
            (Action::Judgment,1),
            (Action::Prise(PriseKey::Bag,Offering(OfferingTarget::Either,OfferingNum::ON3)),2),
            (Action::Prise(PriseKey::Chest,Offering(OfferingTarget::Either,OfferingNum::ON2)),2),
            (Action::Prise(PriseKey::Key,Offering(OfferingTarget::Either,OfferingNum::ON1)),2),
            (Action::Twist,1),
            (Action::Meditation,2),
            (Action::Levy,1),
            (Action::Restraint,2),
            (Action::Reveal,1)
        ];
        
        let actions = nums
            .iter()
            .flat_map(|(num,(spec,eith))| {
                [
                    (OfferingTarget::Specific(Character::Adam),num.clone(),spec),
                    (OfferingTarget::Specific(Character::Eve),num.clone(),spec),
                    (OfferingTarget::Either,num.clone(),eith)
                ].map(|(offer,num,card_num)| (Action::Offer(Offering(offer,num)),*card_num))
            })
            .chain(others)
            .collect();
        ActionList {
            actions
        }
    }
    pub fn pick<R: Rng + Clone>(&self,num: usize,rng: &R) -> Vec<Action> {
        pick(&self.actions, num, rng, Clone::clone)
    }
}

impl AppSetting {
    pub fn new() -> Self {
        AppSetting {
            actions: ActionList::new(),
        }
    }
}