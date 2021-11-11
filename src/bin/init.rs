use tomachidomain;

fn main() {
    let init = tomachidomain::function::init_actions(4);
    for (player,actions) in init.iter() {
        println!("p: {:?} act: {:?}",player,actions);
    }
}