use std::iter::repeat;

use rand::{Rng, prelude::SliceRandom};

pub fn pick<T,Rn: Rng + Clone,R,F: FnMut(&T) -> R>(list:&Vec<(T, usize)>,num:usize,rng: &Rn,finalize: F) -> Vec<R> {
    let list : Vec<&T> = list.iter().flat_map(|(item,num)| repeat(item).take(*num)).collect();
    (0..num).map(|_| *list.choose(&mut rng.clone()).unwrap()).map(finalize).collect()
}
