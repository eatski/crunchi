use tomachidomain;

fn main() {
    let actions = tomachidomain::function::pick_action(4);
    println!("act: {:?}",actions);
}