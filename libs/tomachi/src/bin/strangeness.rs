use rand::thread_rng;
use tomachidomain;

fn main() {
    let strs = tomachidomain::model::Strangeness::check(12, thread_rng());
   println!("strs: {:?}",strs);
}