use std::{collections::HashMap};
use rand::{thread_rng};

use crate::{model::*, settings::{ActionList}};

pub fn init_actions(players_num: usize) -> HashMap<Player,Vec<Action>> {
    let list = ActionList::new();
    (0..players_num).map(|player| {
        (player,list.pick( 4,&thread_rng()))
    }).collect()
}

pub fn pick_action(num: usize) -> Vec<Action> {
    let list = ActionList::new();
    list.pick(num, &thread_rng())
}
