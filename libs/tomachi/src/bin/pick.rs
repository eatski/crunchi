use tomachidomain;

fn main() {
    let actions = tomachidomain::function::pick_action(5);
    println!("act: {:?}",actions);
}