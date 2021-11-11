use std::{collections::HashMap, vec};
use rand::{thread_rng, Rng};

use crate::model::*;

fn tpl_to_action(a:OfferingNum,e:OfferingNum) -> Action {
    let vec = vec![
        (Character::Adam, a),
        (Character::Eve, e)
    ];
    Action::Offer(vec.into_iter().collect())
}

fn action_list() -> Vec<(Action, usize)>{
    vec![
        (tpl_to_action(OfferingNum::ON1,OfferingNum::ON0),8),
        (tpl_to_action(OfferingNum::ON0,OfferingNum::ON1),8),
        (tpl_to_action(OfferingNum::ON2,OfferingNum::ON0),4),
        (tpl_to_action(OfferingNum::ON1,OfferingNum::ON1),4),
        (tpl_to_action(OfferingNum::ON0,OfferingNum::ON2),4),
        (tpl_to_action(OfferingNum::ON3,OfferingNum::ON0),2),
        (tpl_to_action(OfferingNum::ON2,OfferingNum::ON1),2),
        (tpl_to_action(OfferingNum::ON1,OfferingNum::ON2),2),
        (tpl_to_action(OfferingNum::ON0,OfferingNum::ON3),2),
        (Action::None,4),
        (Action::Discard,4),
    ]
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
        (player,pick(&list, 3,thread_rng(),Action::clone))
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
