use std::{collections::HashMap};
use rand::{thread_rng, Rng};

use crate::model::*;

fn action_list() -> Vec<(Action, usize)>{

    let nums  = [
        (OfferingNum::ON1,(8,4)),
        (OfferingNum::ON2,(8,4)),
        (OfferingNum::ON3,(8,2))
    ];

    let others = [
        (Action::Draw,4),
        (Action::Clean,2),
        (Action::Prise(PriseKey::Bag,Offering(OfferingTarget::Either,OfferingNum::ON3)),2),
        (Action::Prise(PriseKey::Chest,Offering(OfferingTarget::Either,OfferingNum::ON2)),2),
        (Action::Prise(PriseKey::Key,Offering(OfferingTarget::Either,OfferingNum::ON1)),2),
    ];

    nums
        .iter()
        .flat_map(|(num,(spec,eith))| {
            [
                (OfferingTarget::Specific(Character::Adam),num.clone(),spec),
                (OfferingTarget::Specific(Character::Eve),num.clone(),spec),
                (OfferingTarget::Either,num.clone(),eith)
            ].map(|(offer,num,card_num)| (Action::Offer(Offering(offer,num)),*card_num))
        })
        .chain(others)
        .collect()

}

fn pick<T,Rn: Rng,R,F: Fn(&T) -> R>(list:&Vec<(T, usize)>,num:usize,mut rng: Rn,finalize: F) -> Vec<R> {
    let sum : usize = list.iter().map(|(_,num)| *num).sum();
    (0..num).map(|_| {
        let limit = rng.gen_range(0..sum);
        let option = list.iter().scan(0, |acc,(target,num)| {
            if *acc > limit {
                None
            } else {
                *acc = *acc + *num;
                Some(target)
            }
        }).last();
        option.unwrap()
    }).map(finalize).collect()
}

pub fn init_actions(players_num: usize) -> HashMap<Player,Vec<Action>> {
    let list = action_list();
    (0..players_num).map(|player| {
        (player,pick(&list, 4,thread_rng(),Action::clone))
    }).collect()
}

pub fn pick_action(num: usize) -> Vec<Action> {
    let list = action_list();
    pick(&list, num,thread_rng(),Action::clone)
}

#[test]
pub fn _test_pick() {
    let list = vec![
        ("a",1),
        ("b",2),
        ("c",3),
        ("d",0)
    ];
    let res = pick(
        &list, 6, 
        thread_rng(),
        |s|*s
    );
    assert!(!res.contains(&"d"));
}
