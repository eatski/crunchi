use tomachidomain;

fn main() {
    let actions = tomachidomain::function::pick_action(1);
    println!("act: {:?}",actions);
}